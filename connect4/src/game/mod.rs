use std::collections::HashSet;
use std::collections::HashMap;

pub mod toto;
use toto::TotoType;

pub mod connect4;
use connect4::ConnectColor;

#[derive(Clone, Copy, Debug)]
pub enum GameType {
    Connect4,
    Toto,
}

#[derive(Clone, Copy, Debug)]
pub enum ChipDescrip {
    Connect(ConnectColor),
    Toto(TotoType),
}

#[derive(Clone, Copy, Debug)]
pub enum BoardState {
    Win(usize),
    Draw,
    Ongoing,
}


pub struct Game {
    turn: usize,
    board: Board,
    game_type: GameType,
}

impl Game {
    pub fn new(board: Board, game_type: GameType) -> Self {
        Self {
            turn: 0,
            board,
            game_type,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn play(&mut self, col: usize, color: ChipDescrip) -> BoardState {
        self.board.insert(Chip::new(col, 10, color));
        self.turn += 1;
        self.check_state()
    }

    pub fn get_game_type(&self) -> GameType {
        self.game_type
    }

    pub fn get_turn(&self) -> usize {
        self.turn
    }

    fn check_state(&self) -> BoardState {
        match self.game_type {
            GameType::Connect4 => connect4::checker(&self),
            GameType::Toto => toto::checker(&self),
        }
    }

    fn get_board_layout(&self) -> Vec<Option<ChipDescrip>> {
        self.board.get_layout()
    }
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

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub chips: Vec<Chip>,
}

impl Board {
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

    fn get_layout(&self) -> Vec<Option<ChipDescrip>> {
        let mut locs = HashMap::new();
        for chip in self.chips.iter() {
            locs.insert(chip.get_pos(), chip.get_descrip());
        }
        let mut layout = Vec::with_capacity(self.width * self.height);
        for x in 0..(self.width*self.height) {
            layout.push(locs.get(&(x % self.width, x / self.width)).map(|x| *x));
        }
        layout
    }

    pub fn unpack_layout<C>(&self, valid: fn(ChipDescrip) -> bool,
                        convert: fn(ChipDescrip) -> C) -> Option<Vec<Option<C>>> {
        let layout = self.get_layout();
        if layout.iter().all(|x| x.is_none() ||
                             x.map(valid).unwrap()) {
            Some(layout.iter().map(|x| x.map(convert)).collect())
        } else {
            None
        }
    }

    // fn chip_at(&self, x: usize, y: usize) -> Option<Chip> {self.chips.iter().find(|&chip| chip.get_pos() == (x, y))}
}

