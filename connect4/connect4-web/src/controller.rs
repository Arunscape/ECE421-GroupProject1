use super::canvas::Canvas;

const COLOR_BLUE: &'static str = "blue";
const COLOR_RED: &'static str = "red";
const COLOR_YELLOW: &'static str = "yellow";

const CHIP_RADIUS: f64 = 60.0;
const CHIP_DIAMETER: f64 = 2.0 * CHIP_RADIUS;
const CHIP_SEPERATION: f64 = 53.0;
const BOARD_MARGIN_X: f64 = 348.0;
const BOARD_MARGIN_Y: f64 = 0.0;
const COLUMN_WIDTH: f64 = CHIP_DIAMETER + CHIP_SEPERATION; // also height of the square around a circle

pub fn draw_chip(canvas: &Canvas, board_height: f64, chip: connect4_lib::game::ChipDescrip, x: usize, y: usize) {
    let colour = match chip.fg_color {
        connect4_lib::io::RED => COLOR_RED,
        connect4_lib::io::YEL => COLOR_YELLOW,
        _ => unreachable!(),
    };
    canvas.draw_circle(
        x as f64 * (COLUMN_WIDTH)
            + BOARD_MARGIN_X
            + CHIP_RADIUS
            + CHIP_SEPERATION,
        board_height
            - (y as f64 * (COLUMN_WIDTH)
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
                (COLUMN_WIDTH) * x as f64
                    + BOARD_MARGIN_X
                    + CHIP_RADIUS
                    + CHIP_SEPERATION,
                (COLUMN_WIDTH) * y as f64
                    + BOARD_MARGIN_Y
                    + CHIP_RADIUS
                    + CHIP_SEPERATION,
                CHIP_RADIUS,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            canvas.context.rect(
                (COLUMN_WIDTH) * x as f64 + BOARD_MARGIN_X + square,
                (COLUMN_WIDTH) * y as f64,
                -square,
                square,
            );
        }
    }
    canvas.context.fill();
    canvas.context.restore();
}

pub fn draw_gameboard(canvas: &Canvas, board: &connect4_lib::game::Board) {
    draw_board_mask(canvas, board.width, board.height);

    let chips = board.get_layout();
    for i in 0..chips.len() {
        let x = i % board.width;
        let y = i / board.width;
        let y = board.height - y - 1;
        let i = x + y * board.width;

        let board_height = CHIP_SEPERATION + (COLUMN_WIDTH) * (board.height as f64);
        match chips[i] {
            Some(chip) => draw_chip(canvas, board_height, chip, x, y),
            None => {}
        };
    }
}

pub fn canvas_loc_to_column(canvas: &Canvas, x: i32, _y: i32, board: &connect4_lib::game::Board) -> Option<usize> {
    let visual_width = canvas.canvas.get_bounding_client_rect().width();
    let render_width = canvas.canvas.width() as f64;
    let x = render_width * (x as f64) / visual_width;
    let cx = (x - BOARD_MARGIN_X - CHIP_SEPERATION / 2.0) / COLUMN_WIDTH;
    if cx < 0.0 || cx >= board.width as f64 {
        None
    } else {
        Some(cx as usize)
    }
}
