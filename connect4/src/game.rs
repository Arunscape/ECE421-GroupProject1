use std::collections::HashSet;

pub trait Chip: Sized {
    fn get_pos(&self) -> (usize, usize);
    fn set_pos(&mut self, x: usize, y: usize);
    fn get_draw_info(&self) -> usize;
}

pub enum ConnectColor {
    Red,
    Yellow,
}
pub struct ConnectChip {
    x: usize,
    y: usize,
    color: ConnectColor,
}

impl Chip for ConnectChip {
    fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn get_draw_info(&self) -> usize {
        match self.color {
            ConnectColor::Red => 1,
            ConnectColor::Yellow => 3,
        }
    }
}
impl ConnectChip {
    pub fn new(x: usize, y: usize, color: ConnectColor) -> Self {
        Self {
            x, y, color
        }
    }

    pub fn play(game: &mut GameBoard<Self>, col: usize, color: ConnectColor) {
        let c = Self::new(col, 10, color);
        game.insert(c);
    }
}

pub enum TotoType {
    T,
    O,
}
pub struct TotoChip {
    x: usize,
    y: usize,
    letter: TotoType,
}

impl Chip for TotoChip {
    fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn get_draw_info(&self) -> usize {
        match self.letter {
            TotoType::T => 10,
            TotoType::O => 11,
        }
    }
}
impl TotoChip {
    pub fn new(x: usize, y: usize, letter: TotoType) -> Self {
        Self {
            x, y, letter
        }
    }

    pub fn play(game: &mut GameBoard<Self>, col: usize, letter: TotoType) {
        let c = Self::new(col, 10, letter);
        game.insert(c);
    }
}



pub struct GameBoard<C: Chip> {
    pub width: usize,
    pub height: usize,
    pub chips: Vec<C>,
}

impl <C: Chip> GameBoard<C> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chips: Vec::new(),
        }
    }

    pub fn insert(&mut self, chip: C) {
        self.chips.push(chip);
        self.falldown();
    }

    pub fn falldown(&mut self) {
        for _ in 0..10 {
            self.falldown1();
        }
    }

    fn falldown1(&mut self) {
        let mut locs = HashSet::new();
        for chip in self.chips.iter() {
            locs.insert(chip.get_pos());
        }

        for chip in self.chips.iter_mut() {
            let (x, y) = chip.get_pos();
            if y > 0 && !locs.contains(&(x, y-1)) {
                chip.set_pos(x, y-1);
            }
        }
    }

    fn chip_at(&self, x: usize, y: usize) -> Option<&C> {
        self.chips.iter().find(|&chip| chip.get_pos() == (x, y))
    }
}
