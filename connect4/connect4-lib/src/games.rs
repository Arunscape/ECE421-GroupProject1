use crate::ai::*;
use crate::game::{Board, ChipDescrip, Game, Player, PlayerType};
use crate::io::{BLK, BLU, BRIGHTEN, FILLED, RED, YEL};

pub enum GameType {
    Connect4,
    Toto,
}

pub fn four_in_a_row(chip: ChipDescrip) -> Vec<ChipDescrip> {
    vec![chip; 4]
}

pub fn wrap_4_check(chip: ChipDescrip, chip_inner: ChipDescrip) -> Vec<ChipDescrip> {
    vec![chip, chip_inner, chip_inner, chip]
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
        && game.get_player(0).chip_options[0] == RED_CHIP
        && game.get_player(1).chip_options[0] == YELLOW_CHIP
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
        && game.get_player(0).chip_options[0] == T_CHIP
        && game.get_player(0).chip_options[1] == O_CHIP
        && game.get_player(1).chip_options[0] == T_CHIP
        && game.get_player(1).chip_options[1] == O_CHIP
}

pub const RED_CHIP: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: RED as isize,
    graphic: FILLED,
};
pub const YELLOW_CHIP: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: YEL as isize,
    graphic: FILLED,
};
pub const BLUE_CHIP: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: BLU as isize,
    graphic: FILLED,
};

pub const T_CHIP: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: RED as isize,
    graphic: 't',
};
pub const O_CHIP: ChipDescrip = ChipDescrip {
    bg_color: BLK as isize + BRIGHTEN as isize,
    fg_color: YEL as isize,
    graphic: 'o',
};

pub fn connect4_large_ai() -> Game {
    let board = Board::new(14, 10);

    let players = vec![
        Player {
            player_type: PlayerType::AI(MID_AI),
            chip_options: vec![RED_CHIP],
            win_conditions: vec![four_in_a_row(RED_CHIP)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![YELLOW_CHIP],
            win_conditions: vec![four_in_a_row(YELLOW_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4_ai() -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: PlayerType::AI(HARD_AI),
            chip_options: vec![RED_CHIP],
            win_conditions: vec![four_in_a_row(RED_CHIP)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![YELLOW_CHIP],
            win_conditions: vec![four_in_a_row(YELLOW_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4_ai_p2() -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![RED_CHIP],
            win_conditions: vec![four_in_a_row(RED_CHIP)],
        },
        Player {
            player_type: PlayerType::AI(HARD_AI),
            chip_options: vec![YELLOW_CHIP],
            win_conditions: vec![four_in_a_row(YELLOW_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4() -> Game {
    connect4_custom(PlayerType::Local, PlayerType::Local)
}

pub fn connect4_custom(player1_type: PlayerType, player2_type: PlayerType) -> Game {
    let board = Board::new(7, 6);

    let players = vec![
        Player {
            player_type: player1_type,
            chip_options: vec![RED_CHIP],
            win_conditions: vec![four_in_a_row(RED_CHIP)],
        },
        Player {
            player_type: player2_type,
            chip_options: vec![YELLOW_CHIP],
            win_conditions: vec![four_in_a_row(YELLOW_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn toto_ai() -> Game {
    let board = Board::new(6, 4);

    let players = vec![
        Player {
            player_type: PlayerType::AI(HARD_AI),
            chip_options: vec![T_CHIP, O_CHIP],
            win_conditions: vec![wrap_4_check(T_CHIP, O_CHIP)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![T_CHIP, O_CHIP],
            win_conditions: vec![wrap_4_check(O_CHIP, T_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn toto() -> Game {
    toto_custom(PlayerType::Local, PlayerType::Local)
}

pub fn toto_custom(player1_type: PlayerType, player2_type: PlayerType) -> Game {
    let board = Board::new(6, 4);

    let players = vec![
        Player {
            player_type: player1_type,
            chip_options: vec![T_CHIP, O_CHIP],
            win_conditions: vec![wrap_4_check(T_CHIP, O_CHIP)],
        },
        Player {
            player_type: player2_type,
            chip_options: vec![T_CHIP, O_CHIP],
            win_conditions: vec![wrap_4_check(O_CHIP, T_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn connect4_3player() -> Game {
    let board = Board::new(9, 7);

    let players = vec![
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![RED_CHIP],
            win_conditions: vec![four_in_a_row(RED_CHIP)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![YELLOW_CHIP],
            win_conditions: vec![four_in_a_row(YELLOW_CHIP)],
        },
        Player {
            player_type: PlayerType::Local,
            chip_options: vec![BLUE_CHIP],
            win_conditions: vec![four_in_a_row(BLUE_CHIP)],
        },
    ];

    Game::new(board, players)
}

pub fn build_game(game_type: GameType, player1_type: PlayerType, player2_type: PlayerType) -> Game {
    match game_type {
        GameType::Connect4 => connect4_custom(player1_type, player2_type),
        GameType::Toto => toto_custom(player1_type, player2_type),
    }
}

#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_is_connect4() {}

    #[test]
    fn test_is_toto() {}
}
