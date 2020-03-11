use connect_game::game::{check_linear_pattern, Checker, ChipDescrip, Game, Player};

use connect_game::io::{BLK, BRIGHTEN, FILLED, RED, YEL, BLU};

pub fn four_in_a_row(chip: ChipDescrip) -> Checker {
    let check = move |game: &Game| -> bool { check_linear_pattern(&vec![chip; 4], game) };
    Box::from(check)
}

pub fn wrap_4_check(chip: ChipDescrip, chip_inner: ChipDescrip) -> Checker {
    let check = move |game: &Game| -> bool {
        check_linear_pattern(&vec![chip, chip_inner, chip_inner, chip], game)
    };
    Box::from(check)
}

fn connect4() -> Game {
    let board = connect_game::game::Board::new(7, 6);

    // setup chip types
    let red = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: RED,
        graphic: FILLED,
    };
    let yellow = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: YEL,
        graphic: FILLED,
    };

    let players = vec![
        Player {
            chip_options: vec![red],
            win_conditions: vec![four_in_a_row(red)],
        },
        Player {
            chip_options: vec![yellow],
            win_conditions: vec![four_in_a_row(yellow)],
        },
    ];

    connect_game::game::Game::new(board, players)
}

fn toto() -> Game {
    let board = connect_game::game::Board::new(6, 4);

    // setup chip types
    let chip_t = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: RED,
        graphic: 't',
    };
    let chip_o = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: YEL,
        graphic: 'o',
    };

    let players = vec![
        Player {
            chip_options: vec![chip_t, chip_o],
            win_conditions: vec![wrap_4_check(chip_t, chip_o)],
        },
        Player {
            chip_options: vec![chip_t, chip_o],
            win_conditions: vec![wrap_4_check(chip_o, chip_t)],
        },
    ];

    connect_game::game::Game::new(board, players)
}

fn connect4_3player() -> Game {
    let board = connect_game::game::Board::new(9, 7);

    // setup chip types
    let red = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: RED,
        graphic: FILLED,
    };
    let yellow = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: YEL,
        graphic: FILLED,
    };
    let blue = ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: BLU,
        graphic: FILLED,
    };

    let players = vec![
        Player {
            chip_options: vec![red],
            win_conditions: vec![four_in_a_row(red)],
        },
        Player {
            chip_options: vec![yellow],
            win_conditions: vec![four_in_a_row(yellow)],
        },
        Player {
            chip_options: vec![blue],
            win_conditions: vec![four_in_a_row(blue)],
        },
    ];

    connect_game::game::Game::new(board, players)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut game = if args.len() > 1 {
        match args[1].as_ref() {
            "toto" => toto(),
            "3" => connect4_3player(),
            _ => connect4(),
        }
    } else {
        connect4()
    };
    connect_game::play(&mut game)
}
