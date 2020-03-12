use crate::game::{check_linear_pattern, Checker, ChipDescrip, Board, Game, Player, PlayerType};
use crate::io::{BLK, BLU, BRIGHTEN, FILLED, RED, YEL};

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

pub const red: ChipDescrip = ChipDescrip {
    bg_color: BLK + BRIGHTEN,
    fg_color: RED,
    graphic: FILLED,
};
pub const yellow: ChipDescrip = ChipDescrip {
    bg_color: BLK + BRIGHTEN,
    fg_color: YEL,
    graphic: FILLED,
};
pub const blue: ChipDescrip = ChipDescrip {
    bg_color: BLK + BRIGHTEN,
    fg_color: BLU,
    graphic: FILLED,
};

pub const chip_t: ChipDescrip = ChipDescrip {
    bg_color: BLK + BRIGHTEN,
    fg_color: RED,
    graphic: 't',
};
pub const chip_o: ChipDescrip = ChipDescrip {
    bg_color: BLK + BRIGHTEN,
    fg_color: YEL,
    graphic: 'o',
};

pub fn connect4_ai() -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: PlayerType::AI,
            chip_options: vec![red],
            win_conditions: vec![four_in_a_row(red)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![yellow],
            win_conditions: vec![four_in_a_row(yellow)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4_ai_p2() -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![red],
            win_conditions: vec![four_in_a_row(red)],
        },
        Player {
            player_type: PlayerType::AI,
            chip_options: vec![yellow],
            win_conditions: vec![four_in_a_row(yellow)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4() -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![red],
            win_conditions: vec![four_in_a_row(red)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![yellow],
            win_conditions: vec![four_in_a_row(yellow)],
        },
    ];

    Game::new(board, players)
}

pub fn toto() -> Game {
    let board = Board::new(6, 4);

    let players = vec![
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![chip_t, chip_o],
            win_conditions: vec![wrap_4_check(chip_t, chip_o)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![chip_t, chip_o],
            win_conditions: vec![wrap_4_check(chip_o, chip_t)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4_3player() -> Game {
    let board = Board::new(9, 7);

    let players = vec![
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![red],
            win_conditions: vec![four_in_a_row(red)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![yellow],
            win_conditions: vec![four_in_a_row(yellow)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![blue],
            win_conditions: vec![four_in_a_row(blue)],
        },
    ];

    Game::new(board, players)
}
