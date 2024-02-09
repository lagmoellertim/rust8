use super::pixel::Pixel;
use crate::chip8::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Screen {
    framebuffer: [[Pixel; SCREEN_WIDTH]; SCREEN_HEIGHT],
    content_updated: bool,
}

pub enum XorPixelErased {
    Yes,
    No,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            framebuffer: [[Pixel::Off; SCREEN_WIDTH]; SCREEN_HEIGHT],
            content_updated: false,
        }
    }
}

impl Screen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.framebuffer = [[Pixel::Off; SCREEN_WIDTH]; SCREEN_HEIGHT]
    }

    pub fn xor_pixel_wrapped_position(
        &mut self,
        x: usize,
        y: usize,
        pixel: Pixel,
    ) -> XorPixelErased {
        let target_pixel_ref = &mut self.framebuffer[y % SCREEN_HEIGHT][x % SCREEN_WIDTH];

        let old_value = *target_pixel_ref;
        let new_value = *target_pixel_ref ^ pixel;

        *target_pixel_ref = new_value;

        self.content_updated = true;

        match (old_value, new_value) {
            (Pixel::On, Pixel::Off) => XorPixelErased::Yes,
            _ => XorPixelErased::No,
        }
    }

    pub fn has_content_updated(&self) -> bool {
        self.content_updated
    }

    pub fn reset_content_updated(&mut self) {
        self.content_updated = false;
    }

    pub fn framebuffer(&self) -> &[[Pixel; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        &self.framebuffer
    }
}
