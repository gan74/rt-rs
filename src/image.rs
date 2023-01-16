use crate::color::*;


#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}


impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<Color>) -> Image {
        assert!((width as usize) * (height as usize) == pixels.len());

        Image {
            width: width,
            height: height,
            pixels: pixels,
        } 
    }

    pub fn pixel_count(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        self.pixels.as_slice()
    }

    pub fn pixel_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.pixels[self.pixel_index(x, y)]
    }

    pub fn pixel_at_clamped(&self, x: i32, y: i32) -> Color {
        let x = x.clamp(0, (self.width() - 1) as i32)as u32;
        let y = y.clamp(0, (self.height() - 1) as i32)as u32;
        self.pixel_at(x, y)
    }
}