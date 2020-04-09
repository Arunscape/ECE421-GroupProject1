use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChipDescrip {
    pub bg_color: usize,
    pub fg_color: usize,
    pub graphic: char,
}

#[derive(Debug)]
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

    pub fn set_descrip(&mut self, c: ChipDescrip) {
        self.descrip = c;
    }
}

