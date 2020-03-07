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

// Assumes the current player is going to play a '1 bit',
// flips the color each time
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

    pub fn get_turns(&self) -> usize {
        self.turns
    }

    pub fn size(&self) -> usize {
        self.height * self.width
    }

    pub fn flip_x(&mut self) -> &Self {
        let mut m = 0;
        let mut p = 0;

        let mask_width = self.height + 1;
        let col_mask = (0x1 << mask_width) - 1;

        for x in 0..self.width {
            let mcol = (self.mask >> ((self.width - x - 1) * mask_width)) & col_mask;
            m |= mcol << x * mask_width;

            let pcol = (self.position >> ((self.width - x - 1) * mask_width)) & col_mask;
            p |= pcol << x * mask_width;
        }

        self.mask = m;
        self.position = p;
        self
    }

    pub fn flip_color(&mut self) -> &Self {
        self.position = self.mask & !self.position;
        self
    }

    pub fn key(&self) -> u64 {
        self.position + self.mask + self.full_bottom_mask()
    }

    pub fn get_pos_mask(&self) -> (u64, u64) {
        (self.position, self.mask)
    }

    pub fn undo_to(&mut self, p: u64, m: u64) {
        self.turns -= 1;
        self.set_pos_mask(p, m);
    }

    fn set_pos_mask(&mut self, p: u64, m: u64) {
        self.position = p;
        self.mask = m;
    }

    pub fn can_play(&self, col: usize) -> bool {
        (self.mask & self.top_mask(col)) == 0
    }

    pub fn play(&mut self, col: usize) {
        self.position ^= self.mask;
        self.mask |= self.mask + self.bottom_mask(col);
        self.turns += 1;
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        let mut pos = self.position;
        pos |= (self.mask + self.bottom_mask(col)) & self.column_mask(col);
        self.alignment(pos)
    }

    fn alignment(&self, pos: u64) -> bool {
        // horizontal
        let mut m = pos & (pos >> (self.height + 1));
        if m & (m >> (2 * (self.height + 1))) != 0 {
            return true;
        }

        // diagonal 1
        m = pos & (pos >> self.height);
        if m & (m >> (2 * self.height)) != 0 {
            return true;
        }

        // diagonal 2
        m = pos & (pos >> (self.height + 2));
        if m & (m >> (2 * (self.height + 2))) != 0 {
            return true;
        }

        // vertical;
        m = pos & (pos >> 1);
        if m & (m >> 2) != 0 {
            return true;
        }

        false
    }

    fn top_mask(&self, col: usize) -> u64 {
        (1 << (self.height - 1)) << col * (self.height + 1)
    }

    fn bottom_mask(&self, col: usize) -> u64 {
        1 << col * (self.height + 1)
    }

    fn full_bottom_mask(&self) -> u64 {
        let mut res = 0;
        for x in 0..self.width {
            res += self.bottom_mask(x);
        }
        res
    }

    fn column_mask(&self, col: usize) -> u64 {
        ((1 << self.height) - 1) << col * (self.height + 1)
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
        let mut l = col.len();
        let mut bit_col = 0;
        for (i, val) in col.enumerate() {
            // ..YR -> 00101
            bit_col |= match val {
                Some(ChipDescrip::Connect(ConnectColor::Red)) => 1,
                Some(ChipDescrip::Connect(ConnectColor::Yellow)) => 0,
                Some(ChipDescrip::Toto(TotoType::T)) => 1,
                Some(ChipDescrip::Toto(TotoType::O)) => 0,
                None => {
                    l -= 1;
                    0
                }
            } << i;
        }
        bit_col |= 1 << l;

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

    let mut unpack_col = |col, c| {
        let mut data = col;
        // ..YR <- 00101
        // println!("{:#b} -> {}", col, x);
        let mut i = 0;
        while i < mask_width {
            let x = (data & 0x1 as u128) as usize;
            data >>= 1;
            game.play(c, bit_to_chip(x, Connect4));
            if x == 1 {
                i = 0;
            }
            i += 1;
        }

        for _ in 0..i {
            game.undo_move()
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

fn decode_key_128(key: u128, width: usize, height: usize) -> (u128, u128) {
    let mask_width = height + 1;
    let col_mask = (0x1 << mask_width) - 1;

    let mut mask: u128 = 0;
    for loc in 0..width {
        let col_bs = (width - loc - 1) * mask_width;
        let col: u128 = (key >> col_bs) & col_mask;
        let lead = col.leading_zeros();
        let m = (1 << (128 - lead - 1)) - 1;
        let p = col & mask;

        mask |= m << col_bs;
    }
    (mask, key & mask)
}
fn decode_key_64(key: u64, width: usize, height: usize) -> (u64, u64) {
    let (m, p) = decode_key_128(key as u128, width, height);
    (m as u64, p as u64)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::io::GameIO;

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
        // 00101_11010_10101_01110

        let game = make_game(4, 4, vec![0, 1, 2, 3, 3, 2, 1, 0, 2, 2, 3, 1, 1, 2]);
        let res = 0b00101_11010_10101_01110;
        let packed = pack_board(&game);
        println!("{:#022b}\n{:#022b}", res, packed);
        assert_eq!(res, packed)
    }

    #[test]
    fn test_pack_7x6() {
        let game = make_game(
            7,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        let res = 0b0000111_0000101_0010010_0001010_0001101_0011010_0000101;
        let packed = pack_board_128(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_unpack_7x6() {
        let game = make_game(
            7,
            6,
            vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3],
        );

        let res = 0b0000111_0000101_0010010_0001010_0001101_0011010_0000101;
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
        //assert!(check_win_4(width, 6, pack_board_64(&game), 23));
        crate::io::TermIO::draw_board(game2.get_board());
        //assert!(!check_win_4(width, 6, pack_board_64(&game2), 23));
    }

    #[test]
    fn test_bit_board_key_and_create() {
        let game = make_game(7, 6, vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0]);
        let bb = BitBoard64::from_game(&game);

        let packed = pack_board_64(&game);
        let key = bb.key();
        crate::io::TermIO::draw_board(game.get_board());
        println!(
            "BB pos:\n{:#051b}\nMask:\n{:#051b}\nBottom\n{:#051b}",
            bb.position,
            bb.mask,
            bb.full_bottom_mask()
        );
        println!("Packed:\n{:#051b}\nKey:\n{:#051b}", packed, key);
        assert_eq!(packed, key);
    }

    #[test]
    fn test_bb_flip_x() {
        let game = make_game(7, 6, vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0]);
        let mut bb = BitBoard64::from_game(&game);

        let key = bb.key();
        let key2 = bb.flip_x().key();
        let key3 = bb.flip_x().key();

        let res = 0b0000111_0000101_0001010_0000110_0000101_0000110_0000001;
        let flp = 0b0000001_0000110_0000101_0000110_0001010_0000101_0000111;
        println!(
            "Key:\n{:#051b}\nKey2:\n{:#051b}\nKey3:\n{:#051b}",
            key, key2, key3
        );
        assert_eq!(res, key);
        assert_eq!(flp, key2);
        assert_eq!(res, key3);
    }
}
