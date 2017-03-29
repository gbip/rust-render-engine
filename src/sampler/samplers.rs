use math::Vector2f;
use sampler::Sampler;

/** Sampler avec une distribution d'échantillon uniforme à travers les pixels
(Stratified sampler without jittering)*/
pub struct DefaultSampler {
    sample_rate: u32,
    sample_square_root : u32,
}

impl DefaultSampler {
    pub fn new(sample_rate : u32) -> DefaultSampler {
        DefaultSampler {
            sample_rate : sample_rate,
            sample_square_root : (sample_rate as f32).sqrt() as u32,
        }
    }
}

impl Sampler for DefaultSampler {
    fn get_sample_distribution(&self) -> Vec<Vector2f> {
        let mut result: Vec<Vector2f> = vec![];

        for i in 0..self.sample_square_root {
            for j in 0..self.sample_square_root {
                result.push(Vector2f {
                    x: i as f32 / self.sample_square_root as f32 + 0.5,
                    y: j as f32 / self.sample_square_root as f32 + 0.5,
                });
            }
        }

        result
    }
}

// TODO PBRT propose une opti pour la base 2
fn get_halton(a: u32, basis: u32) -> f32 {
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
    sample_rate: u32,
}

impl HaltonSampler {
    pub fn new(sample_rate : u32) -> HaltonSampler {
        HaltonSampler {
            sample_rate : sample_rate,
        }
    }
}

impl Sampler for HaltonSampler {
    fn get_sample_distribution(&self) -> Vec<Vector2f> {
        let mut result: Vec<Vector2f> = vec![];
        for i in 0..self.sample_rate {
            result.push(Vector2f {
                x: get_halton(i, 2),
                y: get_halton(i, 3),
            });
        }

        result
    }
}
