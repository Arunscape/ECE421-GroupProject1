#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChipDescrip {
    pub bg_color: usize,
    pub fg_color: usize,
    pub graphic: char
}

pub struct Chip {
    x: usize,
    descrip: ChipDescrip,
}

impl Chip {
    pub fn new(x: usize, descrip: ChipDescrip) -> Self {
        Self { x, descrip }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn get_descrip(&self) -> ChipDescrip {
        self.descrip
    }
}


const FILLED: char = '◼';
const BLK: usize = 0;
const RED: usize = 1;
const YEL: usize = 3;
const WHT: usize = 7;
const BRIGHTEN: usize = 60;

pub fn connect4_yellow() -> ChipDescrip {
    ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: YEL,
        graphic: FILLED,
    }
}

pub fn connect4_red() -> ChipDescrip {
    ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: RED,
        graphic: FILLED,
    }
}

pub fn connect4_col(col: usize) -> ChipDescrip {
    ChipDescrip {
        bg_color: BLK + BRIGHTEN,
        fg_color: col,
        graphic: FILLED,
    }
}
