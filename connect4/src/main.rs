use connect_game::game::GameType::*;

fn connect4() {
    let board = connect_game::game::Board::new(7, 6);
    let mut game = connect_game::game::Game::new(board, Connect4);
    connect_game::play(&mut game)
}

fn toto() {
    let board = connect_game::game::Board::new(6, 4);
    let mut game = connect_game::game::Game::new(board, Toto);
    connect_game::play(&mut game)
}

fn main() {
    if std::env::args().len() > 1 {
        toto();
    } else {
        connect4();
    }
}
