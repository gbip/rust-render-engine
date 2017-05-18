pub mod samplers;

use color_float::LinearColor;
use math::Vector2f;
use sampler::samplers::{DefaultSampler, HaltonSampler};

/** Un sample, qui correspondra à un rayon émis dans la scène. L'ensemble
des samples est ensuite interpolé pour former l'image finale. */
#[derive(Clone,Debug)]
pub struct Sample {
    /** La position relative du rayon par rapport au centre de son pixel */
    position: Vector2f,
    pub color: LinearColor,
}

impl Sample {
    pub fn new(x: f32, y: f32) -> Self {
        Sample {
            position: Vector2f { x: x, y: y },
            color: LinearColor::default(),
        }
    }

    pub fn position(&self) -> Vector2f {
        self.position
    }
}

// Une section rectangulaire samplable. Peut être un bloc ou une lumière surfacique.
pub trait SamplableArea {
    fn dimensions(&self) -> (f32, f32);
    fn offset(&self) -> Vector2f;
    fn pixel_width(&self) -> u32 {
        1u32
    }
    fn pixel_height(&self) -> u32 {
        1u32
    }
    fn add_sample(&mut self, sample: Sample);
}

pub trait Sampler {
    fn create_samples(&self, area: &mut SamplableArea) {
        let offset = area.offset();
        let (width, height) = area.dimensions();
        let sub_x = width / area.pixel_width() as f32;
        let sub_y = height / area.pixel_height() as f32;

        for y in 0..area.pixel_height() {
            for x in 0..area.pixel_width() {
                let distrib = self.get_sample_distribution();

                for point in distrib {
                    let sample_pos = Vector2f {
                        x: (x as f32 + point.x) * sub_x,
                        y: (y as f32 + point.y) * sub_y,
                    } + offset;

                    area.add_sample(Sample::new(sample_pos.x, sample_pos.y));
                }
            }
        }
    }

    fn get_sample_distribution(&self) -> Vec<Vector2f> {
        vec![]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SamplerFactory {
    HaltonSampler { subdivision_sampling: u32 },
    DefaultSampler { subdivision_sampling: u32 },
}

impl SamplerFactory {
    pub fn create_sampler(&self) -> Box<Sampler> {
        match *self {
            SamplerFactory::HaltonSampler { subdivision_sampling } => {
                Box::new(HaltonSampler::new(subdivision_sampling))
            }
            SamplerFactory::DefaultSampler { subdivision_sampling } => {
                Box::new(DefaultSampler::new(subdivision_sampling))
            }
        }
    }
}
