use minifb::{WindowOptions, Scale, Window};
use minifb_fonts::*;

fn main() {
    const WINDOW_WIDTH: usize = 400;
    const WINDOW_HEIGHT: usize = 200;

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut font_renderer = font4x6::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0x99_99_99);
    font_renderer.draw_text(&mut buffer, 10, 10, "Font-4x6");
    font_renderer.set_color(0xff_ff_ff);
    font_renderer.draw_text(&mut buffer, 10, 20, "Rusty claws play Dm on a xylophone, quickly bizarre, just for given vibes. ");

    let mut font_renderer = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0x99_99_99);
    font_renderer.draw_text(&mut buffer, 10, 40, "Font-5x8");
    font_renderer.set_color(0xff_ff_ff);
    font_renderer.draw_text(&mut buffer, 10, 50, "Rusty claws play Dm on a xylophone, quickly bizarre, just for given vibes. ");

    let mut font_renderer = font6x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0x99_99_99);
    font_renderer.draw_text(&mut buffer, 10, 70, "Font-6x8");
    font_renderer.set_color(0xff_ff_ff);
    font_renderer.draw_text(&mut buffer, 10, 80, "Rusty claws play Dm on a xylophone,\nquickly bizarre,\njust for given vibes. ");

    const RED_COLOR: u32 = 0x66_00_00;
    let mut color = RED_COLOR;
    let mut notice = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, color);

    // minifb window initialization
    let mut window = Window::new("minifb Font - ESC to exit", WINDOW_WIDTH, WINDOW_HEIGHT,
    WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    }).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        notice.set_color(color);
        notice.draw_text(&mut buffer, 10, 180, "Press ESC to exit");
        color += 0x02_00_00;
        if color > 0xFF_FF_FF {
            color = RED_COLOR
        }

        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}
