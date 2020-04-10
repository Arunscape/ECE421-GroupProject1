use crate::ai::*;
use crate::game::{check_linear_pattern, Board, Checker, ChipDescrip, Game, Player, PlayerType};
use crate::io::{BLK, BLU, BRIGHTEN, FILLED, RED, YEL};
use std::rc::Rc;

pub fn four_in_a_row(chip: ChipDescrip) -> Checker {
    let check = move |game: &Game| -> bool { check_linear_pattern(&vec![chip; 4], game) };
    Rc::from(check)
}

pub fn wrap_4_check(chip: ChipDescrip, chip_inner: ChipDescrip) -> Checker {
    let check = move |game: &Game| -> bool {
        check_linear_pattern(&vec![chip, chip_inner, chip_inner, chip], game)
    };
    Rc::from(check)
}

pub fn is_connect4(game: &Game) -> bool {
    // TODO: this doesn't ensure that the win conditions match
    game.get_board().width == 7
        && game.get_board().height == 6
        && game.get_player_count() == 2
        && game.get_player(0).win_conditions.len() == 1
        && game.get_player(1).win_conditions.len() == 1
        && game.get_player(0).chip_options.len() == 1
        && game.get_player(1).chip_options.len() == 1
        && game.get_player(0).chip_options[0] == red
        && game.get_player(1).chip_options[0] == yellow
}

pub fn is_toto(game: &Game) -> bool {
    // TODO: this doesn't ensure that the win conditions match
    game.get_board().width == 6
        && game.get_board().height == 4
        && game.get_player_count() == 2
        && game.get_player(0).win_conditions.len() == 1
        && game.get_player(1).win_conditions.len() == 1
        && game.get_player(0).chip_options.len() == 2
        && game.get_player(1).chip_options.len() == 2
        && game.get_player(0).chip_options[0] == chip_t
        && game.get_player(0).chip_options[1] == chip_o
        && game.get_player(1).chip_options[0] == chip_t
        && game.get_player(1).chip_options[1] == chip_o
}

pub const red: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: RED as isize,
    graphic: FILLED,
};
pub const yellow: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: YEL as isize,
    graphic: FILLED,
};
pub const blue: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: BLU as isize,
    graphic: FILLED,
};

pub const chip_t: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: RED as isize,
    graphic: 't',
};
pub const chip_o: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: YEL as isize,
    graphic: 'o',
};

pub fn connect4_large_ai() -> Game {
    let board = Board::new(14, 10);

    let players = vec![
        Player {
            player_type: PlayerType::AI(MID_AI),
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

pub fn connect4_ai() -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: PlayerType::AI(HARD_AI),
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
            player_type: PlayerType::AI(HARD_AI),
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

pub fn toto_ai() -> Game {
    let board = Board::new(6, 4);

    let players = vec![
        Player {
            player_type: PlayerType::AI(HARD_AI),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_connect4() {}

    #[test]
    fn test_is_toto() {}
}
