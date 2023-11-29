# minifb_fonts
Simple Addon for the [minifb](https://github.com/emoon/rust_minifb/tree/master) crate that enables drawing text using bitmap fonts.

![Example](https://github.com/p9436/rust_minifb_fonts/blob/main/assets/screenshot.png?raw=true)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
minifb_fonts = "0.1"
```

## Example

```rust
use minifb::{WindowOptions, Scale, Window};
use minifb_fonts::*;

fn main() {
    const WINDOW_WIDTH: usize = 400;
    const WINDOW_HEIGHT: usize = 200;

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let text = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, color);
    text.draw_text(&mut buffer, 10, 20, "Hello World!");
    text.set_color(0xff_00_00);
    text.draw_text(&mut buffer, 10, 180, "Press ESC to exit");

    // minifb window initialization
    let mut window = Window::new("minifb Font - ESC to exit", WINDOW_WIDTH, WINDOW_HEIGHT,
    WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    }).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}
```

## Examples

```
cargo build
cargo run --example draw_text
```

This will run [draw text](./examples/draw_text.rs) example.

Find more exaples [here](./examples/)

## License

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
