use super::game::{BoardState, ChipDescrip, Game};
use rand::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AIConfig {
    carlo_iter: isize,
    minmax_depth: isize,
}

pub const EASY_AI: AIConfig = AIConfig {
    carlo_iter: 5,
    minmax_depth: 2,
};

pub const MID_AI: AIConfig = AIConfig {
    carlo_iter: 1000,
    minmax_depth: 4,
};

pub const HARD_AI: AIConfig = AIConfig {
    carlo_iter: 4000,
    minmax_depth: 6,
};

pub fn get_best_move(game: &mut Game, ai_conf: AIConfig) -> (isize, ChipDescrip) {
    if ai_conf == HARD_AI {
        if crate::games::is_connect4(game) {
            // toto specific hard AI
        } else if crate::games::is_toto(game) {
            // toto specific hard AI
        }
    }

    let (_, mov, chip) = evaluate_board(game, ai_conf);
    (mov, chip)
}

const MINMAX_SHIFT: isize = 14;
// returns board evaluation and next best move
pub fn evaluate_board(game: &mut Game, ai_conf: AIConfig) -> (isize, isize, ChipDescrip) {
    let is_max = game.get_turn() % 2 == 0;

    fn test_move(mov: isize, chip: ChipDescrip, game: &mut Game, ai_conf: AIConfig) -> isize {
        game.play(mov, chip);
        let mut score = minmax_search(game, ai_conf.minmax_depth) << MINMAX_SHIFT;
        if score == 0 {
            score = monte_carlo_search(game, ai_conf);
        }
        game.undo_move();
        score
    }

    let mut potentials: Vec<(isize, isize, ChipDescrip)> = game
        .get_board()
        .get_valid_moves()
        .iter()
        .flat_map(|&mov| {
            game.current_player()
                .chip_options
                .iter()
                .map(move |&c| (mov, c))
        })
        .map(|(mov, c)| (test_move(mov, c, &mut game.clone(), ai_conf), mov, c))
        .collect();

    potentials.sort_by(|a, b| {
        if is_max {
            (b.0).partial_cmp(&a.0).unwrap()
        } else {
            (a.0).partial_cmp(&b.0).unwrap()
        }
    });

    // println!("{:?}", potentials);
    let (score, b_mov, c) = potentials[0];
    (score >> MINMAX_SHIFT, b_mov, c)
}

fn monte_carlo_search(game: &mut Game, ai_conf: AIConfig) -> isize {
    let mut score = 0;
    (0..ai_conf.carlo_iter).for_each(|_| {
        let mut moves = 0;
        let mut res = BoardState::Ongoing;
        let mut finished = false;
        while !finished {
            match res {
                BoardState::Ongoing => {
                    let m = game.get_board().get_valid_moves();
                    let ove = random::<usize>() % m.len();
                    let mov = m[ove];
                    let chip = random::<usize>() % game.current_player().chip_options.len();
                    let chip = game.current_player().chip_options[chip];
                    res = game.play(mov, chip);
                    moves += 1;
                }
                BoardState::Invalid => {
                    moves -= 1;
                    res = BoardState::Ongoing;
                }
                BoardState::Draw => {
                    finished = true;
                }
                BoardState::Win(x) => {
                    if x == 1 {
                        score += 1
                    } else {
                        score -= 1
                    }
                    finished = true;
                }
            }
        }
        for _ in 0..moves {
            game.undo_move()
        }
    });

    score
}

static mut COUNT: isize = 0;
// specifically a 2 player AI
// returns < 0 if player 2 wins
// returns > 0 if player 1 wins
fn minmax_search(game: &mut Game, depth: isize) -> isize {
    unsafe {
        COUNT += 1;
    }
    if depth == 0 {
        return 0;
    }

    let is_max = game.get_turn() % 2 == 0;
    if game.get_player(1).win_conditions.iter().any(|x| x(game)) {
        return -(depth as isize);
    }
    if game.get_player(0).win_conditions.iter().any(|x| x(game)) {
        return depth as isize;
    }

    let minmax: fn(isize, isize) -> isize = if is_max { std::cmp::max } else { std::cmp::min };

    let mut score = if is_max {
        std::isize::MIN
    } else {
        std::isize::MAX
    };

    let moves = game.get_board().get_valid_moves();
    let player = game.current_player().clone();
    for mov in moves {
        for chip in &player.chip_options {
            game.play_no_check(mov, *chip);
            score = minmax(score, minmax_search(game, depth - 1));
            game.undo_move();
        }
    }

    score
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::io::{GameIO, TermIO};

    use std::time::Instant;
    macro_rules! time {
        ($x:expr) => {{
            let now = Instant::now();
            $x;
            now.elapsed().as_micros()
        }};
    }

    fn make_game(moves: Vec<isize>) -> Game {
        let mut game = crate::games::connect4_ai();
        for mov in moves {
            let chip = game.current_player().chip_options[0];
            game.play(mov, chip);
        }
        game
    }

    #[test]
    fn test_win_1() {
        let mut game = make_game(vec![1, 2, 1, 2, 1, 2]);
        crate::io::draw_term_board(game.get_board());
        let ai = MID_AI;
        let (eval, mov, _) = evaluate_board(&mut game, ai);
        println!("Best move = {} which is {}", mov, eval);
        assert_eq!(eval, ai.minmax_depth as isize);
        assert_eq!(mov, 1);
    }

    #[test]
    fn test_win_1_p2() {
        let mut game = make_game(vec![1, 2, 1, 2, 1, 2, 0]);
        let ai = MID_AI;
        let (eval, mov, _) = evaluate_board(&mut game, ai);
        assert_eq!(eval, -(ai.minmax_depth as isize));
        assert_eq!(mov, 2);
    }

    #[test]
    #[ignore]
    fn test_timing() {
        let mut game = make_game(vec![]);

        unsafe {
            COUNT = 0;
        }
        let mut ai = HARD_AI;
        ai.carlo_iter += 1;
        let time = time!(get_best_move(&mut game, ai));

        println!("This test is supposed to fail. It is for keeping track of performance");
        unsafe {
            println!(
                "Took {}µs for depth of {}. Searched {} iterations",
                time, HARD_AI.minmax_depth, COUNT
            );
        }
        assert!(false);
    }
}
