use crate::game::*;
use crate::ai::bitboard::*;

pub fn gen_table(width: usize, height: usize) {
    let mut bb = BitBoard64::new(width, height);
    let s = (bb.size() / 2) as isize;
    let res = evaluate_board(&mut bb, -s, s, 17);
    unsafe {
        println!(
            "res is {}. searched {} states",
            res,
            STATES,
        )
    }
}

pub fn get_best_move(game: &Game) -> usize {
    let mut bb = BitBoard64::from_game(&game);

    let mut v = (0..bb.width).map(|x| if bb.can_play(x) {
        let (p, m) = bb.get_pos_mask();
        bb.play(x);
        let s = evaluate_board(&mut bb, -1, 1, 15);
        bb.undo_to(p, m);
        (s as isize, x)
    } else {
        (std::isize::MIN, x)
    }).collect::<Vec<(isize, usize)>>();
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("ints should compare"));
    v[0].1
}

static mut STATES: usize = 0;
pub fn evaluate_board(
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

