use super::canvas::Canvas;

const COLOR_BLUE: &'static str = "blue";
const COLOR_RED: &'static str = "red";
const COLOR_YELLOW: &'static str = "yellow";

const CHIP_RADIUS: f64 = 60.0;
const CHIP_DIAMETER: f64 = 2.0 * CHIP_RADIUS;
const CHIP_SEPERATION: f64 = 53.0;
const BOARD_MARGIN_X: f64 = 348.0;
const BOARD_MARGIN_Y: f64 = 0.0;

pub fn draw_chip(canvas: &Canvas, board_height: f64, chip: crate::game::ChipDescrip, x: usize, y: usize) {
    let colour = match chip.fg_color {
        crate::io::RED => COLOR_RED,
        crate::io::YEL => COLOR_YELLOW,
        _ => unreachable!(),
    };
    canvas.draw_circle(
        x as f64 * (CHIP_DIAMETER + CHIP_SEPERATION)
            + BOARD_MARGIN_X
            + CHIP_RADIUS
            + CHIP_SEPERATION,
        board_height
            - (y as f64 * (CHIP_DIAMETER + CHIP_SEPERATION)
                + BOARD_MARGIN_Y
                + CHIP_RADIUS
                + CHIP_SEPERATION),
        CHIP_RADIUS,
        colour.into(),
        "black".into(),
    );
}

pub fn draw_board_mask(canvas: &Canvas, width: usize, height: usize) {
    let bg_color = COLOR_BLUE;
    let square = 2.0 * CHIP_SEPERATION + CHIP_DIAMETER;
    canvas.context.save();
    canvas.context.set_fill_style(&bg_color.into());
    canvas.context.begin_path();
    for y in 0..height {
        for x in 0..width {
            canvas.context.arc(
                (CHIP_DIAMETER + CHIP_SEPERATION) * x as f64
                    + BOARD_MARGIN_X
                    + CHIP_RADIUS
                    + CHIP_SEPERATION,
                (CHIP_DIAMETER + CHIP_SEPERATION) * y as f64
                    + BOARD_MARGIN_Y
                    + CHIP_RADIUS
                    + CHIP_SEPERATION,
                CHIP_RADIUS,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            canvas.context.rect(
                (CHIP_DIAMETER + CHIP_SEPERATION) * x as f64 + BOARD_MARGIN_X + square,
                (CHIP_DIAMETER + CHIP_SEPERATION) * y as f64,
                -square,
                square,
            );
        }
    }
    canvas.context.fill();
    canvas.context.restore();
}

pub fn draw_gameboard(canvas: &Canvas, board: &crate::game::Board) {
    draw_board_mask(canvas, board.width, board.height);

    let chips = board.get_layout();
    for i in 0..chips.len() {
        let x = i % board.width;
        let y = i / board.width;
        let y = board.height - y - 1;
        let i = x + y * board.width;

        let board_height = CHIP_SEPERATION + (CHIP_DIAMETER + CHIP_SEPERATION) * (board.height as f64);
        match chips[i] {
            Some(chip) => draw_chip(canvas, board_height, chip, x, y),
            None => {}
        };
    }
}

pub fn draw(canvas: &Canvas) {
    // TODO: implement the finite state machine drawing
    /*
        match self.state {
            State::GetMove => {
            call draw game board
            call function to highlight column on which a player is moused over or, to show a ghost chip where it would go if they clicked, or something that gives feedback to show they can click to select a move.
        },
            State::DrawBoard => {
            call draw game board
            call function that draws an animation of a chip falling down, or appearing into the spot
        },
            State::GameOver => {
            call function that draws a game over message and shows who wins
        }
    }
         */
}
