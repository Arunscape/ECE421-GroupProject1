use super::game::{ChipDescrip, Game};

pub fn get_best_move(game: &mut Game) -> (usize, ChipDescrip) {
    let chip = game.current_player().chip_options[0];

    let (_, mov) = evaluate_board(game);
    (mov, chip)
}

const MAX_DEPTH: usize = 4;
// returns board evaluation and next best move
pub fn evaluate_board(game: &mut Game) -> (isize, usize) {
    let is_max = game.get_turn() % 2 == 0;
    let minmax: fn(isize, isize) -> isize = if is_max { std::cmp::max } else { std::cmp::min };

    let mut score = if is_max {
        std::isize::MIN
    } else {
        std::isize::MAX
    };

    let mut b_mov = std::usize::MAX;

    for mov in game.get_board().get_valid_moves() {
        game.play(mov, game.current_player().chip_options[0]);
        let os = score;
        score = minmax(score, minmax_search(game, MAX_DEPTH));
        if score != os {
            b_mov = mov;
        }
        game.undo_move();
    }

    (score, b_mov)
}

// specifically a 2 player AI
// returns < 0 if player 2 wins
// returns > 0 if player 1 wins
fn minmax_search(game: &mut Game, depth: usize) -> isize {
    if depth == 0 {
        return 0;
    }
    if game.get_player(0).win_conditions.iter().any(|x| x(game)) {
        return depth as isize;
    }
    if game.get_player(1).win_conditions.iter().any(|x| x(game)) {
        return -(depth as isize);
    }

    let is_max = game.get_turn() % 2 == 0;
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

    use std::time::Instant;
    macro_rules! time {
        ($x:expr) => {
            {
                let now = Instant::now();
                $x;
                now.elapsed().as_micros()
            }
        };
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
        let (eval, mov) = evaluate_board(&mut game);
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
        println!("Took {}µs for depth of {}", time, MAX_DEPTH);
        assert!(false);
    }
}
