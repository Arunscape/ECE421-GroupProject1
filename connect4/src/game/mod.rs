use std::collections::HashMap;

pub mod chip;
pub use chip::*;

pub type Checker = Box<dyn Fn(&Game) -> bool>;

pub struct Player {
    pub chip_options: Vec<ChipDescrip>,
    pub win_conditions: Vec<Checker>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardState {
    Invalid,
    Win(isize),
    Draw,
    Ongoing,
}

pub struct Game {
    turn: usize,
    board: Board,
    players: Vec<Player>,
}

impl Game {
    pub fn new(board: Board, players: Vec<Player>) -> Self {
        Self {
            turn: 0,
            board,
            players,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn play(&mut self, col: usize, color: ChipDescrip) -> BoardState {
        let y = self.board.get_col_height(col);
        self.board.insert(Chip::new(col, color));
        self.turn += 1;
        if y + 1 > self.board.height {
            BoardState::Invalid
        } else {
            let player_num = (self.turn - 1) % self.players.len();
            let player = &self.players[player_num];
            let game = &self;
            if player.win_conditions.iter().any(|x| x(game)) {
                BoardState::Win(player_num as isize + 1)
            } else {
                BoardState::Ongoing
            }
        }
    }

    pub fn get_turn(&self) -> usize {
        self.turn
    }

    pub fn print_moves(&self) {
        for c in self.board.chips.iter() {
            print!("{}, ", c.get_x());
        }
    }

    pub fn get_board_layout(&self) -> Vec<Option<ChipDescrip>> {
        self.board.get_layout()
    }

    pub fn undo_move(&mut self) {
        self.turn -= 1;
        self.board.chips.pop();
    }

    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }

    pub fn get_player(&self, p: usize) -> &Player {
        &self.players[p]
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.turn % self.players.len()]
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
    }

    fn get_col_height(&self, x: usize) -> usize {
        self.chips.iter().filter(|ch| ch.get_x() == x).count()
    }

    fn chipmap(&self) -> HashMap<(usize, usize), ChipDescrip> {
        let mut heights = vec![0; self.width];
        let mut locs = HashMap::new();
        for chip in self.chips.iter() {
            locs.insert((chip.get_x(), heights[chip.get_x()]), chip.get_descrip());
            heights[chip.get_x()] += 1;
        }
        locs
    }

    pub fn get_layout(&self) -> Vec<Option<ChipDescrip>> {
        let locs = self.chipmap();
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
            .any(|x| x < 0 || x as usize >= width * height)
            || x + dx * (l - 1) < 0
            || y + dy * (l - 1) < 0
            || y + dy * (l - 1) >= height as isize
            || x + dx * (l - 1) >= width as isize
        {
            return false;
        }

        /*
        println!("Check loc[{},{}], dir[{},{}] -> {:?}",x,y,dx,dy,
                 (0..len).map(|i|idx(i)).collect::<Vec<isize>>());
        */

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
            return true;
        }
    }
    for y in 0..height {
        res |= check_line(0, y as isize, 1, 0); // horizontal
        res |= check_line(0, y as isize, 1, 1); // diagonal /
        res |= check_line(width as isize - 1, y as isize, -1, 1); // diagonal \
        if res {
            return true;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const P_RED: ChipDescrip = ChipDescrip {
        bg_color: 0,
        fg_color: 0,
        graphic: 'r',
    };

    const P_YEL: ChipDescrip = ChipDescrip {
        bg_color: 1,
        fg_color: 1,
        graphic: 'y',
    };

    pub fn four_in_a_row_red() -> Checker {
        fn check(game: &Game) -> bool {
            check_pattern(&vec![P_RED; 4], game)
        }
        Box::from(check)
    }

    pub fn four_in_a_row_yellow() -> Checker {
        fn check(game: &Game) -> bool {
            check_pattern(&vec![P_YEL; 4], game)
        }
        Box::from(check)
    }
    // specifically connect4
    fn make_game(locs: &Vec<usize>) -> Game {
        let board = Board::new(7, 6);
        let players = vec![
            Player {
                chip_options: vec![P_RED],
                win_conditions: vec![four_in_a_row_red()],
            },
            Player {
                chip_options: vec![P_YEL],
                win_conditions: vec![four_in_a_row_yellow()],
            },
        ];

        let mut game = Game::new(board, players);

        let mut i = 0;
        for x in locs {
            let col = if i % 2 == 0 { P_RED } else { P_YEL };
            i += 1;
            game.play(*x, col);
        }
        game
    }

    #[test]
    fn test_hor_check() {
        let pat = vec![P_RED, P_RED];
        assert!(check_pattern(
            &pat,
            &make_game(&vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 2, 1, 3])
        ));
        assert!(check_pattern(&pat, &make_game(&vec![0, 6, 0, 5])));

        let pat = vec![P_RED, P_RED, P_RED];
        assert!(!check_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_ver_check() {
        let pat = vec![P_RED, P_RED, P_RED];
        assert!(check_pattern(&pat, &make_game(&vec![0, 1, 0, 1, 0, 1])));
        assert!(check_pattern(
            &pat,
            &make_game(&vec![0, 6, 1, 6, 6, 1, 6, 1, 6, 1])
        ));
        assert!(!check_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_dia_check() {
        let pat = vec![P_RED, P_RED, P_RED];

        assert!(check_pattern(&pat, &make_game(&vec![0, 1, 1, 2, 3, 2, 2])));
        assert!(check_pattern(&pat, &make_game(&vec![0, 0, 0, 1, 1, 3, 2])));

        let pat = vec![P_RED, P_RED, P_RED, P_RED];
        assert!(check_pattern(
            &pat,
            &make_game(&vec![
                0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1
            ])
        ));
    }
    #[test]
    fn test_dia_check2() {
        let pat = vec![P_RED, P_RED, P_RED, P_RED];
        assert!(!check_pattern(
            &pat,
            &make_game(&vec![
                0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4
            ])
        ));
    }
}
