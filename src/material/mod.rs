use ray::Fragment;
use scene::World;
use color::RGBA32;
use renderer::TextureRegister;

pub mod channel;
pub mod flat_material;

pub trait Material {
    fn get_color(&self,
                 frag: &Fragment,
                 world: &World,
                 texture_data: Option<(f32, f32, &TextureRegister)>)
                 -> RGBA32;
}
