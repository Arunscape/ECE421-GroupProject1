use std::collections::HashSet;

pub struct Game {
    turn: usize,
    board: GameBoard,
    game_type: GameType,
}

impl Game {
    pub fn new(board: GameBoard, game_type: GameType) -> Self {
        Self {
            turn: 0,
            board,
            game_type,
        }
    }

    pub fn get_board(&self) -> &GameBoard {
        &self.board
    }

    pub fn play(&mut self, col: usize, color: ChipDescrip) {
        self.board.insert(Chip::new(col, 10, color));
        self.turn += 1;
    }

    pub fn get_game_type(&self) -> GameType {
        self.game_type
    }

    pub fn get_turn(&self) -> usize {
        self.turn
    }
}

#[derive(Clone, Copy)]
pub enum GameType {
    Connect4,
    Toto,
}

#[derive(Clone, Copy)]
pub enum ChipDescrip {
    Connect(ConnectColor),
    Toto(TotoType),
}

#[derive(Clone, Copy)]
pub enum ConnectColor {
    Red,
    Yellow,
}

#[derive(Clone, Copy)]
pub enum TotoType {
    T,
    O,
}

pub struct Chip {
    x: usize,
    y: usize,
    descrip: ChipDescrip,
}

impl Chip {
    fn new(x: usize, y: usize, descrip: ChipDescrip) -> Self {
        Self { x, y, descrip }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn get_descrip(&self) -> ChipDescrip {
        self.descrip
    }
}

pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    pub chips: Vec<Chip>,
}

impl GameBoard {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chips: Vec::new(),
        }
    }

    fn insert(&mut self, chip: Chip) {
        self.chips.push(chip);
        self.falldown();
    }

    fn falldown(&mut self) {
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
            if y > 0 && !locs.contains(&(x, y - 1)) {
                chip.set_pos(x, y - 1);
            }
        }
    }

    // fn chip_at(&self, x: usize, y: usize) -> Option<Chip> {self.chips.iter().find(|&chip| chip.get_pos() == (x, y))}
}
