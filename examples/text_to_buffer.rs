use minifb::{WindowOptions, Scale, Window};
use minifb_fonts::*;

const WINDOW_WIDTH: usize = 320;
const WINDOW_HEIGHT: usize = 200;

const FONT_BUF_W: usize = 200;
const FONT_BUF_H: usize = 10;

fn main() {
    // Draw text to intermediate buffer
    let mut text_buffer: Vec<u32> = vec![0; FONT_BUF_W * FONT_BUF_H];

    let font_renderer = font5x8::new_renderer(FONT_BUF_W, FONT_BUF_H, 0xFF_FF_FF);
    font_renderer.draw_text(&mut text_buffer, 0, 0, "Hello world! I'm a font library for minifb");


    // Initialize window buffer
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let draw_pos_x = 30;
    let draw_pos_y = 30;

    // Copy text buffer to the window frame buffer
    for iy in 0..FONT_BUF_H {
        for ix in 0..FONT_BUF_W {
            let main_buf_x = ix + draw_pos_x;
            let main_buf_y = iy + draw_pos_y;
            window_buffer[main_buf_x + main_buf_y * WINDOW_WIDTH] = text_buffer[iy * FONT_BUF_W + ix]
        }
    }

    // minifb window initialization
    let mut window = Window::new("minifb Font Example ", WINDOW_WIDTH, WINDOW_HEIGHT,
    WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    }).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}
