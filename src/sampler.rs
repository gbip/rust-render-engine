use renderer::block::Block;
use color::RGBA32;
use math::Vector2f;

/** Un sample, qui correspondra à un rayon émis dans la scène. L'ensemble
des samples est ensuite interpolé pour former l'image finale. */
#[derive(Clone)]
pub struct Sample {
    /** La position relative du rayon par rapport au centre de son pixel */
    position: Vector2f,
    pub color: RGBA32,
}

impl Sample {
    pub fn new(x: f32, y: f32) -> Self {
        Sample {
            position: Vector2f { x: x, y: y },
            color: RGBA32::new_black(),
        }
    }

    pub fn position(&self) -> Vector2f {
        self.position
    }
}

pub trait Sampler {
    fn create_samples(&self, block: &mut Block, res_x: u32, res_y: u32);
}

/** Sampler avec une distribution d'échantillon uniforme à travers les pixels*/
pub struct DefaultSampler {
    pub sample_rate: u32,
}

impl Sampler for DefaultSampler {
    fn create_samples(&self, block: &mut Block, res_x: u32, res_y: u32) {
        let pos_x = block.position_x();
        let pos_y = block.position_y();


        let block_width = pos_x + block.dimensions().0;
        let block_height = pos_y + block.dimensions().1;

        //let (width, height): (u32, u32) = block.dimensions();
        for y in pos_y..block_height {
            for x in pos_x..block_width {
                let mut pixel = block.get_pixel(x - pos_x, y - pos_y);
                for i in 0..self.sample_rate {
                    for j in 0..self.sample_rate {
                        pixel.add_sample(Sample::new((x as f32 +
                                                      i as f32 / self.sample_rate as f32) /
                                                     res_x as f32,
                                                     (y as f32 +
                                                      j as f32 / self.sample_rate as f32) /
                                                     res_y as f32));
                    }
                }
            }
        }
    }
}
