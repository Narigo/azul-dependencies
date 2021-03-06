//! Andrew is a crate for drawing objects
#![warn(missing_docs)]
extern crate line_drawing;
extern crate rusttype;
extern crate walkdir;
extern crate xdg;
extern crate xml;

#[macro_use]
extern crate bitflags;

/// A module that contains functions and objects relating to lines
pub mod line;
/// A module that contains functions and objects relating to shapes
pub mod shapes;
/// A module that contains functions and objects relating to text
pub mod text;

/// The Drawable trait allows object to be drawn to a buffer or canvas
pub trait Drawable {
    /// A function that draws the object to a canvas
    fn draw(&self, canvas: &mut Canvas);
}

/// The canvas object acts as a wrapper around a buffer, providing information and functions
/// for drawing
pub struct Canvas<'a> {
    /// A buffer for the canvas to draw to
    pub buffer: &'a mut [u8],
    /// The width in pixels of the canvas
    pub width: usize,
    /// The height in pixels of the canvas
    pub height: usize,
    /// The number of bytes between each line of pixels on the canvas
    pub stride: usize,
    /// The number of bytes contained in each pixel
    pub pixel_size: usize,
}

impl<'a> Canvas<'a> {
    /// Creates a new canvas object
    pub fn new(buffer: &'a mut [u8], width: usize, height: usize, stride: usize) -> Canvas<'a> {
        assert!(
            stride % width == 0,
            "Incorrect Dimensions - Stride is not a multiple of width"
        );
        assert!(buffer.len() == stride * height);
        let pixel_size = stride / width;
        Canvas {
            buffer,
            width,
            height,
            stride,
            pixel_size,
        }
    }

    /// Draws an object that implements the Drawable trait to the buffer
    pub fn draw<D: Drawable>(&mut self, drawable: &D) {
        drawable.draw(self);
    }

    /// Draws a pixel at the x and y coordinate
    pub fn draw_point(&mut self, x: usize, y: usize, color: [u8; 4]) {
        for c in 0..3 {
            let alpha = f32::from(color[3]) / 255.0;
            let color_diff = (color[c] as isize
                - self.buffer[self.stride * y + self.pixel_size * x + c] as isize)
                as f32
                * alpha;
            let new_color = (f32::from(self.buffer[self.stride * y + self.pixel_size * x + c])
                + color_diff) as u8;
            self.buffer[self.stride * y + self.pixel_size * x + c] = new_color as u8;
        }
        self.buffer[self.stride * y + self.pixel_size * x + 3] = 255 as u8;
    }

    /// Clears the entire canvas buffer by zeroing it
    pub fn clear(&mut self) {
        for i in 0..self.width * self.height * 4 {
            self.buffer[i] = 0x00;
        }
    }
}
