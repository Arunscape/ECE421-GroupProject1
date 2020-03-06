use connect_game::game::*;
use connect_game::game::GameType::*;
use connect_game::game::connect4::ConnectColor;
use connect_game::game::toto::TotoType;

fn main() {
    gen_table(4, 6, connect4::checker)
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
    let pack_column = |col| {
        lay.iter().skip(col).step_by(game.get_board().width)
    };
    let mut res = 0;
    let bit_width = (1 + game.get_board().height);
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
                None => if first_none {
                    first_none = false;
                    1
                } else {
                    0
                },
            };
            bit_col <<= 1;
        }
        bit_col |= if first_none {
            1
        } else {
            0
        };

        res |= bit_col;
        res <<= bit_width;
    }
    res >> bit_width
}

fn unpack_board(data: u32, width: usize, height: usize) -> Game {
    let col_mask = (0x1 << (height+1)) - 1;
    unimplemented!();
}

fn gen_table(width: usize, height: usize, checker: fn(&Game) -> BoardState) {
    println!("hello world");

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
        let moves = [0, 1, 2, 3, 3, 2, 1, 0, 2, 2, 3, 1, 1, 2, ];
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
}
