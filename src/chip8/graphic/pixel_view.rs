use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

use super::pixel::Pixel;

pub trait PixelView {
    fn get_pixel_unchecked(&self, x: usize, y: usize) -> Pixel;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub struct BitSlicePixelView<'a> {
    slice: &'a BitSlice<u8, Msb0>,
    width: usize,
    height: usize,
}

impl<'a> BitSlicePixelView<'a> {
    pub fn new(slice: &'a BitSlice<u8, Msb0>, width: usize, height: usize) -> BitSlicePixelView {
        BitSlicePixelView {
            slice,
            width,
            height,
        }
    }

    pub fn new_from_byte_slice(slice: &'a [u8], width: usize, height: usize) -> BitSlicePixelView {
        BitSlicePixelView {
            slice: slice.view_bits::<Msb0>(),
            width,
            height,
        }
    }
}

impl<'a> PixelView for BitSlicePixelView<'a> {
    fn get_pixel_unchecked(&self, x: usize, y: usize) -> Pixel {
        self.slice[y * self.width + x].into()
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}
