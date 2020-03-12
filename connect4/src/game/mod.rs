use std::collections::HashMap;
use std::rc::Rc;

pub mod chip;
pub use chip::*;

pub type Checker = Rc<dyn Fn(&Game) -> bool>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerType {
    Local,
    AI,
}

#[derive(Clone)]
pub struct Player {
    pub player_type: PlayerType,
    pub chip_options: Vec<ChipDescrip>,
    pub win_conditions: Vec<Checker>,
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", &self.player_type, &self.chip_options, &self.win_conditions.len())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardState {
    Invalid,
    Win(isize),
    Draw,
    Ongoing,
}

#[derive(Clone, Debug)]
pub struct Game {
    turn: usize,
    board: Board,
    players: Vec<Player>,
}


impl Game {
    pub fn new(board: Board, players: Vec<Player>) -> Self {
        Self {
            turn: 0,
            players,
            board,
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
        if y + 1 > self.board.height || col > self.board.width {
            BoardState::Invalid
        } else {
            self.board.insert(Chip::new(col, color));
            self.turn += 1;
            let player_num = (self.turn - 1) % self.players.len();
            let player = &self.players[player_num];
            let game = &self;
            if player.win_conditions.iter().any(|x| x(game)) {
                BoardState::Win(player_num as isize + 1)
            } else {
                if self.turn == self.board.width * self.board.height {
                    BoardState::Draw
                } else {
                    BoardState::Ongoing
                }
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

    pub fn get_board_layout(&self) -> &Vec<Option<ChipDescrip>> {
        self.board.get_layout()
    }

    pub fn undo_move(&mut self) {
        self.turn -= 1;
        self.board.remove_last_chip();
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

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub chips: Vec<Chip>,
    layout: Vec<Option<ChipDescrip>>,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut x = Self {
            width: self.width,
            height: self.height,
            layout: vec![None; self.width * self.height],
            chips: Vec::new(),
        };
        for chip in &self.chips {
            x.insert(Chip::new(chip.get_x(), chip.get_descrip()));
        }
        x
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chips: Vec::new(),
            layout: vec![None; height * width],
        }
    }

    fn insert(&mut self, chip: Chip) {
        let y = self.get_col_height(chip.get_x());
        self.layout[chip.get_x() + y * self.width] = Some(chip.get_descrip());
        self.chips.push(chip);
    }

    pub fn get_col_height(&self, x: usize) -> usize {
        for y in 0..self.height {
            if self.layout[x + y * self.width].is_none() {
                return y;
            }
        }
        self.height
    }

    pub fn get_valid_moves(&self) -> Vec<usize> {
        let mut v: Vec<usize> = (0..self.width)
            .filter(|x| self.get_col_height(*x) < self.height)
            .collect();
        return v;

        // Intellegent move ordering
        // v.sort_by(|a, b| {
        //     ((self.width as isize) / 2 - (*a as isize))
        //         .abs()
        //         .partial_cmp(&((self.width as isize) / 2 - (*b as isize)).abs())
        //         .unwrap()
        // });
        // v
    }

    pub fn last_move_loc(&self) -> (usize, usize) {
        let x = self.chips[self.chips.len() - 1].get_x();
        (x, self.get_col_height(x) - 1)
    }

    pub fn get_layout(&self) -> &Vec<Option<ChipDescrip>> {
        &self.layout
    }

    pub fn remove_last_chip(&mut self) {
        let chip = self.chips.pop();
        let x = chip.expect("Should never undo no moves").get_x();

        let y = self.get_col_height(x) - 1;
        self.layout[x + y * self.width] = None;
    }
}

pub fn check_linear_pattern(pattern: &Vec<ChipDescrip>, game: &Game) -> bool {
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
        //let idx = |i| ((x + dx * i as isize) + (y + dy * i as isize) * (width as isize));
        let idx = |x, y| x as usize + y as usize * width;
        let width = width as isize;
        let height = height as isize;
        let mut x = x;
        let mut y = y;
        let mut matched = 0;

        while x >= 0 && x < width && y >= 0 && y < height {
            match lay[idx(x, y)] {
                _ if matched == len => return true,
                Some(chip) if chip == pattern[matched] => {
                    matched += 1;
                }
                _ => {
                    x -= dx * (matched as isize);
                    y -= dy * (matched as isize);
                    matched = 0;
                }
            }

            x += dx;
            y += dy;
        }
        matched == len
    };

    let check_line = |x, y, dx, dy| check_dir(x, y, dx, dy, len, width, height, pattern, &lay);

    let mut res = false;

    let (x, y) = game.get_board().last_move_loc();
    let x = x as isize;
    let y = y as isize;

    let m = std::cmp::min(x, y);
    let h = game.get_board().height as isize - 1;
    let w = game.get_board().width as isize - 1;
    let m2 = std::cmp::min(x, h - y);

    res |= check_line(x, 0, 0, 1);
    res |= check_line(0, y, 1, 0); // horizontal
    res |= check_line(x - m2, y + m2, 1, -1); // diagonal \
    res |= check_line(x - m, y - m, 1, 1); // diagonal /

    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::games::*;

    // specifically connect4
    fn make_game(locs: &Vec<usize>) -> Game {
        let mut game = connect4();
        for x in locs {
            let col = game.current_player().chip_options[0];
            game.play(*x, col);
        }
        game
    }

    #[test]
    fn test_hor_check() {
        let pat = vec![red, red];
        assert!(check_linear_pattern(
            &pat,
            &make_game(&vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 2, 1, 3])
        ));
        assert!(check_linear_pattern(&pat, &make_game(&vec![0, 6, 0])));

        let pat = vec![red, red, red];
        assert!(!check_linear_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_ver_check() {
        let pat = vec![red, red, red];
        assert!(check_linear_pattern(&pat, &make_game(&vec![0, 1, 0, 1, 0])));
        assert!(check_linear_pattern(
            &pat,
            &make_game(&vec![0, 6, 1, 6, 6, 1, 6, 1, 6])
        ));
        assert!(!check_linear_pattern(&pat, &make_game(&vec![0, 2, 1])));
    }

    #[test]
    fn test_dia_check() {
        let pat = vec![red, red, red];

        assert!(check_linear_pattern(
            &pat,
            &make_game(&vec![0, 1, 1, 2, 3, 2, 2])
        ));
        assert!(check_linear_pattern(
            &pat,
            &make_game(&vec![0, 0, 0, 1, 1, 3, 2])
        ));

        let pat = vec![red, red, red, red];
        assert!(check_linear_pattern(
            &pat,
            &make_game(&vec![
                0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1
            ])
        ));
    }

    #[test]
    fn test_dia_check2() {
        let pat = vec![red, red, red, red];
        assert!(!check_linear_pattern(
            &pat,
            &make_game(&vec![
                0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4
            ])
        ));

        let pat = vec![yellow, yellow, yellow, yellow];
        assert!(check_linear_pattern(
            &pat,
            &make_game(&vec![
                3, 0, 3, 1, 3, 3, 3, 1, 3, 2, 4, 6, 4, 4, 4, 1, 1, 0, 4, 0, 0, 6, 4, 6, 6, 6, 5, 5
            ])
        ));

        let pat = vec![yellow, yellow, yellow, yellow];
        assert!(!check_linear_pattern(
            &pat,
            &make_game(&vec![
                3, 2, 3, 3, 3, 2, 3, 1, 3, 4, 2, 4, 2, 5, 2, 2, 1, 0, 1, 0, 1, 0, 1
            ])
        ));
    }

    #[test]
    fn test_check_small() {
        let pat = vec![red, red];
        assert!(check_linear_pattern(&pat, &make_game(&vec![0, 1, 0])));

        let game = make_game(&vec![2]);
        assert!(!game.current_player().win_conditions[0](&game));
    }
}
