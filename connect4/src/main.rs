use connect_game::game::{four_in_a_row_red, four_in_a_row_yellow, Checker, ChipDescrip, Player};

fn connect4() {
    let board = connect_game::game::Board::new(7, 6);

    let yellow = connect_game::game::connect4_yellow();
    let red = connect_game::game::connect4_red();

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
