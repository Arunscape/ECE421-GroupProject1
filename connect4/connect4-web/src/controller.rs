use crate::canvas::Canvas;
use crate::log;

use connect4_lib::game::Board;
use connect4_lib::game::Chip;

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
const COLUMN_WIDTH: f64 = CHIP_DIAMETER + CHIP_SEPERATION; // also height of the square around a circle
const GRAVITY: f64 = -9.81 * 100.0/*cm/m*/ * CHIP_DIAMETER / CHIP_DIAMETER_CM;

pub fn draw_chip(
    canvas: &Canvas,
    board_height: f64,
    chip: connect4_lib::game::ChipDescrip,
    x: usize,
    y: usize,
) {
    place_chip(canvas, board_height, chip, x as f64, y as f64);
}

pub fn place_chip(
    canvas: &Canvas,
    board_height: f64,
    chip: connect4_lib::game::ChipDescrip,
    x: f64,
    y: f64,
) {
    let colour = match chip.fg_color {
        connect4_lib::io::RED => COLOR_RED,
        connect4_lib::io::YEL => COLOR_YELLOW,
        _ => unreachable!(),
    };
    canvas.draw_circle(
        x * (COLUMN_WIDTH) + BOARD_MARGIN_X + CHIP_RADIUS + CHIP_SEPERATION,
        board_height - (y * (COLUMN_WIDTH) + BOARD_MARGIN_Y + CHIP_RADIUS + CHIP_SEPERATION),
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
pub fn draw_board_mask_column(
    canvas: &Canvas,
    height: usize,
    column_num: usize,
    color: &'static str,
) {
    let square = 2.0 * CHIP_SEPERATION + CHIP_DIAMETER;
    canvas.context.save();
    canvas.context.set_fill_style(&color.into());
    canvas.context.clear_rect(
        (COLUMN_WIDTH) * column_num as f64 + BOARD_MARGIN_X + square,
        0.0,
        -square,
        square * (height as f64),
    );
    canvas.context.begin_path();
    for y in 0..height {
        canvas.context.arc(
            (COLUMN_WIDTH) * column_num as f64 + BOARD_MARGIN_X + CHIP_RADIUS + CHIP_SEPERATION,
            (COLUMN_WIDTH) * y as f64 + BOARD_MARGIN_Y + CHIP_RADIUS + CHIP_SEPERATION,
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
    draw_board_mask(canvas, board.width(), board.height());
}
fn calculate_draw_height(board_height: usize) -> f64 {
    CHIP_SEPERATION + (COLUMN_WIDTH) * (board_height as f64)
}
pub fn draw_game_pieces(canvas: &Canvas, board: &Board, chips: &[Chip]) {
    let mut heights = vec![0; board.width()];
    for c in chips {
        let x = c.get_x() as usize;
        let y = heights[x];
        heights[x] += 1;
        let board_height = calculate_draw_height(board.height());
        draw_chip(canvas, board_height, c.get_descrip(), x, y);
    }
}

pub fn canvas_loc_to_column(canvas: &Canvas, x: i32, _y: i32, board: &Board) -> Option<isize> {
    let visual_width = canvas.canvas.get_bounding_client_rect().width();
    let render_width = canvas.canvas.width() as f64;
    let x = render_width * (x as f64) / visual_width;
    let cx = (x - BOARD_MARGIN_X - CHIP_SEPERATION / 2.0) / COLUMN_WIDTH;
    if cx < 0.0 || cx >= board.width as f64 {
        None
    } else {
        Some(cx as isize)
    }
}

pub fn highlight_column(canvas: &Canvas, height: usize, col: isize) {
    draw_board_mask_column(canvas, height, col as usize, COLOR_HIGHLIGHT);
}

pub fn do_falling_piece_frame(
    canvas: &Canvas,
    ani: &mut crate::game_object::ChipAnimation,
) -> bool {
    let delta = 1.0 / 60.0; // TODO: get the actual delta
    ani.vy += delta * GRAVITY;
    ani.y += delta * ani.vy;
    if (ani.y / COLUMN_WIDTH) > ani.final_y as f64 {
        // TODO: clear rectangle behind first
        draw_board_mask_column(canvas, ani.height, ani.x as usize, COLOR_BLUE);
        place_chip(
            canvas,
            calculate_draw_height(ani.height),
            ani.chip,
            ani.x as f64,
            ani.y / (COLUMN_WIDTH),
        );
        return true;
    }
    return false;
}

pub fn message(canvas: &Canvas, msg: String) {
    canvas.context.set_font("100px Arial");
    canvas.context.fill_text(&msg, 10.0, 150.0);
}

pub fn get_chip_fall(board: &Board) -> f64 {
    COLUMN_WIDTH * ((board.height + 1) as f64)
}
