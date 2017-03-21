pub mod point_light;

use scene::World;
use math::Vector3f;

/** Un trait qui represente une lumière */
pub trait Light {
    fn visible(&self, point: &Vector3f, world: &World) -> bool;
}
