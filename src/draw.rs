use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn cord(loc: i32) -> f64 {

    (loc as f64) * BLOCK_SIZE

}

pub fn cord_to_u32(loc: i32) -> u32 {
    cord(loc) as u32
}

pub fn draw_block(color: Color, x:i32, y:i32, con: &Context, g: &mut G2d) {
    let gui_x = cord(x);
    let gui_y = cord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE,],
        con.transform,
        g
    );
}

pub fn draw_rectangle(color: Color,
                      x: i32,
                      y: i32,
                      height: i32,
                      width: i32,
                      con: &Context,
                      g: &mut G2d
)
{
    let gui_x = cord(x);
    let gui_y = cord(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (height as f64),
            BLOCK_SIZE * (width as f64),
        ],
        con.transform,
        g,
    );
}
