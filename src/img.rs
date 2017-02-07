use image;
use std::vec;
use std::path::Path;
use color::RGBA32;

pub trait Pixel: Copy {
    fn to_rgb_pixel(&self) -> (u8, u8, u8);
    fn to_rgba_pixel(&self) -> (u8, u8, u8, u8);
}

pub struct Image<T: Pixel> {
    width: usize,
    height: usize,
    pub pixels: Vec<T>,
}

impl<T: Pixel> Image<T> {
    /** Convertit une liste de lignes de pixels en image exportable.  */
    pub fn from_vec_vec(vec_vec: &Vec<Vec<T>>) -> Image<T> {
        let mut pixels: Vec<T> = vec![];

        for vec in vec_vec {
            for t in vec {
                pixels.push(*t);
            }
        }

        Image {
            width: vec_vec[0].len(),
            height: vec_vec.len(),
            pixels: pixels,
        }
    }

    pub fn write_to_file(&self, pathname: &str) {
        let mut buffer = vec![];

        for color in &self.pixels {
            let (r, g, b) = color.to_rgb_pixel();
            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
        }

        image::save_buffer(&Path::new(pathname),
                           buffer.as_slice(),
                           self.width as u32,
                           self.height as u32,
                           image::RGB(8))
            .unwrap();
    }
}

impl Image<RGBA32> {
    pub fn new(sizex: usize, sizey: usize) -> Self {
        let px: Vec<RGBA32> = vec![RGBA32::new_black(); sizex * sizey];
        Image {
            width: sizex,
            height: sizey,
            pixels: px,
        }
    }
}
