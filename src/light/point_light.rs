use math::{VectorialOperations, Vector3f};
use scene::World;
use light::Light;
use ray::Ray;

/** Represente une lumière ponctuelle */
#[derive(Serialize,Deserialize, Debug)]
pub struct PointLight {
    position: Vector3f,

    intensity: f32,
}


impl Light for PointLight {
    fn visible(&self, point: &Vector3f, world: &World) -> bool {
        let mut ray: Ray = Ray::new(*point, self.position - *point);
        ray.max_t = self.position.norm();
        world.is_occluded(&mut ray)
    }

    fn emit_rays(&self, point: &Vector3f, _: &World) -> Vec<Ray> {
        let mut result: Vec<Ray> = vec![];
        result.push(Ray::new(*point, self.position - *point));
        result
    }
}
