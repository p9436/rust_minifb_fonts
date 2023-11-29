use minifb::{WindowOptions, Scale, Window};
use minifb_fonts::*;

fn main() {
    const WINDOW_WIDTH: usize = 400;
    const WINDOW_HEIGHT: usize = 200;

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // ASCII table
    let ascii_string: String = (0..=0xFF)
        .map(|x| char::from(x as u8))
        .enumerate()
        .map(|(i, c)| if i > 0 && i % 16 == 0 { format!("\n{}", c) } else { c.to_string() })
        .collect();
    let ascii_str: &str = ascii_string.as_str();

    let font_renderer = font4x6::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0xFF_FF_FF);
    font_renderer.draw_text(&mut buffer, 20, 10, ascii_str);

    let font_renderer = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0xFF_FF_FF);
    font_renderer.draw_text(&mut buffer, 120, 10, ascii_str);

    let font_renderer = font6x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0xFF_FF_FF);
    font_renderer.draw_text(&mut buffer, 240, 10, ascii_str);

    // minifb window initialization
    let mut window = Window::new("minifb Font Example ", WINDOW_WIDTH, WINDOW_HEIGHT,
    WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    }).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}
