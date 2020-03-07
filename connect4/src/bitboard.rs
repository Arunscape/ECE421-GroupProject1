use super::game::connect4::ConnectColor;
use super::game::toto::TotoType;
use super::game::GameType::*;
use super::game::*;

pub struct BitBoard64 {
    pub width: usize,
    pub height: usize,
    mask: u64,
    position: u64,
    turns: usize,
}

impl BitBoard64 {
    pub fn from_game(game: &Game) -> Self {
        let key = pack_board_64(game);
        let (mask, pos) = decode_key_64(key, game.get_board().width, game.get_board().height);
        Self {
            width: game.get_board().width,
            height: game.get_board().height,
            mask: mask,
            position: pos,
            turns: 0,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            mask: 0,
            position: 0,
            turns: 0,
        }
    }

    pub fn flip_x(&mut self) -> &Self {
        self
    }

    pub fn flip_color(&mut self) -> &Self {
        self.position = self.mask & !self.position;
        self
    }

    pub fn key(&self) -> u64 {
        self.position + self.mask
    }
}

// The board packing takes a game object and stores it in as few bits
// as possible. Currently, this only works for 2 tiletype games.
//
// This board will pack to this
// .RY.
// .YRR
// YRYR
// RYRY
//
// ((height: 4) + 1) * (width: 4) = 20 bits
// first 5 bits are the first column, RY... -> 10100
// R is 1, Y is 0, and first empty bit is 1, rest of empty bits are 0
// col1  col2  col3  col4
// 10100_01011_10101_01110

fn pack_board(game: &Game) -> u32 {
    pack_board_n(game, 32) as u32
}

fn pack_board_64(game: &Game) -> u64 {
    pack_board_n(game, 64) as u64
}

fn pack_board_128(game: &Game) -> u128 {
    pack_board_n(game, 128)
}

fn pack_board_n(game: &Game, n: usize) -> u128 {
    assert!((1 + game.get_board().height) * game.get_board().width < n);
    let lay = game.get_board_layout();
    let pack_column = |col| lay.iter().skip(col).step_by(game.get_board().width);
    let mut res = 0;
    let bit_width = 1 + game.get_board().height;
    for col in 0..game.get_board().width {
        let col = pack_column(col);
        let mut bit_col = 0;
        let mut first_none = true;
        for val in col {
            // RY... -> 10100
            bit_col |= match val {
                Some(ChipDescrip::Connect(ConnectColor::Red)) => 1,
                Some(ChipDescrip::Connect(ConnectColor::Yellow)) => 0,
                Some(ChipDescrip::Toto(TotoType::T)) => 1,
                Some(ChipDescrip::Toto(TotoType::O)) => 0,
                None => {
                    if first_none {
                        first_none = false;
                        1
                    } else {
                        0
                    }
                }
            };
            bit_col <<= 1;
        }
        bit_col |= if first_none { 1 } else { 0 };

        res |= bit_col;
        res <<= bit_width;
    }
    res >> bit_width
}

// cannot reproduce order of moves
fn unpack_board(data: u128, width: usize, height: usize) -> Game {
    let mask_width = height + 1;
    let col_mask = (0x1 << mask_width) - 1;

    let board = crate::game::Board::new(width, height);
    let mut game = crate::game::Game::new(board, Connect4);

    fn bit_to_chip(bit: usize, typ: GameType) -> ChipDescrip {
        match (bit, typ) {
            (1, Connect4) => ChipDescrip::Connect(ConnectColor::Red),
            (0, Connect4) => ChipDescrip::Connect(ConnectColor::Yellow),
            (1, Toto) => ChipDescrip::Toto(TotoType::T),
            (0, Toto) => ChipDescrip::Toto(TotoType::O),
            _ => panic!("invalid"),
        }
    }

    let mut unpack_col = |col, x| {
        let mut data = col;
        let mut stuff = Vec::with_capacity(height);
        let mut first = true;

        // println!("{:#b} -> {}", col, x);
        let mut i = 0;
        while i < mask_width {
            i += 1;
            let x = (data & 0x1 as u128) as usize;
            data >>= 1;
            if x == 0 && first {
            } else if x == 1 && first {
                first = false;
            } else {
                stuff.push(bit_to_chip(x, Connect4))
            }
        }

        for chip in stuff.iter().rev() {
            game.play(x, *chip);
        }
    };

    for x in 0..width {
        unpack_col((data >> ((width - x - 1) * mask_width)) & col_mask, x)
    }

    game
}

// modifies in place and returns a reference
fn flip_color(game: &mut Game) -> &mut Game {
    let board = game.get_board_mut();
    board.chips.iter_mut().for_each(|chip| chip.flip());
    game
}

// modifies in place and returns a reference
fn flip_x(game: &mut Game) -> &mut Game {
    let board = game.get_board_mut();
    let bw = board.width;
    board
        .chips
        .iter_mut()
        .for_each(|chip| chip.set_x(bw - (chip.get_x() + 1)));
    game
}

pub fn check_win_4(width: usize, height: usize, game: u64, pos: usize) -> bool {
    false
}

fn decode_key_128(key: u128, width: usize, height: usize) -> (u128, u128) {
    let mask_width = height + 1;
    let col_mask = (0x1 << mask_width) - 1;

    let mut mask = 0;
    let mut pos = 0;
    for loc in 0..width {
        let col_bs = (width - loc - 1) * mask_width;
        let mut col = (key >> col_bs) & col_mask;
        let mut i = 0;
        let mut first = true;

        while i < mask_width {
            i += 1;
            let x = (col & 0x1 as u128) as usize;
            col >>= 1;
            if x == 0 && first {
            } else if x == 1 && first {
                first = false;
            } else {
                pos |= x << (col_bs + i);
                mask |= 1 << (col_bs + i);
            }
        }
    }
    (mask as u128, pos as u128)
}
fn decode_key_64(key: u64, width: usize, height: usize) -> (u64, u64) {
    let (m, p) = decode_key_128(key as u128, width, height);
    (m as u64, p as u64)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_chip_descript_from_int(i: usize) -> ChipDescrip {
        if i % 2 == 0 {
            ChipDescrip::Connect(ConnectColor::Red)
        } else {
            ChipDescrip::Connect(ConnectColor::Yellow)
        }
    }

    fn make_game(width: usize, height: usize, moves: Vec<usize>) -> Game {
        let board = crate::game::Board::new(width, height);
        let mut game = crate::game::Game::new(board, Connect4);
        let mut i = 0;
        for m in moves {
            game.play(m, get_chip_descript_from_int(i));
            i += 1;
        }

        game
    }

    #[test]
    fn test_pack_4x4() {
        // .RY.
        // .YRR
        // YRYR
        // RYRY
        // ====
        // 10100_01011_10101_01110

        let game = make_game(4, 4, vec![0, 1, 2, 3, 3, 2, 1, 0, 2, 2, 3, 1, 1, 2]);
        let res = 0b10100_01011_10101_01110;
        let packed = pack_board(&game);
        println!("{:#b}\n{:#b}", res, packed);
        assert_eq!(res, packed)
    }

    #[test]
    fn test_pack_7x6() {
        let game = make_game(
            7,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        let res = 0b1110000_1010000_0100100_0101000_1011000_0101100_1010000;
        let packed = pack_board_128(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_unpack_7x6() {
        use crate::io::GameIO;
        let game = make_game(
            7,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        let res = 0b1110000_1010000_0100100_0101000_1011000_0101100_1010000;
        let packed = pack_board_128(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);

        let game2 = unpack_board(packed, 7, 6);

        println!("Want:");
        crate::io::TermIO::draw_board(game.get_board());
        println!("Got:");
        crate::io::TermIO::draw_board(game2.get_board());

        let packed = pack_board_128(&game2);
        println!("{:#051b}", packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_flip_x() {
        use crate::io::GameIO;
        let width = 7;
        let mut game = make_game(
            width,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );
        let game2 = make_game(
            width,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        println!("OG:");
        crate::io::TermIO::draw_board(game.get_board());

        flip_x(&mut game);
        println!("Flipped X:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_x() == width - 1 - r.get_x()));

        flip_x(&mut game);
        println!("Double Flipped X:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_x() == r.get_x()));
    }

    #[test]
    fn test_flip_color() {
        use crate::io::GameIO;
        let width = 7;
        let mut game = make_game(
            width,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        let game2 = make_game(
            width,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        println!("OG:");
        crate::io::TermIO::draw_board(game.get_board());

        flip_color(&mut game);
        println!("Flipped Color:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_descrip() != r.get_descrip()));

        flip_color(&mut game);
        println!("Double Flipped Color:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_descrip() == r.get_descrip()));
    }

    #[test]
    fn test_check_win_4() {
        use crate::io::GameIO;
        let width = 7;
        let mut game = make_game(
            width,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5],
        );
        let game2 = make_game(
            width,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5],
        );
        game.play(3, get_chip_descript_from_int(1));

        crate::io::TermIO::draw_board(game.get_board());
        assert!(check_win_4(width, 6, pack_board_64(&game), 23));
        crate::io::TermIO::draw_board(game2.get_board());
        assert!(!check_win_4(width, 6, pack_board_64(&game2), 23));
    }

    #[test]
    fn test_bit_board_key_and_create() {}
}
