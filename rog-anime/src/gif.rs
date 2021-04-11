use glam::Vec2;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, path::Path, time::Duration};

use crate::{error::AnimeError, AnimeDataBuffer, AnimeDiagonal, AnimeImage, Pixel};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnimeFrame {
    /// Precomputed data for the frame. This can be transferred directly to the
    /// the `asusd` daemon over dbus or converted to USB packet with `AnimePacketType::from(buffer)`
    data: AnimeDataBuffer,
    delay: Duration,
}

impl AnimeFrame {
    /// Get the inner data buffer of the gif frame
    #[inline]
    pub fn frame(&self) -> &AnimeDataBuffer {
        &self.data
    }

    /// Get the `Duration` of the delay for this frame
    #[inline]
    pub fn delay(&self) -> Duration {
        self.delay
    }
}

/// Defines the time or animation cycle count to use for a gif
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimTime {
    /// Time in milliseconds for animation to run
    Time(Duration),
    /// How many full animation loops to run
    Cycles(u32),
    /// Run for infinite time
    Infinite,
}

impl Default for AnimTime {
    #[inline]
    fn default() -> Self {
        Self::Infinite
    }
}

/// A gif animation. This is a collection of frames from the gif, and a duration
/// that the animation should be shown for.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnimeGif(Vec<AnimeFrame>, AnimTime);

impl AnimeGif {
    /// Create an animation using the 74x36 ASUS gif format
    #[inline]
    pub fn create_diagonal_gif(
        file_name: &Path,
        duration: AnimTime,
        brightness: f32,
    ) -> Result<Self, AnimeError> {
        let mut matrix = AnimeDiagonal::new(None);

        let mut decoder = gif::DecodeOptions::new();
        // Configure the decoder such that it will expand the image to RGBA.
        decoder.set_color_output(gif::ColorOutput::RGBA);
        // Read the file header
        let file = File::open(file_name)?;
        let mut decoder = decoder.read_info(file)?;

        let mut frames = Vec::with_capacity(decoder.buffer_size());
        while let Some(frame) = decoder.read_next_frame()? {
            let wait = frame.delay * 10;
            if matches!(frame.dispose, gif::DisposalMethod::Background) {
                frames = Vec::new();
            }
            for (y, row) in frame.buffer.chunks(frame.width as usize * 4).enumerate() {
                for (x, px) in row.chunks(4).enumerate() {
                    if px[3] != 255 {
                        // should be t but not in some gifs? What, ASUS, what?
                        continue;
                    }
                    matrix.get_mut()[y + frame.top as usize][x + frame.left as usize] =
                        (px[0] as f32 * brightness) as u8;
                }
            }

            frames.push(AnimeFrame {
                data: <AnimeDataBuffer>::from(&matrix),
                delay: Duration::from_millis(wait as u64),
            });
        }
        Ok(Self(frames, duration))
    }

    /// Create an animation using a gif of any size. This method must precompute the
    /// result.
    #[inline]
    pub fn create_png_gif(
        file_name: &Path,
        scale: f32,
        angle: f32,
        translation: Vec2,
        duration: AnimTime,
        brightness: f32,
    ) -> Result<Self, AnimeError> {
        let mut frames = Vec::new();

        let mut decoder = gif::DecodeOptions::new();
        // Configure the decoder such that it will expand the image to RGBA.
        decoder.set_color_output(gif::ColorOutput::RGBA);
        // Read the file header
        let file = File::open(file_name)?;
        let mut decoder = decoder.read_info(file)?;

        let height = decoder.height();
        let width = decoder.width();
        let pixels: Vec<Pixel> =
            vec![Pixel::default(); (decoder.width() as u32 * decoder.height() as u32) as usize];
        let mut image = AnimeImage::new(
            Vec2::new(scale, scale),
            angle,
            translation,
            brightness,
            pixels,
            decoder.width() as u32,
        );

        while let Some(frame) = decoder.read_next_frame()? {
            let wait = frame.delay * 10;
            if matches!(frame.dispose, gif::DisposalMethod::Background) {
                let pixels: Vec<Pixel> =
                    vec![Pixel::default(); (width as u32 * height as u32) as usize];
                image = AnimeImage::new(
                    Vec2::new(scale, scale),
                    angle,
                    translation,
                    brightness,
                    pixels,
                    width as u32,
                );
            }
            for (y, row) in frame.buffer.chunks(frame.width as usize * 4).enumerate() {
                for (x, px) in row.chunks(4).enumerate() {
                    if px[3] != 255 {
                        // should be t but not in some gifs? What, ASUS, what?
                        continue;
                    }
                    let pos =
                        (x + frame.left as usize) + ((y + frame.top as usize) * width as usize);
                    image.get_mut()[pos] = Pixel {
                        color: ((px[0] as u32 + px[1] as u32 + px[2] as u32) / 3),
                        alpha: 1.0,
                    };
                }
            }
            image.update();

            frames.push(AnimeFrame {
                data: <AnimeDataBuffer>::from(&image),
                delay: Duration::from_millis(wait as u64),
            });
        }
        Ok(Self(frames, duration))
    }

    /// Get a slice of the frames this gif has
    #[inline]
    pub fn frames(&self) -> &[AnimeFrame] {
        &self.0
    }

    /// Get the time/count for this gif
    #[inline]
    pub fn duration(&self) -> AnimTime {
        self.1
    }
}