pub mod point_light;
use scene::World;
use math::Vector3f;
use ray::Ray;
use light::point_light::PointLight;

/** Un trait qui represente une lumière */
pub trait Light {
    fn visible(&self, point: &Vector3f, world: &World) -> bool;
    fn emit_rays(&self, point: &Vector3f, world: &World) -> Vec<Ray>;
    fn intensity(&self) -> f32;
}

// Pour la sérialisation

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LightObject {
    Point { point: PointLight },
}

impl LightObject {
    pub fn as_trait(&self) -> &Light {
        match *self {
            LightObject::Point { ref point } => point,
        }
    }
}
