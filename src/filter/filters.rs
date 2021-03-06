use color_float::LinearColor;
use renderer::Pixel;
use math::{Vector2, Vector2f};
use filter::Filter;


/** Les paramètres standard d'un filtre de Mitchell-Netravali.
 * Le filtre à un rayon de 1 pixel : il ne regarde que les samples dans le pixel actuel*/
pub struct MitchellFilter {
    b: f32,
    c: f32,
    image_size: Vector2<u32>,
}

impl MitchellFilter {
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

    pub fn set_image_size(&mut self, x: u32, y: u32) {
        self.image_size = Vector2::new(x, y);
    }
}

impl Default for MitchellFilter {
    fn default() -> Self {
        MitchellFilter {
            b: 1.0 / 3.0,
            c: 1.0 / 3.0,
            image_size: Vector2::new(0, 0),
        }
    }
}

impl Filter for MitchellFilter {
    // TODO : Accélerer ce calcul ?
    fn compute_color(&self, data: &Pixel, pixel_position: (u32, u32)) -> LinearColor {
        let mut result: LinearColor = LinearColor::new_black();
        let mut weight_sum: f32 = 0.0;
        // On calcule les contributions de chaque sample
        for sample in data.samples() {
            // La position exprimée dans le système de coordonnée de l'image
            let absolute_sample_pos = sample.position();
            // On ramène la valeur pour la mettre au centre du pixel concerné
            let relative_sample_pixel_pos =
                Vector2f::new(absolute_sample_pos.x - data.x() as f32 - pixel_position.0 as f32 -
                              0.5,
                              absolute_sample_pos.y - data.y() as f32 - pixel_position.1 as f32 -
                              0.5);
            weight_sum += self.weight_contribution(relative_sample_pixel_pos);
        }
        for sample in data.samples() {
            // La position exprimée dans le système de coordonnée de l'image
            let absolute_sample_pos = sample.position();
            // On ramène la valeur pour la mettre au centre du pixel concerné
            let relative_sample_pixel_pos =
                Vector2f::new(absolute_sample_pos.x - data.x() as f32 - pixel_position.0 as f32 -
                              0.5,
                              absolute_sample_pos.y - data.y() as f32 - pixel_position.1 as f32 -
                              0.5);
            let weight = self.weight_contribution(relative_sample_pixel_pos);
            result += &(sample.color * (weight / weight_sum));
        }
        result
    }
}


#[derive(Default)]
pub struct BoxFilter {}


impl Filter for BoxFilter {
    fn compute_color(&self, data: &Pixel, _: (u32, u32)) -> LinearColor {
        let mut result: LinearColor = LinearColor::new_black();
        let sum: u32 = data.samples().fold(0, |acc, _| acc + 1);
        for sample in data.samples() {
            result += &(sample.color / sum as f32);
        }
        result

    }
}
