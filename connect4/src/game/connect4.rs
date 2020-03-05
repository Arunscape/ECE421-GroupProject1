use super::{BoardState, ChipDescrip, Game};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectColor {
    Red,
    Yellow,
}

pub fn checker(game: &Game) -> BoardState {
    let pat = vec![
        ChipDescrip::Connect(ConnectColor::Red),
        ChipDescrip::Connect(ConnectColor::Red),
    ];
    // println!("{:?}", check_pattern(pat, game));
    BoardState::Ongoing
}

fn check_pattern(pattern: &Vec<ChipDescrip>, game: &Game) -> bool {
    let lay = game.get_board_layout();
    let width = game.board.width;
    let height = game.board.height;
    let len = pattern.len();
    assert!(len <= width);
    assert!(len <= height);

    fn check<I>(pat: &Vec<ChipDescrip>, lay: &Vec<Option<ChipDescrip>>, nums: I) -> bool
    where
        I: Iterator<Item = usize>,
    {
        pat.iter()
            .zip(nums)
            .all(|(p, n)| lay[n].is_some() && p == &lay[n].unwrap())
    }

    let check_row = |row: usize| {
        let start = row * width;
        (0..(width - len + 1)).any(|x| check(&pattern, &lay, (start + x)..(start + x + len)))
    };

    let check_col = |col: usize| {
        (0..(height - len + 1)).any(|x| {
            check(
                &pattern,
                &lay,
                ((col + x * width)..(width * height + 1)).step_by(width),
            )
        })
    };

    let mut res = false;

    for x in 0..height {
        res |= check_row(x);
    }

    for x in 0..width {
        res |= check_col(x);
    }

    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::game::GameType::*;
    use crate::game::*;

    fn make_game(locs: &Vec<usize>) -> Game {
        let board = Board::new(7, 6);
        let mut game = Game::new(board, Connect4);

        let mut i = 0;
        for x in locs {
            let col = if i % 2 == 0 {
                ChipDescrip::Connect(ConnectColor::Red)
            } else {
                ChipDescrip::Connect(ConnectColor::Yellow)
            };
            i += 1;
            game.play(*x, col);
        }
        game
    }

    #[test]
    fn test_hor_check() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(check_pattern(
            &pat,
            &make_game(&vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 2, 1, 3])
        ));
        assert!(check_pattern(&pat, &make_game(&vec![0, 2, 1])));

        let game = make_game(&vec![0, 2, 1]);
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(!check_pattern(&pat, &game));
    }

    #[test]
    fn test_ver_check() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(check_pattern(&pat, &make_game(&vec![0, 1, 0, 1, 0, 1])));
        assert!(check_pattern(
            &pat,
            &make_game(&vec![0, 6, 1, 6, 6, 1, 6, 1, 6, 1])
        ));
        assert!(!check_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_dia_check() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];

        //assert!(check_pattern(&pat, &make_game(&vec![0, 1, 0, 1, 0, 1])));
    }
}
