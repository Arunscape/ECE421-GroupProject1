use std::collections::HashMap;

use connect_game::game::connect4::ConnectColor;
use connect_game::game::toto::TotoType;
use connect_game::game::GameType::*;
use connect_game::game::*;

fn main() {
    gen_table(4, 4, connect4::checker)
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

fn pack_board_n(game: &Game, n: usize) -> u128 {
    assert!((1 + game.get_board().height) * game.get_board().width < n);
    pack_board_128(game) & ((0x1 << n) - 1)
}

fn pack_board_128(game: &Game) -> u128 {
    assert!((1 + game.get_board().height) * game.get_board().width < 128);
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

    let board = connect_game::game::Board::new(width, height);
    let mut game = connect_game::game::Game::new(board, Connect4);

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
    let bw = board.width;
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

fn gen_table(width: usize, height: usize, checker: fn(&Game) -> BoardState) {
    let board = connect_game::game::Board::new(4, 4);
    let mut game = connect_game::game::Game::new(board, Connect4);
    let mut data = HashMap::new();

    evaluate_board(&mut game, &mut data);
}

// specifically for the 4x6 board, as it uses a u32
fn evaluate_board(game: &mut Game, data: &mut HashMap<u32, i8>) -> isize {
    fn min_or_max(isP2: bool, x1: isize, x2: isize) -> isize {
        if isP2 {
            // Min
            std::cmp::min(x1, x2)
        } else {
            // P1 -> Max
            std::cmp::max(x1, x2)
        }
    }

    fn inner(game: &mut Game, data: &mut HashMap<u32, i8>) -> isize {
        let mut score = 0;
        for x in 0..game.get_board().width {
            let res = game.play(
                x,
                if game.get_turn() % 2 == 1 {
                    ChipDescrip::Connect(ConnectColor::Yellow)
                } else {
                    ChipDescrip::Connect(ConnectColor::Red)
                },
            );
            match res {
                BoardState::Invalid => (),
                BoardState::Ongoing => {
                    score = min_or_max(
                        game.get_turn() % 2 == 1,
                        score,
                        evaluate_board(game, data)
                    )
                }
                BoardState::Draw => score = 0,
                BoardState::Win(x) => score = x,
            }
            game.undo_move();
        }

        score
    }

    use connect_game::io::GameIO;
    // connect_game::io::TermIO::draw_board(game.get_board());

    let p1 = pack_board(flip_color(game)); // FL color, OG X
    let p2 = pack_board(flip_x(game)); // FL color, FL X
    let p3 = pack_board(flip_color(game)); // OG color, FL X
    let p4 = pack_board(flip_x(game)); // OG color, OG X

    if let Some(res) = data.get(&p1) {
        *res as isize
    } else if let Some(res) = data.get(&p2) {
        *res as isize
    } else if let Some(res) = data.get(&p3) {
        *res as isize
    } else if let Some(res) = data.get(&p4) {
        *res as isize
    } else {
        let val = inner(game, data);
        data.insert(p4, val as i8);
        val
    }
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
    #[test]
    fn test_pack_4x4() {
        // .RY.
        // .YRR
        // YRYR
        // RYRY
        // ====
        // 10100_01011_10101_01110

        let board = connect_game::game::Board::new(4, 4);
        let mut game = connect_game::game::Game::new(board, Connect4);
        let moves = [0, 1, 2, 3, 3, 2, 1, 0, 2, 2, 3, 1, 1, 2];
        let mut i = 0;
        for m in &moves {
            game.play(*m, get_chip_descript_from_int(i));
            i += 1;
        }

        let res = 0b10100_01011_10101_01110;
        let packed = pack_board(&game);
        println!("{:#b}\n{:#b}", res, packed);
        assert_eq!(res, packed)
    }

    #[test]
    fn test_pack_7x6() {
        let board = connect_game::game::Board::new(7, 6);
        let mut game = connect_game::game::Game::new(board, Connect4);
        let moves = [0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3];
        let mut i = 0;
        for m in &moves {
            game.play(*m, get_chip_descript_from_int(i));
            i += 1;
        }

        let res = 0b1110000_1010000_0100100_0101000_1011000_0101100_1010000;
        let packed = pack_board_128(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_unpack_7x6() {
        use connect_game::io::GameIO;
        let board = connect_game::game::Board::new(7, 6);
        let mut game = connect_game::game::Game::new(board, Connect4);
        let moves = [0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3];
        let mut i = 0;
        for m in &moves {
            game.play(*m, get_chip_descript_from_int(i));
            i += 1;
        }

        let res = 0b1110000_1010000_0100100_0101000_1011000_0101100_1010000;
        let packed = pack_board_128(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);

        let game2 = unpack_board(packed, 7, 6);

        println!("Want:");
        connect_game::io::TermIO::draw_board(game.get_board());
        println!("Got:");
        connect_game::io::TermIO::draw_board(game2.get_board());

        let packed = pack_board_128(&game2);
        println!("{:#051b}", packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_flip_x() {
        use connect_game::io::GameIO;
        let width = 7;
        let board = connect_game::game::Board::new(width, 6);
        let mut game = connect_game::game::Game::new(board, Connect4);
        let board = connect_game::game::Board::new(width, 6);
        let mut game2 = connect_game::game::Game::new(board, Connect4);
        let moves = [0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3];
        let mut i = 0;
        for m in &moves {
            game.play(*m, get_chip_descript_from_int(i));
            game2.play(*m, get_chip_descript_from_int(i));
            i += 1;
        }
        println!("OG:");
        connect_game::io::TermIO::draw_board(game.get_board());

        flip_x(&mut game);
        println!("Flipped X:");
        connect_game::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_x() == width - 1 - r.get_x()));

        flip_x(&mut game);
        println!("Double Flipped X:");
        connect_game::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_x() == r.get_x()));
    }

    #[test]
    fn test_flip_color() {
        use connect_game::io::GameIO;
        let width = 7;
        let board = connect_game::game::Board::new(width, 6);
        let mut game = connect_game::game::Game::new(board, Connect4);
        let board = connect_game::game::Board::new(width, 6);
        let mut game2 = connect_game::game::Game::new(board, Connect4);
        let moves = [0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3];
        let mut i = 0;
        for m in &moves {
            game.play(*m, get_chip_descript_from_int(i));
            game2.play(*m, get_chip_descript_from_int(i));
            i += 1;
        }
        println!("OG:");
        connect_game::io::TermIO::draw_board(game.get_board());

        flip_color(&mut game);
        println!("Flipped Color:");
        connect_game::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_descrip() != r.get_descrip()));

        flip_color(&mut game);
        println!("Double Flipped Color:");
        connect_game::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_descrip() == r.get_descrip()));
    }
}
