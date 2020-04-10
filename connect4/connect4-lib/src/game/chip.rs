use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChipDescrip {
    pub bg_color: isize,
    pub fg_color: isize,
    pub graphic: char,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chip {
    x: isize,
    descrip: ChipDescrip,
}

impl Chip {
    pub fn new(x: isize, descrip: ChipDescrip) -> Self {
        Self { x, descrip }
    }

    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn set_x(&mut self, x: isize) {
        self.x = x;
    }

    pub fn get_descrip(&self) -> ChipDescrip {
        self.descrip
    }

    pub fn set_descrip(&mut self, c: ChipDescrip) {
        self.descrip = c;
    }
}
