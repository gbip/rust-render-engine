use ray::{Fragment, Ray};
use scene::World;
use color::RGBA32;
use renderer::TextureRegister;

pub mod channel;
pub mod flat_material;

pub trait Material {
    fn get_color(&self,
                 frag: &Fragment,
                 ray: &Ray,
                 world: &World,
                 texture_data: Option<&TextureRegister>)
                 -> RGBA32;
}
