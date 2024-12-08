pub mod font4x6;
pub mod font5x8;
pub mod font6x8;

pub struct FbFont<const CHAR_WIDTH: usize> {
    pub char_w: usize,
    pub char_h: usize,
    pub char_bitmap_padding: usize,
    pub font_data: [[u8; CHAR_WIDTH]; 256],
}

pub struct FbFontRenderer<const CHAR_WIDTH: usize> {
    pub buf_width: usize,
    pub buf_height: usize,
    pub color: u32,
    pub font: FbFont<CHAR_WIDTH>,
}

impl<const CHAR_WIDTH: usize> FbFontRenderer<CHAR_WIDTH> {
    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn draw_text(&self, buf: &mut Vec<u32>, pos_x: usize, pos_y: usize, text: &str) {
        let font_data = &self.font.font_data;

        let mut current_x = pos_x;
        let mut current_y = pos_y;
        for ch in text.chars() {
            if ch == ' ' {
                current_x += self.font.char_w;
                continue;
            }
            if ch == '\n' {
                current_x = pos_x;
                current_y += self.font.char_h + 2;
                continue;
            }
            let mut index = ch as usize;

            if index >= font_data.len() {
                index = '?' as usize; // Character not in the font
            }

            let width = self.draw_char(buf, current_x, current_y, &font_data[index]);
            current_x += width;
        }
    }

    fn draw_char(&self, buffer: &mut Vec<u32>, pos_x: usize, pos_y: usize, char_data: &[u8]) -> usize {
        let font_height = self.font.char_h;
        let color = self.color;
        let char_pad = self.font.char_bitmap_padding;

        let mut start_col = 0;
        let mut end_col = self.font.char_w;

        // Find the start column with a non-empty pixel
        while start_col < self.font.char_w && char_data[start_col] == 0 {
            start_col += 1;
        }

        // Find the end column with a non-empty pixel
        while end_col > start_col && char_data[end_col - 1] == 0 {
            end_col -= 1;
        }

        for ix in start_col..end_col {
            // for iy in 8 - font_height..font_height + 2 {
            for iy in char_pad..font_height + char_pad {
                let pixel_value = (char_data[ix] >> iy) & 1;
                if pixel_value == 1 {
                    let pixel_pos_x = pos_x + ix - start_col;
                    let pixel_pos_y = pos_y + iy;
                    if pixel_pos_x < self.buf_width && pixel_pos_y < self.buf_height {
                        let buffer_index = pixel_pos_y * self.buf_width + pixel_pos_x;
                        buffer[buffer_index] = color;
                    }
                }
            }
        }
        return end_col - start_col + 1;
    }
}
