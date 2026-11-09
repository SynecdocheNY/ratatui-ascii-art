use image::{DynamicImage, ImageError, Rgba};
use ratatui::{prelude::*, widgets::Paragraph};

const RATATUI_LOGO_BYTES: &'static [u8] = include_bytes!("ratatui-logo.png");

pub struct AsciiImage {
    pub image: DynamicImage,
    pub fit: AsciiImageFitting,
    pub inverted: bool,
}

impl AsciiImage {
    pub fn open<P: AsRef<std::path::Path>>(mut self, image_path: P) -> Result<Self, ImageError> {
        self.image = image::open(image_path)?;

        Ok(self)
    }

    pub fn fitting(mut self, fitting: AsciiImageFitting) -> Self {
        self.fit = fitting;

        self
    }

    pub fn inverted(mut self) -> Self {
        self.inverted = true;

        self
    }
}

impl Default for AsciiImage {
    fn default() -> Self {
        AsciiImage {
            image: image::load_from_memory(RATATUI_LOGO_BYTES)
                .expect("Failed to load default image from binary."),
            fit: AsciiImageFitting::default(),
            inverted: false,
        }
    }
}

/// How an image should fill its bounding area.
#[derive(Default)]
pub enum AsciiImageFitting {
    /// The image will be rendered at the provided dimensions. Too small a container will clip the image; too large will create empty space around it.
    ///
    /// Note that the values are in "number of terminal characters," *not* pixels.
    Exact(u16, u16),
    /// The image resizes itself, but keeps its aspect ratio. A misfit container will create empty space.
    Fit,
    /// The image resizes itself, warping to fill the available area. A misfit container will create stretching effects.
    #[default]
    Stretch,
}

impl Widget for &AsciiImage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height): (u16, u16) = match self.fit {
            AsciiImageFitting::Exact(width, height) => (width, height),
            AsciiImageFitting::Fit => {
                todo!()
            }
            AsciiImageFitting::Stretch => (area.width, area.height),
        };

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
