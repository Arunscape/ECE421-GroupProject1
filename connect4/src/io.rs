use super::game::{ChipDescrip, ConnectColor, GameBoard, TotoType, Game, GameType};
use std::io::{self, Write, stdin, stdout};

pub trait GameIO {
    fn draw_board(game: &GameBoard);
    fn get_move(game: &Game) -> (usize, ChipDescrip);
}

const FILLED: char = '◼';
const EMPTY: char = '◻';
const FG: usize = 30;
const BG: usize = 40;
const BRIGHTEN: usize = 60;
const BLK: usize = 0;
const RED: usize = 1;
const YEL: usize = 3;
const WHT: usize = 7;
const RST: usize = 9;

pub struct TermIO {
    fg: usize,
    bg: usize,
}

impl TermIO {
    fn paint(fg: usize, bg: usize) {
        let esc = char::from(0x1b);
        print!("{}[{};{}m", esc, fg + FG, bg + BG)
    }
    fn endpaint() {
        let esc = char::from(0x1b);
        print!("{}[0m", esc)
    }

    fn print_with_color(&mut self, s: char, fg: usize, bg: usize) {
        if fg != self.fg || bg == self.fg {
            Self::paint(fg, bg);
            self.fg = fg;
            self.bg = bg;
        }
        print!("{}", s);
    }
}
impl GameIO for TermIO {
    fn draw_board(game: &GameBoard) {
        let mut drawer = Self { fg: 0, bg: 0 };
        for r in (0..game.height).rev() {
            for c in 0..game.width {
                if let Some(chip) = game.chips.iter().find(|ch| ch.get_pos() == (c, r)) {
                    match chip.get_descrip() {
                        ChipDescrip::Connect(col) => {
                            drawer.print_with_color(
                                FILLED,
                                match col {
                                    ConnectColor::Yellow => YEL,
                                    ConnectColor::Red => RED,
                                },
                                BLK + BRIGHTEN,
                            );
                        }
                        ChipDescrip::Toto(ty) => {
                            drawer.print_with_color(
                                match ty {
                                    TotoType::T => 'T',
                                    TotoType::O => 'O',
                                },
                                WHT + BRIGHTEN,
                                BLK + BRIGHTEN,
                            );
                        }
                    }
                } else {
                    drawer.print_with_color(EMPTY, WHT, BLK + BRIGHTEN);
                }
            }
            print!(" ");
            drawer.print_with_color('\n', RST, RST);
        }

        drawer.print_with_color('1', WHT, BLK + BRIGHTEN);
        print!("234567 ");
        Self::endpaint();
        println!();
    }

    fn get_move(game: &Game) -> (usize, ChipDescrip) {
        const UNASSIGNED: usize = std::usize::MAX;
        let mut buffer = String::new();
        let mut val = UNASSIGNED;

        let ch = if let GameType::Connect4 = game.get_game_type() {
            if game.get_turn() % 2 == 0 {
                ChipDescrip::Connect(ConnectColor::Yellow)
            } else {
                ChipDescrip::Connect(ConnectColor::Red)
            }
        } else {
            fn get_toto_type() -> ChipDescrip {
                print!("Enter 't' or 'o': ");
                stdout().flush().expect("Failed to flush");
                let mut buffer = String::new();
                stdin().read_line(&mut buffer).expect("Did not get toot and otto type");
                buffer = buffer.trim().to_string();
                if buffer == "t" {
                    ChipDescrip::Toto(TotoType::T)
                } else if buffer == "o" {
                    ChipDescrip::Toto(TotoType::O)
                } else {
                    get_toto_type()
                }
            }
            get_toto_type()
        };

        print!("Enter a number in range [1,7]: ");
        stdout().flush().expect("Failed to flush");
        if let Ok(_) = stdin().read_line(&mut buffer) {
            if let Ok(v) =  buffer.trim().parse::<usize>(){
                if v > 0 && v <= 7 {
                    val = v-1;
                }
            }
        }

        if val == UNASSIGNED {
            print!("Invalid move. ");
            Self::get_move(game)
        } else {
            (val, ch)
        }
    }
}
