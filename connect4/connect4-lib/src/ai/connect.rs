use super::bitboard;
use super::bitboard::BitBoard;
use crate::game::{ChipDescrip, Game};
use crate::io::{GameIO, TermIO};

pub fn get_best_move(game: &mut Game) -> (isize, ChipDescrip) {
    let chip = game.current_player().chip_options[0];
    let mov = game.get_board().get_valid_moves()[0];
    let bb = BitBoard::from_game(game);

    (mov, chip)
}

fn solve(bb: &mut BitBoard) -> isize {
    // null window search to maximize alpha beta pruning
    let mut min = -(((bb.size() - bb.get_turns()) / 2) as isize);
    let mut max = ((bb.size() + 1 - bb.get_turns()) / 2) as isize;
    while min < max {
        let mut mid = min + (max - min) / 2;
        if mid <= 0 && min / 2 < mid {
            mid = min / 2;
        } else if mid >= 0 && max / 2 > mid {
            mid = max / 2;
        }

        unsafe {
            COUNT = 0;
        }
        let score = negamax(bb, mid, mid + 1);
        println!("got {} from window ({})-({})", score, mid, mid + 1);
        if score <= mid {
            max = score;
        } else {
            min = score;
        }
        println!("minmax is now [min: {}, max: {}]", min, max);
    }
    min
}

static mut COUNT: usize = 0;
fn negamax(bb: &BitBoard, alpha: isize, beta: isize) -> isize {
    unsafe {
        COUNT += 1;
        if COUNT > 10000000 {
            return 0;
        }
    }

    if bb.get_turns() == bb.size() {
        return 0;
    }

    if beta <= alpha {
        return alpha;
    }

    //println!("searching board: alpha({}), beta({})", alpha, beta);
    //TermIO::draw_board(bitboard::unpack_board(bb.key()).get_board());

    for mov in 0..7 {
        if bb.can_play(mov) && bb.is_winning_move(mov) {
            //println!("found win: {}, on turn {}", mov, bb.get_turns());
            //TermIO::draw_board(bitboard::unpack_board(bb.key()).get_board());
            return ((bb.size() + 1 - bb.get_turns()) / 2) as isize;
        }
    }

    let mut beta = beta;
    let max = ((bb.size() - 1 - bb.get_turns()) / 2) as isize;
    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    let mut alpha = alpha;
    for mov in bb.clone().get_valid_moves() {
        let mut p2 = bb.clone();
        p2.play(mov);
        let score = -negamax(&p2, -beta, -alpha);
        if score >= beta {
            return score;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ai::bitboard::unpack_board;
    use std::time::Instant;
    macro_rules! time {
        ($x:expr) => {{
            let now = Instant::now();
            $x;
            now.elapsed().as_micros()
        }};
    }

    fn make_game(moves: Vec<usize>) -> Game {
        let mut game = crate::games::connect4();
        for mov in moves {
            let chip = game.current_player().chip_options[0];
            game.play(mov as isize, chip);
        }
        game
    }

    #[test]
    fn endgame_test() {
        let game = make_game(vec![0, 1, 0, 1, 0, 1]);
        let score = solve(&mut BitBoard::from_game(&game));
        println!("score: {}", score);
        assert_eq!(score, 18);

        let game = make_game(vec![6, 6, 0, 1, 0, 1, 0, 1]);
        let score = solve(&mut BitBoard::from_game(&game));
        println!("score: {}", score);
        assert_eq!(score, 17);

        let game = make_game(vec![0, 1, 0, 1, 0, 1, 6]);
        let score = solve(&mut BitBoard::from_game(&game));
        println!("score: {}", score);
        assert_eq!(score, 18);

        let game = make_game(vec![0, 3, 0, 2, 6, 4]);
        let score = solve(&mut BitBoard::from_game(&game));
        crate::io::draw_term_board(game.get_board());
        println!("score: {}", score);
        assert_eq!(score, -18);

        let game = make_game(vec![0, 3, 0, 2, 6]);
        let mut bb = BitBoard::from_game(&game);
        let score = solve(&mut bb);
        crate::io::draw_term_board(unpack_board(bb.key()).get_board());
        println!("score: {}", score);
        assert_eq!(score, 18);
    }

    #[test]
    fn time_test() {
        let game = make_game(vec![]);

        let time = time!(solve(&mut BitBoard::from_game(&game)));
        println!("This test is supposed to fail. It is for keeping track of performance");
        unsafe {
            println!("Took {}µs. Searched {} iterations", time, COUNT);
        }
        assert!(false);
    }
}
