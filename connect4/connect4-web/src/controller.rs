use super::canvas::Canvas;
use connect4_lib::game::Chip;
use connect4_lib::game::Board;
use crate::log;

const COLOR_BLUE: &'static str = "blue";
const COLOR_RED: &'static str = "red";
const COLOR_YELLOW: &'static str = "yellow";

const COLOR_HIGHLIGHT: &'static str = "#0099CC";

const CHIP_RADIUS: f64 = 60.0;
const CHIP_DIAMETER: f64 = 2.0 * CHIP_RADIUS;
const CHIP_DIAMETER_CM: f64 = 6.0;
const CHIP_SEPERATION: f64 = 53.0;
const BOARD_MARGIN_X: f64 = 348.0;
const BOARD_MARGIN_Y: f64 = 0.0;
pub const COLUMN_WIDTH: f64 = CHIP_DIAMETER + CHIP_SEPERATION; // also height of the square around a circle
pub const GRAVITY: f64 = -9.81 * 100.0/*cm/m*/ * CHIP_DIAMETER / CHIP_DIAMETER_CM;

pub fn draw_chip(canvas: &Canvas, board_height: f64, chip: connect4_lib::game::ChipDescrip, x: usize, y: usize) {
    place_chip(canvas, board_height, chip, x as f64, y as f64);
}

pub fn place_chip(canvas: &Canvas, board_height: f64, chip: connect4_lib::game::ChipDescrip, x: f64, y: f64) {
    let colour = match chip.fg_color {
        connect4_lib::io::RED => COLOR_RED,
        connect4_lib::io::YEL => COLOR_YELLOW,
        _ => unreachable!(),
    };
    canvas.draw_circle(
        x * (COLUMN_WIDTH)
            + BOARD_MARGIN_X
            + CHIP_RADIUS
            + CHIP_SEPERATION,
        board_height
            - (y * (COLUMN_WIDTH)
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
    for x in 0..width {
        draw_board_mask_column(canvas, height, x, bg_color);
    }
}
pub fn draw_board_mask_column(canvas: &Canvas, height: usize, column_num: usize, color: &'static str) {

    let square = 2.0 * CHIP_SEPERATION + CHIP_DIAMETER;
    canvas.context.save();
    canvas.context.set_fill_style(&color.into());
    canvas.context.begin_path();
    for y in 0..height {
        canvas.context.arc(
            (COLUMN_WIDTH) * column_num as f64
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
            (COLUMN_WIDTH) * column_num as f64 + BOARD_MARGIN_X + square,
            (COLUMN_WIDTH) * y as f64,
            -square,
            square,
        );
    }
    canvas.context.fill();
    canvas.context.restore();
}

pub fn draw_gameboard(canvas: &Canvas, board: &connect4_lib::game::Board) {
    draw_board_mask(canvas, board.width, board.height);
}
fn calculate_draw_height(board_height: usize) -> f64 {
    CHIP_SEPERATION + (COLUMN_WIDTH) * (board_height as f64)
}
pub fn draw_game_pieces(canvas: &Canvas,
                        board: &Board,
                        chips: &[Chip]) {
    console_log!("{:?}", chips);
    let mut heights = vec![0; board.width];
    for c in chips {
        let x = c.get_x();
        let y = heights[x];
        heights[x] += 1;
        let board_height = calculate_draw_height(board.height);
        draw_chip(canvas, board_height, c.get_descrip(), x, y);
    }
}

pub fn canvas_loc_to_column(canvas: &Canvas, x: i32, _y: i32, board: &Board) -> Option<usize> {
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

pub fn highlight_column(canvas: &Canvas, col: usize) {
    draw_board_mask_column(canvas, 6, col, COLOR_HIGHLIGHT);
}

pub fn animate_falling_piece(canvas: &Canvas, chip: connect4_lib::game::ChipDescrip, board: &Board, loc: (usize, f64, f64)) {

    place_chip(canvas, calculate_draw_height(board.height), chip, loc.0 as f64, loc.1 / (COLUMN_WIDTH));
}
