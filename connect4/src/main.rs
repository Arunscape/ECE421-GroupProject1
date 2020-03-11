use connect_game::game::{Checker, ChipDescrip, Player, Game, check_pattern};


use connect_game::io::{BLK, BRIGHTEN, YEL, RED, FILLED};

pub fn connect4_yellow() -> ChipDescrip {
    ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: YEL,
        graphic: FILLED,
    }
}

pub fn connect4_red() -> ChipDescrip {
    ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: RED,
        graphic: FILLED,
    }
}

pub fn four_in_a_row_red() -> Checker {
    fn check(game: &Game) -> bool {
        check_pattern(&vec![connect4_red(); 4], game)
    }
    Box::from(check)
}
pub fn four_in_a_row_yellow() -> Checker {
    fn check(game: &Game) -> bool {
        check_pattern(&vec![connect4_yellow(); 4], game)
    }
    Box::from(check)
}

fn connect4() {
    let board = connect_game::game::Board::new(7, 6);

    let red = connect4_red();
    let yellow = connect4_yellow();

    let mut game = connect_game::game::Game::new(
        board,
        vec![
            Player {
                chip_options: vec![red],
                win_conditions: vec![four_in_a_row_red()],
            },
            Player {
                chip_options: vec![yellow],
                win_conditions: vec![four_in_a_row_yellow()],
            },
        ],
    );
    connect_game::play(&mut game)
}

fn toto() {
    let board = connect_game::game::Board::new(6, 4);
    // let mut game = connect_game::game::Game::new(board, Toto);
    // connect_game::play(&mut game)
}

fn main() {
    if std::env::args().len() > 1 {
        toto();
    } else {
        connect4();
    }
}
