use crate::canvas::Canvas;
use crate::storage::LocalStorage;

use connect4_lib::game::Board;
use connect4_lib::game::Chip;
use connect4_lib::game::ChipDescrip;
use connect4_lib::game::Player;
use connect4_lib::io;

const COLOR_BLUE: &'static str = "blue";
const COLOR_RED: &'static str = "red";
const COLOR_YELLOW: &'static str = "yellow";
const COLOR_GREEN: &'static str = "green";
const COLOR_BLACK: &'static str = "black";
const COLOR_WHITE: &'static str = "white";
const COLOR_MAGENTA: &'static str = "fuchsia";
const COLOR_CYAN: &'static str = "cyan";

const COLOR_HIGHLIGHT: &'static str = COLOR_CYAN;

const CHIP_RADIUS: f64 = 60.0;
const CHIP_DIAMETER: f64 = 2.0 * CHIP_RADIUS;
const CHIP_DIAMETER_CM: f64 = 6.0;
const CHIP_SEPERATION: f64 = 53.0;
const COLUMN_WIDTH: f64 = CHIP_DIAMETER + CHIP_SEPERATION; // also height of the square around a circle
const GRAVITY: f64 = -9.81 * 100.0/*cm/m*/ * CHIP_DIAMETER / CHIP_DIAMETER_CM;

pub fn draw_chip(
    canvas: &Canvas,
    width: isize,
    height: isize,
    chip: connect4_lib::game::ChipDescrip,
    x: usize,
    y: usize,
) {
    draw_chip_at(canvas, width, height, chip, x as f64, y as f64);
}
// in draw_chip, and draw_chip_at, both x and y are in board coordinates
pub fn draw_chip_at(
    canvas: &Canvas,
    width: isize,
    height: isize,
    chip: connect4_lib::game::ChipDescrip,
    x: f64,
    y: f64,
) {
    let (board_margin_x, board_margin_y, _pwidth, pheight, box_size) =
        get_rendering_gameboard_bounds(canvas, width, height);
    let chip_seperation = (box_size / COLUMN_WIDTH) * CHIP_SEPERATION;
    let chip_radius = (box_size / COLUMN_WIDTH) * CHIP_RADIUS;
    let column_width = box_size;

    place_chip(
        canvas,
        chip,
        x * (column_width) + board_margin_x + chip_radius + chip_seperation,
        board_margin_y + pheight - (y * (column_width) + chip_radius), // TODO: add chip_seperation when I break this later
        chip_radius,
    );
}

// in place_chip both x and y are in canvas coordinates
pub fn place_chip(
    canvas: &Canvas,
    chip: connect4_lib::game::ChipDescrip,
    x: f64,
    y: f64,
    radius: f64,
) {
    let colour = match chip.fg_color {
        io::RED => COLOR_RED,
        io::YEL => COLOR_YELLOW,
        io::BLK => COLOR_RED,
        io::WHT => COLOR_WHITE,
        io::GRN => COLOR_GREEN,
        io::BLU => COLOR_BLUE,
        io::MAG => COLOR_MAGENTA,
        io::CYN => COLOR_CYAN,
        _ => COLOR_BLACK,
    };
    canvas.draw_circle(x, y, radius, colour.into(), "black".into());

    let letter_to_draw = match chip.graphic {
        io::FILLED => match LocalStorage::get_colorblind_setting() {
            true => match chip.fg_color {
                io::RED => Some('R'),
                io::YEL => Some('Y'),
                _ => unreachable!(),
            },
            false => None,
        },
        c => Some(c),
    };
    match letter_to_draw {
        None => {} // do nothing extra
        Some(c) => {
            canvas.context.set_font(&font_size(radius as usize));
            canvas
                .context
                .fill_text(
                    &format!("{}", c),
                    x - radius * (1.0 / 4.0),
                    y + radius * (1.0 / 2.0),
                )
                .unwrap();
        }
    }
}

pub fn draw_board_mask(canvas: &Canvas, width: isize, height: isize) {
    let bg_color = COLOR_BLUE;
    for x in 0..width {
        draw_board_mask_column(canvas, width, height, x as usize, bg_color);
    }
}
pub fn draw_board_mask_column(
    canvas: &Canvas,
    width: isize,
    height: isize,
    column_num: usize,
    color: &'static str,
) {
    draw_board_mask_column_above(canvas, width, height, column_num, color, 0);
}
pub fn draw_board_mask_column_above(
    canvas: &Canvas,
    width: isize,
    height: isize,
    column_num: usize,
    color: &'static str,
    above: isize,
) {
    let (off_x, off_y, _pwidth, _pheight, box_size) =
        get_rendering_gameboard_bounds(canvas, width, height);
    let chip_seperation = (box_size / COLUMN_WIDTH) * CHIP_SEPERATION;
    let chip_radius = (box_size / COLUMN_WIDTH) * CHIP_RADIUS;
    let chip_diameter = (box_size / COLUMN_WIDTH) * CHIP_DIAMETER;
    let square = 2.0 * chip_seperation + chip_diameter;
    canvas.context.save();
    canvas.context.set_fill_style(&color.into());
    canvas.context.begin_path();
    // TODO: this isn't quite clearing properly, so the chip looks like it slides over the
    // boards. It's not a big issue, but it would be nice to fix
    for y in 0..(height - above) {
        canvas.context.clear_rect(
            (box_size) * column_num as f64 + off_x + square,
            (box_size) * y as f64 + off_y,
            -square,
            square,
        );
        canvas
            .context
            .arc(
                (box_size) * column_num as f64 + off_x + chip_radius + chip_seperation,
                (box_size) * y as f64 + off_y + chip_radius + chip_seperation,
                chip_radius,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
        canvas.context.rect(
            (box_size) * column_num as f64 + off_x + square,
            (box_size) * y as f64 + off_y,
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

pub fn draw_game_pieces(canvas: &Canvas, board: &Board, chips: &[Chip]) {
    let mut heights = vec![0; board.width()];
    for c in chips {
        let x = c.get_x() as usize;
        let y = heights[x];
        heights[x] += 1;
        draw_chip(canvas, board.width, board.height, c.get_descrip(), x, y);
    }
}

use crate::{console_log, log};
pub fn canvas_loc_to_column(canvas: &Canvas, x: i32, y: i32, board: &Board) -> Option<isize> {
    let visual_width = canvas.canvas.get_bounding_client_rect().width();
    let render_width = canvas.canvas.width() as f64;
    let visual_height = canvas.canvas.get_bounding_client_rect().height();
    let render_height = canvas.canvas.height() as f64;
    let tx = render_width * (x as f64) / visual_width;
    let ty = render_height * (y as f64) / visual_height;
    let (x, y, w, h, _) = get_rendering_gameboard_bounds(canvas, board.width, board.height);
    if tx < x || tx >= w + x || ty < y || ty >= h + y {
        None
    } else {
        console_log!(
            "TX: {} -> COL: {}",
            tx,
            ((tx - x) / w) * (board.width as f64)
        );
        Some(((tx - x) / w * (board.width as f64)) as isize)
    }
}

pub fn highlight_column(canvas: &Canvas, width: isize, height: isize, col: isize) {
    draw_board_mask_column(canvas, width, height, col as usize, COLOR_HIGHLIGHT);
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
        draw_board_mask_column_above(
            canvas,
            ani.width,
            ani.height,
            ani.x as usize,
            COLOR_BLUE,
            ani.final_y,
        );
        draw_chip_at(
            canvas,
            ani.width,
            ani.height,
            ani.chip,
            ani.x as f64,
            ani.y / (COLUMN_WIDTH),
        );
        return true;
    }
    return false;
}
pub fn get_chip_fall(board: &Board) -> f64 {
    COLUMN_WIDTH * ((board.height + 1) as f64)
}

pub fn message(canvas: &Canvas, msg: String) {
    let (x, y, w, h) = get_message_bounds(canvas);
    canvas.context.clear_rect(x, y, w, h);
    let fsize = (h / if canvas.is_skinny() { 4.0 } else { 7.0 }) as usize;

    canvas.context.set_font(&font_size(fsize));

    let mut lines = vec![String::new()];

    for word in msg.split(' ') {
        let word = String::from(word);
        let last = lines.len() - 1;
        if is_msg_width_okay(canvas, &(lines[lines.len() - 1].clone() + &word)) {
            lines[last] += &(word + " ");
        } else {
            if lines[last] == "" {
                console_log!("Attempted to draw message that would overflow hbox");
                return;
            }
            lines.push(word + " ");
        }
    }
    // TODO: center message in field
    console_log!("Wrapped to: {:?}", lines);
    let mut y = y;
    for line in lines {
        canvas
            .context
            .fill_text(&line, x, y + fsize as f64)
            .unwrap();
        y += fsize as f64;
    }
}

fn is_msg_width_okay(canvas: &Canvas, msg: &str) -> bool {
    let (_, _, w, _) = get_message_bounds(canvas);
    canvas
        .context
        .measure_text(msg)
        .ok()
        .map(|t| t.width() < w)
        .unwrap_or(true)
}

pub fn draw_move_selection(canvas: &Canvas, player: &Player, chip: Option<ChipDescrip>) {
    let (x, y, w, h) = get_chipselect_bounds(canvas);
    let fs = (w / 10.0).min(h / 10.0);

    canvas.context.clear_rect(x, y, w, h);
    let all_chips = &player.chip_options;
    let selected_chip = chip.or(player.chip_options.iter().cloned().next()); // default first option

    if let Some(selected_chip) = selected_chip {
        if canvas.is_skinny() {
            let x1 = x;
            let y1 = y;
            let w1 = w / 3.0;
            let h1 = h;
            draw_selected_move_selection(canvas, selected_chip, x1, y1, w1, h1, fs);
            let x2 = x + w / 3.0;
            let y2 = y;
            let w2 = 2.0 * w / 3.0;
            let h2 = h;
            draw_unselected_move_selection(canvas, all_chips, x2, y2, w2, h2, fs);
        } else {
            let x1 = x;
            let y1 = y;
            let w1 = w;
            let h1 = h / 2.0;
            draw_selected_move_selection(canvas, selected_chip, x1, y1, w1, h1, fs);
            let x2 = x;
            let y2 = y + h / 2.0;
            let w2 = w;
            let h2 = h / 2.0;
            draw_unselected_move_selection(canvas, all_chips, x2, y2, w2, h2, fs);
        }
    } else {
        console_log!("No available chips");
    }
}

fn draw_selected_move_selection(
    canvas: &Canvas,
    chip: ChipDescrip,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    fontsize: f64,
) {
    let ratio = 2.0 / 6.0;
    let rbox_size = w.min(h - fontsize);
    let box_size = rbox_size * ratio;
    let r = box_size / 2.0;
    place_chip(canvas, chip, x + w / 2.0, y + h / 2.0, r);
    canvas.context.set_text_align("center");
    canvas.context.set_font(&font_size(fontsize as usize));
    canvas
        .context
        .fill_text("Selected", x + w / 2.0, y + h / 2.0 + fontsize + r)
        .unwrap();
    canvas.context.set_text_align("left");
}
fn draw_unselected_move_selection(
    canvas: &Canvas,
    chips: &[ChipDescrip],
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    fontsize: f64,
) {
    let ratio = 2.0 / 6.0;
    let rbox_size = w.min(h - fontsize);
    let box_size = rbox_size * ratio;
    let r = box_size / 2.0;
    let pad = r / 3.0;

    let sw = 3.0 * r;
    let tw = sw * chips.len() as f64 - sw;
    for (i, &ch) in chips.iter().enumerate() {
        place_chip(
            canvas,
            ch,
            x + w / 2.0 - tw / 2.0 + (i as f64) * sw,
            y + h - 2.0 * fontsize - 2.0 * pad - 2.0 * r,
            r,
        );
    }
    canvas.context.set_text_align("center");
    canvas.context.set_font(&font_size(fontsize as usize));
    canvas
        .context
        .fill_text("Options", x + w / 2.0, y + h - fontsize - pad)
        .unwrap();
    canvas.context.set_text_align("left");
}

pub fn selected_new_move(
    canvas: &Canvas,
    loc: (i32, i32),
    chips: &[ChipDescrip],
) -> Option<ChipDescrip> {
    let x = loc.0;
    let y = loc.1;

    let visual_width = canvas.canvas.get_bounding_client_rect().width();
    let render_width = canvas.canvas.width() as f64;
    let visual_height = canvas.canvas.get_bounding_client_rect().height();
    let render_height = canvas.canvas.height() as f64;
    let tx = render_width * (x as f64) / visual_width;
    let ty = render_height * (y as f64) / visual_height;

    let (x, y, w, h) = get_chipselect_bounds(canvas);
    // TODO: move this math somewhere else.
    // It needs to be the same as in draw_move_selection
    let fs = (w / 10.0).min(h / 10.0).floor();
    let (x, y, w, h) = if canvas.is_skinny() {
        (x + w / 3.0, y, 2.0 * w / 3.0, h)
    } else {
        (x, y + h / 2.0, w, h / 2.0)
    };

    let ratio = 2.0 / 6.0;
    let rbox_size = w.min(h - fs);
    let box_size = rbox_size * ratio;
    let r = box_size / 2.0;
    let pad = r / 3.0;

    let sw = 3.0 * r;
    let tw = sw * chips.len() as f64 - sw;

    let off_x = x + w / 2.0 - tw / 2.0 - r - r / 2.0; // r/2 is for spaceing, r is cause circles are drawn from center
    let off_y = y + h - 2.0 * fs - 2.0 * pad - 2.0 * r;
    let sloc = (tx - off_x, ty - off_y + r);
    console_log!(
        "Touched at: {:?}, which is {:?} relativly -> {:?}",
        loc,
        sloc,
        chips.get((sloc.0 / sw) as usize)
    );
    console_log!(
        "    touch data: i={} r={}, tw={}, sw={}",
        (sloc.0 / sw),
        r,
        tw,
        sw
    );
    let i = sloc.0 / sw;
    if sloc.1.abs() <= (r * 2.0) && i >= 0.0 && i < chips.len() as f64 {
        return Some(chips[i as usize]);
    }

    None
}

pub fn font_size(size: usize) -> String {
    format!("{}px Poppins", size)
}

fn get_message_bounds(canvas: &Canvas) -> (f64, f64, f64, f64) {
    if canvas.is_skinny() {
        (0.0, 0.0, canvas.get_width(), canvas.get_height() / 4.0)
    } else {
        (
            0.0,
            0.0,
            canvas.get_width() / 4.0,
            canvas.get_height() / 2.0,
        )
    }
}

fn get_gameboard_bounds(canvas: &Canvas) -> (f64, f64, f64, f64) {
    if canvas.is_skinny() {
        (
            0.0,
            canvas.get_height() / 4.0,
            canvas.get_width(),
            canvas.get_height() / 2.0,
        )
    } else {
        (
            canvas.get_width() / 4.0,
            0.0,
            3.0 * canvas.get_width() / 4.0,
            canvas.get_height(),
        )
    }
}

fn get_chipselect_bounds(canvas: &Canvas) -> (f64, f64, f64, f64) {
    if canvas.is_skinny() {
        (
            0.0,
            3.0 * canvas.get_height() / 4.0,
            canvas.get_width(),
            canvas.get_height() / 4.0,
        )
    } else {
        (
            0.0,
            canvas.get_height() / 2.0,
            canvas.get_width() / 4.0,
            canvas.get_height() / 2.0,
        )
    }
}

fn get_rendering_gameboard_bounds(
    canvas: &Canvas,
    bwidth: isize,
    bheight: isize,
) -> (f64, f64, f64, f64, f64) {
    let bwidth = bwidth as f64;
    let bheight = bheight as f64;
    let (x, y, w, h) = get_gameboard_bounds(canvas);

    let cw =
        w / (bwidth * CHIP_SEPERATION / CHIP_DIAMETER + bwidth + CHIP_SEPERATION / CHIP_DIAMETER);
    let sw = CHIP_SEPERATION / CHIP_DIAMETER * cw;
    let tmw = cw + sw;
    let ch =
        h / (bheight * CHIP_SEPERATION / CHIP_DIAMETER + bheight + CHIP_SEPERATION / CHIP_DIAMETER);
    let sh = CHIP_SEPERATION / CHIP_DIAMETER * ch;
    let tmh = ch + sh;

    let mm = tmw.floor().min(tmh.floor());

    (x, y, bwidth * mm, bheight * mm, mm)
}
