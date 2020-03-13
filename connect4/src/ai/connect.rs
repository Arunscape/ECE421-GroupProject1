use crate::game::{Game, ChipDescrip};
use super::bitboard::BitBoard;
use super::bitboard;
use crate::io::{GameIO, TermIO};


pub fn get_best_move(game: &mut Game) -> (usize, ChipDescrip) {
    let chip = game.current_player().chip_options[0];
    let mov = game.get_board().get_valid_moves()[0];
    let bb = BitBoard::from_game(game);

    (mov, chip)
}

fn negamax(bb: &mut BitBoard, depth: usize) -> isize {
    if depth == 0 {
        return 0
    }

    let mut score = std::isize::MIN;
    for mov in 0..7 {
        if bb.is_winning_move(mov) {
            println!("found win: {}, on turn {}", mov, bb.get_turns());
            TermIO::draw_board(bitboard::unpack_board(bb.key()).get_board());
            return ((bb.size() - bb.get_turns())/2) as isize;
        }
    }

    for mov in bb.clone().get_valid_moves() {
        let (p, m) = bb.get_pos_mask();
        bb.play(mov);
        let ns = -negamax(bb, depth - 1);
        if ns > score {
            score = ns;
        }
        bb.undo_to(p, m);
    }
    score
}


#[cfg(test)]
mod test {
    use super::*;
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
            game.play(mov, chip);
        }
        game
    }

    #[test]
    fn endgame_test() {
        let game = make_game(vec![0, 1, 0, 1, 0, 1]);
        let score = negamax(&mut BitBoard::from_game(&game), 5);
        println!("score: {}", score);
        assert_eq!(score, 18);

        let game = make_game(vec![0, 3, 0, 2, 6, 4]);
        let score = negamax(&mut BitBoard::from_game(&game), 5);
        println!("score: {}", score);
        assert_eq!(score, -17);
    }

    #[test]
    fn time_test() {
        let game = make_game(vec![]);
        let time = time!(negamax(&mut BitBoard::from_game(&game), 6));
        assert_eq!(time, 0);
    }
}
