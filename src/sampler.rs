use render::RenderData;
use color::RGBA32;
use math::Vector2f;

/** Un sample, qui correspondra à un rayon émis dans la scène. L'ensemble
des samples est ensuite interpolé pour former l'image finale. */
pub struct Sample {
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
    fn create_samples(&self, data: &mut RenderData);
}

// Sampler dégueu
pub struct DefaultSampler {
    pub sample_rate: u32,
}

impl Sampler for DefaultSampler {
    fn create_samples(&self, data: &mut RenderData) {
        let (width, height) = data.dimensions();

        for y in 0..height {
            for x in 0..width {
                let mut pixel = data.get_pixel(x, y);

                for i in 0..self.sample_rate {
                    for j in 0..self.sample_rate {
                        pixel.add_sample(Sample::new((x as f32 +
                                                      i as f32 / self.sample_rate as f32) /
                                                     width as f32,
                                                     (y as f32 +
                                                      j as f32 / self.sample_rate as f32) /
                                                     height as f32));
                    }
                }
            }
        }
    }
}
