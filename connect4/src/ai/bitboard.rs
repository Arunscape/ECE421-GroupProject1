use crate::game::*;
use crate::games::{red, yellow};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Clone, Debug)]
pub struct BitBoard {
    mask: u64,
    position: u64,
    turns: usize,
}

// Assumes the current player is going to play a '1 bit',
// flips the color each time
impl BitBoard {
    pub fn from_game(game: &Game) -> Self {
        let key = pack_board(game);
        let (mask, pos) = decode_key(key, game.get_board().width, game.get_board().height);
        let mut bb = Self {
            mask: mask,
            position: pos,
            turns: game.get_turn(),
        };
        if game.get_turn() % 2 == 1 {
            bb.position ^= bb.mask;
        }
        bb
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            mask: 0,
            position: 0,
            turns: 0,
        }
    }

    pub fn get_turns(&self) -> usize {
        self.turns
    }

    pub fn size(&self) -> usize {
        HEIGHT * WIDTH
    }

    pub fn flip_x(&mut self) -> &Self {
        let mut m = 0;
        let mut p = 0;

        let mask_width = HEIGHT + 1;
        let col_mask = (0x1 << mask_width) - 1;

        for x in 0..WIDTH {
            let mcol = (self.mask >> ((WIDTH - x - 1) * mask_width)) & col_mask;
            m |= mcol << x * mask_width;

            let pcol = (self.position >> ((WIDTH - x - 1) * mask_width)) & col_mask;
            p |= pcol << x * mask_width;
        }

        self.mask = m;
        self.position = p;
        self
    }

    pub fn flip_color(&mut self) -> &Self {
        self.position = self.mask & !self.position;
        self
    }

    pub fn key(&self) -> u64 {
        self.position + self.mask + self.full_bottom_mask()
    }

    pub fn get_pos_mask(&self) -> (u64, u64) {
        (self.position, self.mask)
    }

    pub fn undo_to(&mut self, p: u64, m: u64) {
        self.turns -= 1;
        self.set_pos_mask(p, m);
    }

    fn set_pos_mask(&mut self, p: u64, m: u64) {
        self.position = p;
        self.mask = m;
    }

    pub fn can_play(&self, col: usize) -> bool {
        (self.mask & self.top_mask(col)) == 0
    }

    pub fn get_valid_moves(&self) -> Vec<usize> {
        (0..WIDTH).filter(|x| self.can_play(*x)).collect()
    }

    pub fn play(&mut self, col: usize) {
        let col = 6 - col;
        self.position ^= self.mask;
        self.mask |= self.mask + self.bottom_mask(col);
        self.turns += 1;
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        let col = 6 - col;
        let mut pos = self.position;
        pos |= (self.mask + self.bottom_mask(col)) & self.column_mask(col);
        self.alignment(pos)
    }

    fn alignment(&self, pos: u64) -> bool {
        // horizontal
        let mut m = pos & (pos >> (HEIGHT + 1));
        if m & (m >> (2 * (HEIGHT + 1))) != 0 {
            return true;
        }

        // diagonal 1
        m = pos & (pos >> HEIGHT);
        if m & (m >> (2 * HEIGHT)) != 0 {
            return true;
        }

        // diagonal 2
        m = pos & (pos >> (HEIGHT + 2));
        if m & (m >> (2 * (HEIGHT + 2))) != 0 {
            return true;
        }

        // vertical;
        m = pos & (pos >> 1);
        if m & (m >> 2) != 0 {
            return true;
        }

        false
    }

    fn top_mask(&self, col: usize) -> u64 {
        (1 << (HEIGHT - 1)) << (col * (HEIGHT + 1))
    }

    fn bottom_mask(&self, col: usize) -> u64 {
        1 << (col * (HEIGHT + 1))
    }

    fn full_bottom_mask(&self) -> u64 {
        let mut res = 0;
        for x in 0..WIDTH {
            res |= self.bottom_mask(x);
        }
        res
    }

    fn column_mask(&self, col: usize) -> u64 {
        ((1 << HEIGHT) - 1) << col * (HEIGHT + 1)
    }
}

// The board packing takes a game object and stores it in as few bits
// as possible. Currently, this only works for 2 tiletype games.
//
// This board will pack to this
// .RY.
// .YRR
// YRYR
// RYRY
//
// ((height: 4) + 1) * (width: 4) = 20 bits
// first 5 bits are the first column, RY... -> 10100
// R is 1, Y is 0, and first empty bit is 1, rest of empty bits are 0
// col1  col2  col3  col4
// 10100_01011_10101_01110
pub fn pack_board(game: &Game) -> u64 {
    assert!((1 + game.get_board().height) * game.get_board().width < 64);
    let lay = game.get_board_layout();
    let pack_column = |col| lay.iter().skip(col).step_by(game.get_board().width);
    let mut res = 0;
    let bit_width = 1 + game.get_board().height;
    for col in 0..game.get_board().width {
        let col = pack_column(col);
        let mut l = col.len();
        let mut bit_col = 0;
        for (i, val) in col.enumerate() {
            // ..YR -> 00101
            bit_col |= match val {
                Some(red) => 1,
                Some(yellow) => 0,
                Some(_) => panic!("not connect 4 board you're packing"),
                None => {
                    l -= 1;
                    0
                }
            } << i;
        }
        bit_col |= 1 << l;

        res |= bit_col;
        res <<= bit_width;
    }
    res >> bit_width
}

// cannot reproduce order of moves
pub fn unpack_board(data: u64) -> Game {
    let width = 7;
    let height = 6;
    let mask_width = height + 1;
    let col_mask = (0x1 << mask_width) - 1;

    let mut game = crate::games::connect4();

    fn bit_to_chip(bit: usize) -> ChipDescrip {
        match bit {
            1 => red,
            0 => yellow,
            _ => panic!("invalid"),
        }
    }

    let mut unpack_col = |col, c| {
        let mut data = col;
        // println!("Unpacking: {:#09b}", col);
        // ..YR <- 00101
        let mut i = 0;
        for _ in 0..mask_width {
            let x = (data & 0x1 as u64) as usize;
            data >>= 1;
            game.play(c, bit_to_chip(x));
            // println!("Inserting({}): {:?}", x, bit_to_chip(x));

            i += 1; // before the reset as we don't need to remove the
                    // final piece, as it will never actually be inserted

            if x == 1 {
                i = 0;
            }
        }

        // println!("Removing last {:?}", i);
        for _ in 0..i {
            game.undo_move()
        }
    };

    for x in 0..width {
        unpack_col((data >> ((width - x - 1) * mask_width)) & col_mask, x)
    }

    game
}

fn decode_key(key: u64, width: usize, height: usize) -> (u64, u64) {
    let mask_width = height + 1;
    let col_mask = (0x1 << mask_width) - 1;

    let mut mask: u64 = 0;
    for loc in 0..width {
        let col_bs = (width - loc - 1) * mask_width;
        let col: u64 = (key >> col_bs) & col_mask;
        let lead = col.leading_zeros();
        let m = (1 << (64 - lead - 1)) - 1;

        mask |= m << col_bs;
    }
    (mask, key & mask)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::io::GameIO;

    // modifies in place and returns a reference
    fn flip_color(game: &mut Game) -> &mut Game {
        let board = game.get_board_mut();
        board.chips.iter_mut().for_each(|chip| {
            let c = match chip.get_descrip() {
                red => yellow,
                yellow => red,
                _ => panic!("Not connect four flip"),
            };
            chip.set_descrip(c);
        });
        game
    }

    // modifies in place and returns a reference
    fn flip_x(game: &mut Game) -> &mut Game {
        let board = game.get_board_mut();
        let bw = board.width;
        board
            .chips
            .iter_mut()
            .for_each(|chip| chip.set_x(bw - (chip.get_x() + 1)));
        game
    }

    fn make_game(moves: Vec<usize>) -> Game {
        let mut game = crate::games::connect4();
        for m in moves {
            let col = game.current_player().chip_options[0];
            game.play(m, col);
        }

        game
    }

    #[test]
    fn test_pack_7x6() {
        let game = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3,
        ]);

        let res = 0b0000111_0000101_0010010_0001010_0001101_0011010_0000101;
        let packed = pack_board(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_unpack_7x6() {
        let game = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3,
        ]);

        let res = 0b0000111_0000101_0010010_0001010_0001101_0011010_0000101;
        let packed = pack_board(&game);
        println!("{:#051b}\n{:#051b}", res, packed);
        assert_eq!(res, packed);

        let game2 = unpack_board(packed);

        println!("Want:");
        crate::io::TermIO::draw_board(game.get_board());
        println!("Got:");
        crate::io::TermIO::draw_board(game2.get_board());

        let packed = pack_board(&game2);
        println!("{:#051b}", packed);
        assert_eq!(res, packed);
    }

    #[test]
    fn test_flip_x() {
        let mut game = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3,
        ]);
        let game2 = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3,
        ]);

        println!("OG:");
        crate::io::TermIO::draw_board(game.get_board());

        flip_x(&mut game);
        println!("Flipped X:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_x() == 7 - 1 - r.get_x()));

        flip_x(&mut game);
        println!("Double Flipped X:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_x() == r.get_x()));
    }

    #[test]
    fn test_flip_color() {
        let mut game = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3,
        ]);

        let game2 = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5, 3,
        ]);

        println!("OG:");
        crate::io::TermIO::draw_board(game.get_board());

        flip_color(&mut game);
        println!("Flipped Color:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_descrip() != r.get_descrip()));

        flip_color(&mut game);
        println!("Double Flipped Color:");
        crate::io::TermIO::draw_board(game.get_board());

        assert!(game
            .get_board()
            .chips
            .iter()
            .zip(game2.get_board().chips.iter())
            .all(|(l, r)| l.get_descrip() == r.get_descrip()));
    }

    fn debug_print(game: &Game) {
        let mut bb = BitBoard::from_game(&game);
        println!("OG Board:");
        crate::io::TermIO::draw_board(game.get_board());
        println!("Unpacked Board:");
        crate::io::TermIO::draw_board(unpack_board(bb.key()).get_board());
        println!("{:#051b} : Key\n{:#051b} : Mask\n{:#051b} : Pos",
                 bb.key(), bb.mask, bb.position);
    }

    #[test]
    fn test_check_win_4() {
        let game = make_game(vec![
            0, 0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5
        ]);
        crate::io::TermIO::draw_board(game.get_board());
        let bb = BitBoard::from_game(&game);
        assert!(bb.is_winning_move(3));

        let game = make_game(vec![
            0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0, 2, 4, 5, 6, 6, 5,
        ]);
        crate::io::TermIO::draw_board(game.get_board());
        assert!(!BitBoard::from_game(&game).is_winning_move(0));

        let game = make_game(vec![
            1, 2, 1, 2, 1, 2
        ]);
        crate::io::TermIO::draw_board(game.get_board());
        assert!(BitBoard::from_game(&game).is_winning_move(1));

        let game = make_game(vec![
            1, 2, 1, 2, 1, 2,
        ]);
        debug_print(&game);
        assert!(BitBoard::from_game(&game).is_winning_move(1));

        let game = make_game(vec![
            1, 2, 1, 2, 1, 2, 3
        ]);
        crate::io::TermIO::draw_board(game.get_board());
        assert!(BitBoard::from_game(&game).is_winning_move(2));

        let game = make_game(vec![
            1, 2, 1, 2, 1, 2
        ]);
        crate::io::TermIO::draw_board(game.get_board());
        assert!(!BitBoard::from_game(&game).is_winning_move(3));
        assert!(!BitBoard::from_game(&game).is_winning_move(4));
        assert!(!BitBoard::from_game(&game).is_winning_move(5));
        assert!(!BitBoard::from_game(&game).is_winning_move(6));
    }

    #[test]
    fn test_bit_board_key_and_create() {
        let game = make_game(vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0]);
        let bb = BitBoard::from_game(&game);

        let packed = pack_board(&game);
        let key = bb.key();
        crate::io::TermIO::draw_board(game.get_board());
        println!(
            "BB pos:\n{:#051b}\nMask:\n{:#051b}\nBottom\n{:#051b}",
            bb.position,
            bb.mask,
            bb.full_bottom_mask()
        );
        println!("Packed:\n{:#051b}\nKey:\n{:#051b}", packed, key);
        assert_eq!(packed, key);
    }

    #[test]
    fn test_bb_flip_x() {
        let game = make_game(vec![0, 2, 1, 3, 4, 5, 2, 2, 3, 4, 5, 1, 0]);
        let mut bb = BitBoard::from_game(&game);

        let key = bb.key();
        let key2 = bb.flip_x().key();
        let key3 = bb.flip_x().key();

        let res = 0b0000111_0000101_0001010_0000110_0000101_0000110_0000001;
        let flp = 0b0000001_0000110_0000101_0000110_0001010_0000101_0000111;
        println!(
            "Key:\n{:#051b}\nKey2:\n{:#051b}\nKey3:\n{:#051b}",
            key, key2, key3
        );
        assert_eq!(res, key);
        assert_eq!(flp, key2);
        assert_eq!(res, key3);
    }
}
