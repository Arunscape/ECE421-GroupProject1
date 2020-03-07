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
    println!("{} items in table", data.len())
}

fn calc_evaluate_board(game: &mut BitBoard64, data: &mut HashMap<u64, i8>) -> isize {
    if game.get_turns() == game.size() {
        return 0; // Draw game, 0 score
    }

    // check for immediate win
    for x in 0..game.width {
        if game.can_play(x) && game.is_winning_move(x) {
            return ((game.size() + 1) as isize - game.get_turns() as isize) / 2;
        }
    }

    let mut score = std::isize::MIN;
    for x in 0..game.width {
        if game.can_play(x) {
            let (p, m) = game.get_pos_mask();
            game.play(x);
            score = std::cmp::max(-evaluate_board(game, data), score);
            game.undo_to(p, m);
        }
    }
    score
}

fn evaluate_board(game: &mut BitBoard64, data: &mut HashMap<u64, i8>) -> isize {
    let k1 = game.flip_color().key();
    let k2 = game.flip_x().key();
    let k3 = game.flip_color().key();
    let k4 = game.flip_x().key();
    if let Some(score) = data.get(&k1) {
        -(*score) as isize
    } else if let Some(score) = data.get(&k2) {
        -(*score) as isize
    } else if let Some(score) = data.get(&k3) {
        (*score) as isize
    } else if let Some(score) = data.get(&k4) {
        (*score) as isize
    } else {
        let score = calc_evaluate_board(game, data);
        data.insert(game.key(), score as i8);
        score
    }
}
