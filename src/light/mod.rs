pub mod point_light;

use scene::World;
use math::Vector3f;
use ray::Ray;

/** Un trait qui represente une lumière */
pub trait Light {
    fn visible(&self, point: &Vector3f, world: &World) -> bool;
    fn emit_rays(&self, point: &Vector3f, world: &World) -> Vec<Ray>;
}
