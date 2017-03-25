use std::slice::{IterMut, Iter};
use renderer::Pixel;

/** Structure qui represente un "bout" d'image à rendre.
 * Elle est utilisée par le sampler pour stocker les samples, et par le filter
pour lire les samples et recomposer l'image finale. */
#[derive(Clone)]
pub struct Block {
    pixels: Vec<Pixel>,
    size_x: u32,
    size_y: u32,
    pos_x: u32,
    pos_y: u32,
}

impl Block {
    pub fn new(size_x: u32, size_y: u32, pos_x: u32, pos_y: u32) -> Self {
        let mut result = Block {
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

    pub fn position_x(&self) -> u32 {
        self.pos_x
    }

    pub fn position_y(&self) -> u32 {
        self.pos_y
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

    pub fn pixels_mut(&mut self) -> IterMut<Pixel> {
        self.pixels.iter_mut()
    }
}
