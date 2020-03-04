use super::game::{Chip, GameBoard};

pub trait GameIO  {
    fn draw_board<C: Chip>(game: &GameBoard<C>);
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

pub struct TermDraw {
    fg: usize,
    bg: usize,
}

impl TermDraw {
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
impl GameIO for TermDraw {
    fn draw_board<C: Chip>(game: &GameBoard<C>) {
        let mut drawer = TermDraw {fg: 0, bg: 0};
        let mut col = RED;
        for r in (0..game.height).rev() {
            for c in 0..game.width {
                if game.chips.iter().any(|ch| if ch.get_pos() == (c, r) { col = ch.get_draw_info(); true } else { false }) {
                    if col >= 10 {
                        drawer.print_with_color(if col == 10 {'T'} else {'O'}, WHT, BLK + BRIGHTEN);
                    } else {
                        drawer.print_with_color(FILLED, col, BLK + BRIGHTEN);
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
        TermDraw::endpaint();
        println!();
    }
}
