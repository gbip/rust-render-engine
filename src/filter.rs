use RGBA32;
use render::Pixel;
use math::Vector2f;

trait Filter {
    fn compute_color(&self, data: Pixel) -> RGBA32;
}



/** Les paramètres standard d'un filtre de mitchell.
 * Le filtre à un rayon de 1 pixel : il ne regarde que les samples dans le pixel actuel*/
struct MNFilter {
    b: f32,
    c: f32,
}

impl MNFilter {
    fn weight_contribution(&self, coords: Vector2f) -> f32 {
        self.polynome(coords.x * 2.0) * self.polynome(coords.y * 2.0)
    }

    /** x doit appartenir à [-2,2] */
    fn polynome(&self, x: f32) -> f32 {
        let abs_x = f32::abs(x);

        if abs_x < 1.0 {
            (1.0 / 6.0) *
            ((12.0 - 9.0 * self.b - 6.0 * self.c) * f32::powi(abs_x, 3) +
             (-18.0 + 12.0 * self.b + 6.0 * self.c) * f32::powi(abs_x, 2) +
             (6.0 - 2.0 * self.b))

        } else if abs_x >= 1.0 && abs_x <= 2.0 {
            (1.0 / 6.0) *
            ((-self.b - 6.0 * self.c) * f32::powi(abs_x, 3) +
             (6.0 * self.b + 30.0 * self.c) * f32::powi(abs_x, 2) +
             (-12.0 * self.b - 48.0 * self.c) * abs_x + (8.0 * self.b + 24.0 * self.c))

        } else {
            0.0
        }
    }
}

impl Default for MNFilter {
    fn default() -> Self {
        MNFilter {
            b: 1.0 / 3.0,
            c: 1.0 / 3.0,
        }
    }
}

impl Filter for MNFilter {
    fn compute_color(&self, data: Pixel) -> RGBA32 {
        let mut result: RGBA32 = RGBA32::new_black();

        // On calcule les contributions de chaque sample
        for sample in data.samples() {
            let weight = self.weight_contribution(sample.position());
            result.r += (weight * sample.color.r() as f32) as u32;
            result.b += (weight * sample.color.b() as f32) as u32;
            result.g += (weight * sample.color.g() as f32) as u32;
        }


        result
    }
}
