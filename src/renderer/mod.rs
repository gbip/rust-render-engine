use img::{Image, RGBAPixel};
use std::slice::Iter;
use std::collections::HashMap;
use sampler::Sample;

pub mod render;
pub mod block;

/** Type representant un registre de texture */
pub type TextureRegister = HashMap<String, Image<RGBAPixel>>;

/** Structure utilisée par le sampler pour stocker les samples, et par le filter
pour les lire et recomposer l'image finale
TODO rename, déplacer ?*/
pub struct RenderData {
    pixels: Vec<Pixel>,
    size_x: u32,
    size_y: u32,
    pos_x: u32,
    pos_y: u32,
}

impl RenderData {
    pub fn new(size_x: u32, size_y: u32, pos_x: u32, pos_y: u32) -> Self {
        let mut result = RenderData {
            pixels: vec![],
            size_x: size_x,
            size_y: size_y,
            pos_x: pos_x,
            pos_y: pos_y,
        };

        // Ajout des pixels (l'ordre des for est important)
        for y in 0..size_y {
            for x in 0..size_x {
                result.pixels.push(Pixel::new(x, y));
            }
        }

        result
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.size_x, self.size_y)
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.pixels[(x + y * self.size_x) as usize]
    }

    pub fn pixels(&self) -> Iter<Pixel> {
        self.pixels.iter()
    }
}

/** Représente un pixel avec des Sample dedans. */
#[derive(Clone,Debug)]
pub struct Pixel {
    x: u32,
    y: u32,
    samples: Vec<Sample>,
}

impl Pixel {
    pub fn new(x: u32, y: u32) -> Pixel {
        Pixel {
            x: x,
            y: y,
            samples: vec![],
        }
    }

    pub fn add_sample(&mut self, sample: Sample) {
        self.samples.push(sample);
    }
    pub fn samples(&self) -> Iter<Sample> {
        self.samples.iter()
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
