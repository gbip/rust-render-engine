use image;
use image::GenericImage;
// Conflit avec notre trait pixel...
use image::Pixel as ImgPixel;
use std::vec;
use std::path::Path;
use color::{RGBA32, RGBA8};
use std::fs::File;

pub trait Pixel: Copy {
    fn to_rgb_pixel(&self) -> (u8, u8, u8);
    fn to_rgba_pixel(&self) -> (u8, u8, u8, u8);
}

#[derive(Clone)]
pub struct Image<T: Pixel> {
    width: usize,
    height: usize,
    pub pixels: Vec<Vec<T>>,
}

impl<T: Pixel> Image<T> {
    /** Convertit une liste de lignes de pixels en image exportable.  */
    pub fn from_vec_vec(vec_vec: &[Vec<T>]) -> Image<T> {
        Image {
            width: vec_vec.len(),
            height: vec_vec[0].len(),
            pixels: vec_vec.to_vec(),
        }
    }

    pub fn write_to_file(&self, pathname: &str) {
        let mut buffer = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (col, line, pixel) in buffer.enumerate_pixels_mut() {
            let render_pix = self.pixels[col as usize][line as usize].to_rgb_pixel();
            *pixel = image::Rgb::from_channels(render_pix.0, render_pix.1, render_pix.2, 0);
        }

        let file_output = &mut File::create(&Path::new(pathname)).unwrap();
        image::ImageRgb8(buffer)
            .save(file_output, image::PNG)
            .expect("Error while saving file");
    }

    pub fn get_pixel_at(&self, x: u32, y: u32) -> T {
        self.pixels[x as usize][y as usize]
    }

    pub fn write_pixel_at(&mut self, x: u32, y: u32, px: T) {
        self.pixels[x as usize][y as usize] = px;
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    /** Permet de superposer une sous image en (pos_x,pos_y) sur une autre image */
    pub fn superpose_sub_image(&mut self, other: Image<T>, pos_x: u32, pos_y: u32) {
        for col in pos_x..(pos_x + other.width as u32) {
            for line in pos_y..(pos_y + other.height as u32) {
                let px = other.get_pixel_at(col - pos_x, line - pos_y);
                self.write_pixel_at(col, line, px);
            }
        }
    }
}

impl Image<RGBA8> {
    pub fn read_from_file(pathname: &str) -> Image<RGBA8> {

        let img = image::open(&Path::new(pathname)).unwrap();
        let dims = img.dimensions();

        let width = dims.0;
        let height = dims.1;
        let mut result = Image::<RGBA8> {
            width: width as usize,
            height: height as usize,
            pixels: vec![],
        };

        for x in 0..width {
            let mut col: Vec<RGBA8> = vec![];
            for y in 0..height {
                let pix = img.get_pixel(x, y);
                col.push(RGBA8::new(&pix.data[0], &pix.data[1], &pix.data[2], &pix.data[3]));
            }
            result.pixels.push(col);
        }

        result
    }
}

impl Image<RGBA32> {
    pub fn new(sizex: usize, sizey: usize) -> Self {
        let px: Vec<Vec<RGBA32>> = vec![vec![RGBA32::new_black();sizey]; sizex];
        Image {
            width: sizex,
            height: sizey,
            pixels: px,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_image_superposition() {
        let mut image1: Image<RGBA32> = Image::new(40, 40);
        let mut image2: Image<RGBA32> = Image::new(10, 10);

        for x in 0..image2.width() {
            for y in 0..image2.height() {
                image2.write_pixel_at(x, y, RGBA32::new_white());
            }
        }
        assert_eq!(image2.get_pixel_at(5, 5), RGBA32::new_white());
        println!("{}x{}", image1.width, image1.height);
        image1.superpose_sub_image(image2, 30, 30);
        assert_eq!(image1.get_pixel_at(39, 39), RGBA32::new_white());
    }
}
