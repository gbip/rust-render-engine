use ray::Fragment;
use scene::World;
use color::RGBA32;

pub mod channel;
pub mod flat_material;

pub trait Material {
    fn get_color(&self, frag: &Fragment, world: &World) -> RGBA32;
}
