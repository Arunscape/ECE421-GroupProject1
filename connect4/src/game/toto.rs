use super::{BoardState, ChipDescrip, Game};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TotoType {
    T,
    O,
}

pub fn checker(game: &Game) -> BoardState {
    println!("{:?}", game.get_board_layout());
    BoardState::Ongoing

}

