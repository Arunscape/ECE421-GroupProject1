use super::{BoardState, ChipDescrip, Game};

#[derive(Clone, Copy, Debug)]
pub enum TotoType {
    T,
    O,
}

pub fn checker(game: &Game) -> BoardState {
    println!("{:?}",
             game.board.unpack_layout(is_chipdescrip_toto,
                                      chipdescipt_as_toto));
    BoardState::Ongoing

}

fn chipdescipt_as_toto(chip: ChipDescrip) -> TotoType {
    match chip {
        ChipDescrip::Toto(x) => x,
        _ => panic!("Could not convert")
    }
}

fn is_chipdescrip_toto(chip: ChipDescrip) -> bool {
    match chip {
        ChipDescrip::Toto(_) => true,
        _ => false
    }
}
