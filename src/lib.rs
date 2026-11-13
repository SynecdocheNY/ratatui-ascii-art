use image::{AnimationDecoder, DynamicImage, ImageError, ImageResult};
use ratatui::style::{Style, Styled};
use std::{fs::read, io::Cursor, path::Path};

use rendering::calculate_size;

mod rendering;

const RATATUI_LOGO_BYTES: &'static [u8] = include_bytes!("ratatui-logo.png");
const RATATUI_GIF_BYTES: &'static [u8] = include_bytes!("rat.gif");

/// Builder and constructor methods common to the `AsciiImage` and `AsciiAnimation` widgets.
pub trait AsciiBuilder: Sized {
    fn new<P: AsRef<Path>>(image_path: P) -> ImageResult<Self>;
    fn open<P: AsRef<Path>>(self, image_path: P) -> ImageResult<Self>;

    fn fitting_mode(self, mode: FittingMode) -> Self;
    fn fit(self) -> Self;
    fn stretch(self) -> Self;

    fn unstyled(self) -> Self;
    fn inverted(self) -> Self;
    fn not_inverted(self) -> Self;

    /// The size at which an ASCII widget will be rendered after the resizing algorithm runs. This method can be used for centering.
    fn fitted_size(&self, area: &ratatui::layout::Rect) -> (u16, u16);
}

/// A static image rendered as ASCII art.
pub struct AsciiImage {
    pub image: DynamicImage,
    fitting_mode: FittingMode,
    inverted: bool,
    style: Option<Style>,
}

impl AsciiBuilder for AsciiImage {
    fn new<P: AsRef<Path>>(image_path: P) -> ImageResult<Self> {
        Ok(AsciiImage::default().open(image_path)?)
    }

    fn open<P: AsRef<Path>>(mut self, image_path: P) -> ImageResult<Self> {
        self.image = image::open(image_path)?;

        Ok(self)
    }

    fn fitting_mode(mut self, mode: FittingMode) -> Self {
        self.fitting_mode = mode;
        self
    }

    fn fit(mut self) -> Self {
        self.fitting_mode = FittingMode::Fit(FitDirection::Both);
        self
    }

    fn stretch(mut self) -> Self {
        self.fitting_mode = FittingMode::Stretch;
        self
    }

    fn unstyled(mut self) -> Self {
        self.style = None;
        self
    }

    fn inverted(mut self) -> Self {
        self.inverted = true;
        self
    }

    fn not_inverted(mut self) -> Self {
        self.inverted = false;
        self
    }

    fn fitted_size(&self, area: &ratatui::layout::Rect) -> (u16, u16) {
        calculate_size(self.fitting_mode, &self.image, area)
    }
}

impl Styled for AsciiImage {
    type Item = Self;

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    fn style(&self) -> Style {
        self.style.unwrap_or(Style::default())
    }
}

impl Default for AsciiImage {
    fn default() -> Self {
        AsciiImage {
            image: image::load_from_memory(RATATUI_LOGO_BYTES)
                .expect("Failed to load default image from binary."),
            fitting_mode: FittingMode::default(),
            inverted: false,
            style: None,
        }
    }
}

/// An animated GIF rendered as ASCII art.
pub struct AsciiAnimation {
    pub frames: Vec<image::Frame>,
    pub index: usize,
    fitting_mode: FittingMode,
    inverted: bool,
    style: Option<Style>,
}

impl AsciiBuilder for AsciiAnimation {
    fn new<P: AsRef<Path>>(image_path: P) -> ImageResult<Self> {
        Ok(AsciiAnimation::default().open(image_path)?)
    }

    fn open<P: AsRef<Path>>(mut self, image_path: P) -> ImageResult<Self> {
        let file = read(image_path).map_err(|io_error| ImageError::IoError(io_error))?;
        let gif_decoder = image::codecs::gif::GifDecoder::new(Cursor::new(file))?;

        self.frames = gif_decoder.into_frames().collect_frames()?;
        self.index = 0;

        Ok(self)
    }

    fn fitting_mode(mut self, mode: FittingMode) -> Self {
        self.fitting_mode = mode;
        self
    }

    fn fit(mut self) -> Self {
        self.fitting_mode = FittingMode::Fit(FitDirection::Both);
        self
    }

    fn stretch(mut self) -> Self {
        self.fitting_mode = FittingMode::Stretch;
        self
    }

    fn unstyled(mut self) -> Self {
        self.style = None;
        self
    }

    fn inverted(mut self) -> Self {
        self.inverted = true;
        self
    }

    fn not_inverted(mut self) -> Self {
        self.inverted = false;
        self
    }

    fn fitted_size(&self, area: &ratatui::layout::Rect) -> (u16, u16) {
        let frame_image =
            DynamicImage::ImageRgba8(self.frames.get(self.index).unwrap().buffer().clone());

        calculate_size(self.fitting_mode, &frame_image, area)
    }
}

impl Styled for AsciiAnimation {
    type Item = Self;

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    fn style(&self) -> Style {
        self.style.unwrap_or(Style::default())
    }
}

impl Default for AsciiAnimation {
    fn default() -> Self {
        let gif_decoder = image::codecs::gif::GifDecoder::new(Cursor::new(RATATUI_GIF_BYTES))
            .expect("Failed to load default GIF from binary.");

        AsciiAnimation {
            frames: gif_decoder
                .into_frames()
                .collect_frames()
                .expect("Failed to decode default GIF from binary."),
            index: 0,
            fitting_mode: FittingMode::default(),
            inverted: false,
            style: None,
        }
    }
}

/// How an image should fill its bounding area.
#[derive(Default, Copy, Clone)]
pub enum FittingMode {
    None,
    /// The image will be rendered at the provided dimensions.
    ///
    /// Too small a container will clip the image; too large will create empty space around it.
    /// Note that the units are "number of terminal characters," *not* pixels.
    Exact(u16, u16),
    /// The image resizes itself, but keeps its aspect ratio.
    ///
    /// A misfit container will create empty space or .
    Fit(FitDirection),
    /// The image warps to fill the available area.
    ///
    /// A misfit container will create distortion.
    #[default]
    Stretch,
}

#[derive(Copy, Clone)]
pub enum FitDirection {
    Width,
    Height,
    Both,
}
