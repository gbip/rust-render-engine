use std::rc::Rc;
use std::cell::RefCell;
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
        let ray: Rc<RefCell<Ray>> = Rc::new(RefCell::new(Ray::new(self.position, slope)));
        ray.borrow_mut().max_t = 0.999;
        !world.is_occluded(ray)
    }

    fn emit_rays(&self, point: &Vector3f, _: &World) -> Vec<Rc<RefCell<Ray>>> {
        let mut result: Vec<Rc<RefCell<Ray>>> = vec![];
        let slope = *point - self.position;
        let ray: Rc<RefCell<Ray>> = Rc::new(RefCell::new(Ray::new(self.position, slope)));
        ray.borrow_mut().max_t = 0.999;
        result.push(ray);
        result
    }
}
