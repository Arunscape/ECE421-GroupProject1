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