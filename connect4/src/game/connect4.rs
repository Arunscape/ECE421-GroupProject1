use super::{BoardState, ChipDescrip, Game, check_pattern};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectColor {
    Red,
    Yellow,
}

pub fn checker(game: &Game) -> BoardState {
    let p1_win = vec![
        ChipDescrip::Connect(ConnectColor::Red),
        ChipDescrip::Connect(ConnectColor::Red),
        ChipDescrip::Connect(ConnectColor::Red),
        ChipDescrip::Connect(ConnectColor::Red),
    ];

    let p2_win = vec![
        ChipDescrip::Connect(ConnectColor::Yellow),
        ChipDescrip::Connect(ConnectColor::Yellow),
        ChipDescrip::Connect(ConnectColor::Yellow),
        ChipDescrip::Connect(ConnectColor::Yellow),
    ];
    if check_pattern(&p1_win, game) {
        BoardState::Win(1)
    } else if check_pattern(&p2_win, game) {
        BoardState::Win(-1)
    } else {
        BoardState::Ongoing
    }
}

