use std::collections::HashMap;
use std::collections::HashSet;

pub mod toto;
use toto::TotoType;

pub mod connect4;
use connect4::ConnectColor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameType {
    Connect4,
    Toto,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChipDescrip {
    Connect(ConnectColor),
    Toto(TotoType),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardState {
    Win(isize),
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

    pub fn print_moves(&self) {
        for c in self.board.chips.iter() {
            print!("{}, ", c.x);
        }
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
        for x in 0..(self.width * self.height) {
            layout.push(locs.get(&(x % self.width, x / self.width)).map(|x| *x));
        }
        layout
    }

    // fn chip_at(&self, x: usize, y: usize) -> Option<Chip> {self.chips.iter().find(|&chip| chip.get_pos() == (x, y))}
}

pub fn check_pattern(pattern: &Vec<ChipDescrip>, game: &Game) -> bool {
    let lay = game.get_board_layout();
    let width = game.board.width;
    let height = game.board.height;
    let len = pattern.len();
    assert!(len <= width);
    assert!(len <= height);

    fn check_dir(
        x: isize,
        y: isize,
        dx: isize,
        dy: isize,
        len: usize,
        width: usize,
        height: usize,
        pattern: &Vec<ChipDescrip>,
        lay: &Vec<Option<ChipDescrip>>,
    ) -> bool {
        let idx = |i| ((x + dx * i as isize) + (y + dy * i as isize) * (width as isize));
        let l = len as isize;
        if (0..len)
            .map(|i| idx(i))
            .any(|x| x < 0 || x as usize >= width * height) ||
            x + dx * (l-1) < 0 ||
            y + dy * (l-1) < 0 ||
            y + dy * (l-1) >= height as isize ||
            x + dx * (l-1) >= width as isize
        {
            return false;
        }

        println!("Check loc[{},{}], dir[{},{}] -> {:?}",x,y,dx,dy,
                 (0..len).map(|i|idx(i)).collect::<Vec<isize>>());

        let check = |nums| -> bool {
            pattern.iter().zip(nums).all(|(p, n)| {
                let ln: Option<ChipDescrip> = lay[n];
                ln.is_some() && p == &ln.unwrap()
            })
        };

        check((0..len).map(|i| idx(i) as usize))
            || check_dir(x + dx, y + dy, dx, dy, len, width, height, pattern, lay)
    };

    let check_line = |x, y, dx, dy| check_dir(x, y, dx, dy, len, width, height, pattern, &lay);

    let mut res = false;

    for x in 0..width {
        res |= check_line(x as isize, 0, 0, 1); // vertical
        res |= check_line(x as isize, 0, 1, 1); // diagonal /
        res |= check_line(x as isize, 0, -1, 1); // diagonal \
        if res {
            return true
        }
    }
    for y in 0..height {
        res |= check_line(0, y as isize, 1, 0); // horizontal
        res |= check_line(0, y as isize, 1, 1); // diagonal /
        res |= check_line(width as isize - 1, y as isize, -1, 1); // diagonal \
        if res {
            return true
        }
    }

    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::game::GameType::*;

    fn make_game(locs: &Vec<usize>) -> Game {
        let board = Board::new(7, 6);
        let mut game = Game::new(board, Connect4);

        let mut i = 0;
        for x in locs {
            let col = if i % 2 == 0 {
                ChipDescrip::Connect(ConnectColor::Red)
            } else {
                ChipDescrip::Connect(ConnectColor::Yellow)
            };
            i += 1;
            game.play(*x, col);
        }
        game
    }

    #[test]
    fn test_hor_check() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(check_pattern(
            &pat,
            &make_game(&vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 2, 1, 3])
        ));
        assert!(check_pattern(&pat, &make_game(&vec![0, 6, 0, 5])));

        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(!check_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_ver_check() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(check_pattern(&pat, &make_game(&vec![0, 1, 0, 1, 0, 1])));
        assert!(check_pattern(
            &pat,
            &make_game(&vec![0, 6, 1, 6, 6, 1, 6, 1, 6, 1])
        ));
        assert!(!check_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_dia_check() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];

        assert!(check_pattern(&pat, &make_game(&vec![0, 1, 1, 2, 3, 2, 2])));
        assert!(check_pattern(&pat, &make_game(&vec![0, 0, 0, 1, 1, 3, 2])));

        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(check_pattern(
            &pat,
            &make_game(&vec![
                0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1
            ])
        ));
    }
    #[test]
    fn test_dia_check2() {
        let pat = vec![
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
            ChipDescrip::Connect(ConnectColor::Red),
        ];
        assert!(!check_pattern(
            &pat,
            &make_game(&vec![
                0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4
            ])
        ));
    }
}
