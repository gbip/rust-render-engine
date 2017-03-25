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
    fn add_sample(&mut self, sample : Sample);
}

pub trait Sampler {
    fn create_samples(&self, area: &mut SamplableArea);
}

/** Sampler avec une distribution d'échantillon uniforme à travers les pixels
(Stratified sampler without jittering)*/
pub struct DefaultSampler {
    pub sample_rate: u32,
}

impl Sampler for DefaultSampler {
    fn create_samples(&self, area: &mut SamplableArea) {
        let offset = area.offset();
        let (width, height) = area.dimensions();
        let sub_x = width / area.pixel_width() as f32;
        let sub_y = height / area.pixel_height() as f32;

        for y in 0..area.pixel_width() {
            for x in 0..area.pixel_height() {
                for i in 0..self.sample_rate {
                    for j in 0..self.sample_rate {
                        let sample_pos = Vector2f {
                            x : (x as f32 + i as f32 / self.sample_rate as f32 + 0.5) * sub_x,
                            y : (y as f32 + j as f32 / self.sample_rate as f32 + 0.5) * sub_y,
                        } + offset;

                        area.add_sample(Sample::new(sample_pos.x, sample_pos.y));
                    }
                }
            }
        }
    }
}

// TODO PBRT propose une opti pour la base 2
fn get_halton(a : u32, basis : u32) -> f32 {
    // Inversion de a
    let mut a_rev = 0;
    let mut a_not_rev = a;
    let inv_basis = 1_f32 / basis as f32;
    let mut total_div = 1_f32;

    while a_not_rev != 0 {
        let d = a_not_rev % basis;
        a_rev = basis * a_rev + d;
        a_not_rev /= basis;
        total_div *= inv_basis;
    }

    // Passage en décimal
    a_rev as f32 * total_div
}

/** Sampler 2D utilisant les séquences de Halton. */
pub struct HaltonSampler {
    pub sample_rate : u32,
}

impl Sampler for HaltonSampler {
    fn create_samples(&self, area: &mut SamplableArea) {
        let offset = area.offset();
        let (width, height) = area.dimensions();
        let sub_x = width / area.pixel_width() as f32;
        let sub_y = height / area.pixel_height() as f32;

        for y in 0..area.pixel_width() {
            for x in 0..area.pixel_height() {
                for i in 0..self.sample_rate {
                    let sample_pos = Vector2f {
                        x : (x as f32 + get_halton(i, 2)) * sub_x,
                        y : (y as f32 + get_halton(i, 3)) * sub_y,
                    } + offset;

                    area.add_sample(Sample::new(sample_pos.x, sample_pos.y));
                }
            }
        }
    }
}
