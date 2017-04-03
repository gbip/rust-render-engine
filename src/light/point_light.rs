use math::Vector3f;
use scene::World;
use light::Light;
use ray::Ray;
use color::RGBA32;

/** Represente une lumière ponctuelle */
#[derive(Serialize,Deserialize, Debug)]
pub struct PointLight {
    position: Vector3f,
    color: RGBA32,
    intensity: f32,
}


impl Light for PointLight {
    fn visible(&self, point: &Vector3f, world: &World) -> bool {
        let slope = *point - self.position;
        let mut ray: Ray = Ray::new(self.position, slope);
        ray.max_t = 0.999;
        !world.is_occluded(&mut ray)
    }

    fn emit_rays(&self, point: &Vector3f, _: &World) -> Vec<Ray> {
        let mut result: Vec<Ray> = vec![];
        let slope = *point - self.position;
        let mut ray: Ray = Ray::new(self.position, slope);
        ray.max_t = 0.999;
        result.push(ray);
        result
    }
}
