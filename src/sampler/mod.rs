pub mod samplers;

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

        for y in 0..area.pixel_width() {
            for x in 0..area.pixel_height() {
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