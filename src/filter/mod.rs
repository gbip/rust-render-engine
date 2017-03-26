pub mod filters;

use RGBA32;
use renderer::Pixel;


/** Un trait qui représente un filtre */
pub trait Filter {
    fn compute_color(&self, data: &Pixel, position: (u32, u32)) -> RGBA32;
}
