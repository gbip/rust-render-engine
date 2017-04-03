pub mod filters;

use RGBA32;
use renderer::Pixel;
use filter::filters::{BoxFilter, MitchellFilter};

/** Un trait qui représente un filtre */
pub trait Filter {
    fn compute_color(&self, data: &Pixel, position: (u32, u32)) -> RGBA32;
}

#[derive(Serialize,Deserialize, Debug)]
pub enum FilterFactory {
    BoxFilter,
    MitchellFilter,
}

impl FilterFactory {
    pub fn create_filter(&self, width: u32, height : u32) -> Box<Filter> {
        match *self {
            FilterFactory::BoxFilter => Box::new(BoxFilter::default()),
            FilterFactory::MitchellFilter => {
                let mut result = Box::new(MitchellFilter::default());
                result.set_image_size(width, height);
                result
            }
        }
    }
}
