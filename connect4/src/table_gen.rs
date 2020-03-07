use std::collections::HashMap;

use connect_game::game::connect4::ConnectColor;
use connect_game::game::toto::TotoType;
use connect_game::game::GameType::*;
use connect_game::game::*;

use connect_game::bitboard::*;

fn main() {
    gen_table(4, 4, connect4::checker)
}


fn gen_table(width: usize, height: usize, checker: fn(&Game) -> BoardState) {
    let board = connect_game::game::Board::new(4, 4);
    let mut game = connect_game::game::Game::new(board, Connect4);
    let mut data = HashMap::new();

    let mut bb = BitBoard64::from_game(&game);
    evaluate_board(&mut bb, &mut data);
}

// specifically for the 4x6 board, as it uses a u32
fn evaluate_board(game: &mut BitBoard64, data: &mut HashMap<u32, i8>) -> isize {
    /*
    fn min_or_max(isP2: bool, x1: isize, x2: isize) -> isize {
        if isP2  {// Min
            std::cmp::min(x1, x2)
        } else { // P1 -> Max
            std::cmp::max(x1, x2)
        }
    }

    fn inner(game: &mut BitBoard64, data: &mut HashMap<u32, i8>) -> isize {
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

    let p1 = game.flip_color().key(); // FL color, OG X
    let p2 = game.flip_x().key(); // FL color, FL X
    let p3 = game.flip_color().key(); // OG color, FL X
    let p4 = game.flip_x().key(); // OG color, OG X

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
    */
    0
}
