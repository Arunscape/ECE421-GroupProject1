use super::{BoardState, ChipDescrip, Game, check_pattern};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TotoType {
    T,
    O,
}

pub fn checker(game: &Game) -> BoardState {
    let p1_win = vec![
        ChipDescrip::Toto(TotoType::T),
        ChipDescrip::Toto(TotoType::O),
        ChipDescrip::Toto(TotoType::O),
        ChipDescrip::Toto(TotoType::T),
    ];

    let p2_win = vec![
        ChipDescrip::Toto(TotoType::O),
        ChipDescrip::Toto(TotoType::T),
        ChipDescrip::Toto(TotoType::T),
        ChipDescrip::Toto(TotoType::O),
    ];
    if check_pattern(&p1_win, game) {
        BoardState::Win(1)
    } else if check_pattern(&p2_win, game) {
        BoardState::Win(-1)
    } else {
        BoardState::Ongoing
    }

}

