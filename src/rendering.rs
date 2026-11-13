use image::{DynamicImage, Rgba};
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{AsciiAnimation, AsciiImage, FitDirection, FittingMode};

impl Widget for &AsciiImage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = calculate_size(self.fit, &self.image, &area);

        let image = self
            .image
            .thumbnail_exact(width as u32, height as u32)
            .to_rgba8();

        let max_luminance = image
            .pixels()
            .fold(0.0, |acc, pixel| get_luminance(pixel).max(acc));

        let mut lines: Vec<Line> = Vec::with_capacity(height as usize);
        let mut spans: Vec<Span> = Vec::with_capacity(width as usize);

        for (_, line, pixel) in image.enumerate_pixels() {
            if line as usize > lines.len() {
                lines.push(Line::from(spans.as_slice()));
                spans.clear();
            }

            let color = Color::Rgb(pixel[0], pixel[1], pixel[2]);

            // Normally this char_for_pixel has to be a char but because of the compatibility
            // reasons with unicode-segmentation It's implemented as a &str (WORKAROUND)
            let char_for_pixel = get_char_for_pixel(pixel, max_luminance, self.inverted);

            spans.push(Span::from(char_for_pixel).fg(color));
        }

        lines.push(Line::from(spans.as_slice())); // Last line after loop finishes.

        Paragraph::new(lines).render(area, buf);
    }
}

impl Widget for &mut AsciiAnimation {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.index >= self.frames.len() {
            self.index = 0;
        }
        let frame = self.frames.get(self.index);
        self.index += 1;

        let frame_image = match frame {
            Some(current_frame) => DynamicImage::ImageRgba8(current_frame.buffer().clone()),
            None => AsciiImage::default().image,
        };

        AsciiImage {
            image: frame_image,
            fit: self.fit.clone(),
            inverted: self.inverted,
        }
        .render(area, buf);
    }
}

pub fn calculate_size(fit: FittingMode, image: &DynamicImage, area: &Rect) -> (u16, u16) {
    match fit {
        FittingMode::None => (image.width() as u16, image.height() as u16),
        FittingMode::Exact(width, height) => (width, height),
        FittingMode::Fit(FitDirection::Width) => (
            area.width,
            (0.5 * image.height() as f32 * area.width as f32 / image.width() as f32) as u16,
        ),
        FittingMode::Fit(FitDirection::Height) => (
            (2.0 * image.width() as f32 * area.height as f32 / image.height() as f32) as u16,
            area.height,
        ),
        FittingMode::Fit(FitDirection::Both) => {
            let area_aspect_ratio = 0.5 * area.width as f32 / area.height as f32;
            let image_aspect_ratio = image.width() as f32 / image.height() as f32;

            match area_aspect_ratio <= image_aspect_ratio {
                true => (
                    area.width,
                    (0.5 * image.height() as f32 * area.width as f32 / image.width() as f32) as u16,
                ),
                false => (
                    (2.0 * image.width() as f32 * area.height as f32 / image.height() as f32)
                        as u16,
                    area.height,
                ),
            }
        }
        FittingMode::Stretch => (area.width, area.height),
    }
}

fn get_char_for_pixel(pixel: &Rgba<u8>, maximum: f64, inverted: bool) -> &str {
    const CHARSET: &[&str] = &[
        " ", ".", "`", "^", "\"", "\\", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
        "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n",
        "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p",
        "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "$", "@",
    ];

    let luminance = get_luminance(pixel) / maximum;

    // TODO: Use alpha channel to determine if pixel is transparent?
    let char_index = (luminance * (CHARSET.len() as f64 - 1.0)) as usize;

    CHARSET[match inverted {
        false => char_index,
        true => CHARSET.len() - 1 - char_index,
    }]
}

fn get_luminance(pixel: &Rgba<u8>) -> f64 {
    const RGB_MAX: f64 = u8::MAX as f64;
    /// Data from [Rec. 709 color spec](https://en.wikipedia.org/wiki/Rec._709).
    const COLOR_CHANNEL_WEIGHTS: [f64; 4] = [0.2126, 0.7152, 0.0722, 0.0];

    let mut luminance = pixel[0] as f64 * COLOR_CHANNEL_WEIGHTS[0];
    luminance += pixel[1] as f64 * COLOR_CHANNEL_WEIGHTS[1];
    luminance += pixel[2] as f64 * COLOR_CHANNEL_WEIGHTS[2];

    luminance / RGB_MAX
}
