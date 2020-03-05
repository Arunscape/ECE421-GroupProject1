use super::{BoardState, ChipDescrip, Game};

#[derive(Clone, Copy, Debug)]
pub enum ConnectColor {
    Red,
    Yellow,
}

pub fn checker(game: &Game) -> BoardState {
    println!("{:?}",
             game.board.unpack_layout(is_chipdescrip_connectcolor,
                                      chipdescipt_as_connectcolor));
    BoardState::Ongoing

}

fn chipdescipt_as_connectcolor(chip: ChipDescrip) -> ConnectColor {
    match chip {
        ChipDescrip::Connect(x) => x,
        _ => panic!("Could not convert")
    }
}

fn is_chipdescrip_connectcolor(chip: ChipDescrip) -> bool {
    match chip {
        ChipDescrip::Connect(_) => true,
        _ => false
    }
}
