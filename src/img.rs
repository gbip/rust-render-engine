use image;
use image::{GenericImage};
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
        println!("Image is : {} x {}", self.width, self.height);
        let mut buffer = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (col, line, pixel) in buffer.enumerate_pixels_mut() {
            let render_pix = self.pixels[col as usize][line as usize].to_rgb_pixel();
            *pixel = image::Rgb::from_channels(render_pix.0, render_pix.1, render_pix.2, 0);
        }

        let file_output = &mut File::create(&Path::new(pathname)).unwrap();
        println!("File res is : {} x {} ", buffer.width(), buffer.height());
        image::ImageRgb8(buffer).save(file_output, image::PNG).expect("Error while saving file");
    }
}

impl Image<RGBA8> {
    pub fn read_from_file(pathname: &str) -> Image<RGBA8> {
        let img = image::open(&Path::new(pathname)).unwrap();
        let dims = img.dimensions();
        println!("Image at {} : resolution is {:?}", pathname, dims);

        let width = dims.0 as usize;
        let height = dims.1 as usize;
        let mut result = Image::<RGBA8> {width : width, height : height, pixels : vec![]};

        for x in 0..dims.0 {
            let mut line : Vec<RGBA8> = vec![];
            for y in 0..dims.1 {
                let pix = img.get_pixel(x, y);
                line.push(RGBA8::new(&pix.data[0], &pix.data[1], &pix.data[2], &pix.data[3]));
            }
            result.pixels.push(line);
        }

        result
    }
}

impl Image<RGBA32> {
    pub fn new(sizex: usize, sizey: usize) -> Self {
        let px: Vec<Vec<RGBA32>> = vec![vec![RGBA32::new_black();sizex]; sizey];
        Image {
            width: sizex,
            height: sizey,
            pixels: px,
        }
    }
}
