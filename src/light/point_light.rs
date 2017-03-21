use math::{VectorialOperations, Vector3f};
use scene::World;
use light::Light;
use ray::Ray;

/** Represente une lumière ponctuelle */
#[derive(Serialize,Deserialize)]
pub struct PointLight {
    position: Vector3f,

    intensitiy: f32,
}


impl Light for PointLight {
    fn visible(&self, point: &Vector3f, world: &World) -> bool {
        let mut ray: Ray = Ray::new(*point, self.position - *point);
        ray.max_t = self.position.norm();
        world.is_occluded(&mut ray)
    }
}
