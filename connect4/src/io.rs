use super::game::{ChipDescrip, Board, Game, BoardState, Player};
use std::io::{Write, stdin, stdout};

pub fn draw_term_board(game: &Board) {
    let io = TermIO::new();
    io.draw_board(game);
}

pub trait GameIO {
    fn draw_board(&self, game: &Board);
    fn get_move(&self, game: &Game) -> (usize, ChipDescrip);
    fn display_gameover(&self, ending: BoardState);
}

pub const EMPTY: char = '◻';
pub const FILLED: char = '◼';
pub const BRIGHTEN: usize = 60;
pub const FG: usize = 30;
pub const BG: usize = 40;
pub const BLK: usize = 0;
pub const RED: usize = 1;
pub const GRN: usize = 2;
pub const YEL: usize = 3;
pub const BLU: usize = 4;
pub const MAG: usize = 5;
pub const CYN: usize = 6;
pub const WHT: usize = 7;
pub const RST: usize = 9;

pub struct TermIO {
    fg: usize,
    bg: usize,
}

impl TermIO {
    pub fn new() -> Self {
        Self {
            fg: RST,
            bg: RST,
        }
    }

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
    fn draw_board(&self, game: &Board) {
        let mut drawer = Self { fg: 0, bg: 0 };
        let chips = game.get_layout();
        for i in 0..chips.len() {
            let x = i % game.width;
            let y = i / game.width;
            let y = game.height - y - 1;
            let i = x + y * game.width;
            if let Some(chip) = chips[i] {
                drawer.print_with_color(chip.graphic, chip.fg_color, chip.bg_color);
            } else {
                drawer.print_with_color(EMPTY, WHT, BLK + BRIGHTEN);
            }
            print!(" ");
            if (i + 1) % game.width == 0 {
                drawer.print_with_color('\n', RST, RST);
            }
        }

        drawer.print_with_color('1', WHT, BLK + BRIGHTEN);
        (1..game.width).for_each(|i| print!(" {}", i+1));
        print!(" ");
        Self::endpaint();
        println!();
    }

    fn get_move(&self, game: &Game) -> (usize, ChipDescrip) {
        fn read_line() -> String {
            let mut buffer = String::new();
            stdout().flush().expect("Failed to flush");
            let res = stdin().read_line(&mut buffer);
            buffer.trim().to_string()
        }
        fn get_num_in_range(lb: usize, ub: usize) -> usize {
            print!("Enter a number in range [{},{}]: ", lb, ub);
            if let Ok(v) = read_line().parse::<usize>(){
                if v >= lb && v <= ub {
                    return v;
                }
            }
            get_num_in_range(lb, ub)
        }

        println!("Player {} turn.", game.get_turn() % game.get_player_count() + 1);

        let player = game.current_player();
        let ch = if player.chip_options.len() == 1 {
            player.chip_options[0]
        } else {
            fn get_chip_type(player: &Player) -> ChipDescrip {
                let mut drawer = TermIO { fg: 0, bg: 0 };
                println!("Select chip type:");
                for chip in &player.chip_options {
                    drawer.print_with_color(chip.graphic, chip.fg_color, chip.bg_color);
                    drawer.print_with_color(' ', chip.fg_color, chip.bg_color)
                }
                TermIO::endpaint();
                println!();
                drawer.print_with_color('​', WHT, BLK + BRIGHTEN);
                for i in 0..player.chip_options.len() {
                    print!("{} ", i + 1);
                }
                TermIO::endpaint();
                println!();

                let l = player.chip_options.len();
                player.chip_options[get_num_in_range(1, l) - 1]
            }
            get_chip_type(player)
        };

        let val = get_num_in_range(1, game.get_board().width) - 1;
        (val, ch)
    }

    fn display_gameover(&self, ending: BoardState) {
        match ending {
            BoardState::Win(x) => println!("Player {} wins!", x),
            BoardState::Draw => println!("It's a draw :("),
            BoardState::Ongoing => (),
            BoardState::Invalid => println!("Illegal move!"),
        }
    }
}
