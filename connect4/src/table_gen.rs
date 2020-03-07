use std::collections::HashMap;

use connect_game::game::connect4::ConnectColor;
use connect_game::game::toto::TotoType;
use connect_game::game::GameType::*;
use connect_game::game::*;

use connect_game::bitboard::*;

fn main() {
    gen_table(7, 6, connect4::checker)
}

fn gen_table(width: usize, height: usize, checker: fn(&Game) -> BoardState) {
    let board = connect_game::game::Board::new(width, height);
    let game = connect_game::game::Game::new(board, Connect4);

    let mut bb = BitBoard64::from_game(&game);
    let s = (bb.size() / 2) as isize;
    let res = evaluate_board(&mut bb, -s, s, 19);
    unsafe {
        println!(
            "res is {}. searched {} states",
            res,
            STATES,
        )
    }
}

static mut STATES: usize = 0;
fn evaluate_board(
    game: &mut BitBoard64,
    alpha: isize,
    beta: isize,
    depth: usize,
) -> isize {
    unsafe {
        STATES += 1;
    }
    if depth == 0 {
        return 0;
    }

    if game.get_turns() == game.size() {
        return 0; // Draw game, 0 score
    }

    // check for immediate win
    for x in 0..game.width {
        if game.can_play(x) && game.is_winning_move(x) {
            return ((game.size() + 1) as isize - game.get_turns() as isize) / 2;
        }
    }

    let mut beta = beta;
    let mut alpha = alpha;
    let top = ((game.size() - 1 - game.get_turns()) / 2) as isize;
    if beta > top {
        beta = top;
        if alpha >= beta {
            return beta;
        }
    }

    for x in 0..game.width {
        if game.can_play(x) {
            let (p, m) = game.get_pos_mask();
            game.play(x);
            let score = -evaluate_board(game, -beta, -alpha, depth - 1);
            game.undo_to(p, m);

            if score >= beta {
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }
    }
    alpha
}

