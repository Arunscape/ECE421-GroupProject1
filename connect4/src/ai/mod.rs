use super::game::{BoardState, ChipDescrip, Game};
use crate::io::{GameIO, TermIO};
use rand::prelude::*;
use std::sync::Mutex;

pub fn get_best_move(game: &mut Game) -> (usize, ChipDescrip) {
    let chip = game.current_player().chip_options[0];

    let (_, mov) = evaluate_board(game);
    (mov, chip)
}

const MAX_DEPTH: usize = 4;
const MONTE_CARLO_ITER: usize = 5000;
// returns board evaluation and next best move
pub fn evaluate_board(game: &mut Game) -> (isize, usize) {
    let is_max = game.get_turn() % 2 == 0;

    fn test_move(mov: usize, game: &mut Game) -> isize {
        game.play(mov, game.current_player().chip_options[0]);
        let mut score = minmax_search(game, MAX_DEPTH) << 14;
        if score == 0 {
            score = monte_carlo_search(game);
        }
        game.undo_move();
        score
    }

    let mut potentials: Vec<(usize, isize)> = game
        .get_board()
        .get_valid_moves()
        .iter()
        .map(|&mov| (mov, test_move(mov, &mut game.clone())))
        .collect();

    potentials.sort_by(|a, b| if is_max {
        (b.1).partial_cmp(&a.1).unwrap()
    } else {
        (a.1).partial_cmp(&b.1).unwrap()
    });

    println!("{:?}", potentials);
    let (b_mov, score) = potentials[0];
    (score >> 14, b_mov)
}

fn monte_carlo_search(game: &mut Game) -> isize {
    let mut score = 0;
    (0..MONTE_CARLO_ITER).for_each(|_| {
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

static mut COUNT: usize = 0;
// specifically a 2 player AI
// returns < 0 if player 2 wins
// returns > 0 if player 1 wins
fn minmax_search(game: &mut Game, depth: usize) -> isize {
    unsafe {
        COUNT += 1;
    }
    if depth == 0 {
        return 0;
    }

    let is_max = game.get_turn() % 2 == 0;
    if is_max {
        if game.get_player(1).win_conditions.iter().any(|x| x(game)) {
            return -(depth as isize);
        }
    } else {
        if game.get_player(0).win_conditions.iter().any(|x| x(game)) {
            return depth as isize;
        }
    }

    let minmax: fn(isize, isize) -> isize = if is_max { std::cmp::max } else { std::cmp::min };

    let mut score = if is_max {
        std::isize::MIN
    } else {
        std::isize::MAX
    };

    for mov in game.get_board().get_valid_moves() {
        game.play(mov, game.current_player().chip_options[0]);
        score = minmax(score, minmax_search(game, depth - 1));
        game.undo_move();
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

    fn make_game(moves: Vec<usize>) -> Game {
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
        TermIO::draw_board(game.get_board());
        let (eval, mov) = evaluate_board(&mut game);
        println!("Best move = {} which is {}", mov, eval);
        assert_eq!(eval, MAX_DEPTH as isize);
        assert_eq!(mov, 1);
    }

    #[test]
    fn test_win_1_p2() {
        let mut game = make_game(vec![1, 2, 1, 2, 1, 2, 0]);
        let (eval, mov) = evaluate_board(&mut game);
        assert_eq!(eval, -(MAX_DEPTH as isize));
        assert_eq!(mov, 2);
    }

    #[test]
    fn test_timing() {
        let mut game = make_game(vec![]);
        let time = time!(get_best_move(&mut game));

        unsafe {
            COUNT = 0;
        }

        let (x, _y) = get_best_move(&mut game);

        println!("This test is supposed to fail. It is for keeping track of performance");
        unsafe {
            println!(
                "Took {}µs for depth of {}. Best move is {:?}. Searched {} iterations",
                time,
                MAX_DEPTH,
                x + 1,
                COUNT
            );
        }
        assert!(false);
    }
}
