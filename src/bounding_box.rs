use obj3D::{Object, Triangle};
use ray::{Surface, Ray, Fragment};
use math::Vector3f;
use std::f32;

pub struct BoundingBox {
    object: Object,
    min: Vector3f,
    max: Vector3f,
}

impl BoundingBox {
    // Ajoute un point à une Bounding Box
    fn add_point(&mut self, b: Vector3f) {

        self.min = Vector3f::new(f32::min(self.min.x, b.x),
                                 f32::min(self.min.y, b.y),
                                 f32::min(self.min.z, b.z));

        self.max = Vector3f::new(f32::max(self.max.x, b.x),
                                 f32::max(self.max.y, b.y),
                                 f32::max(self.max.z, b.z));
    }

    // Ajoute un triangle à une Bounding Box
    fn add_triangle(&mut self, b: &Triangle) {
        self.add_point(b.u_pos());
        self.add_point(b.v_pos());
        self.add_point(b.w_pos());
    }

    // Crées une Bounding Box contenant l'objet Object
    fn make_bbox(obj: Object) -> BoundingBox {

        let mut result = BoundingBox {
            object: Object::new_empty(),
            min: Vector3f::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vector3f::new(f32::MIN, f32::MIN, f32::MIN),
        };


        for tri in obj.triangles() {
            result.add_triangle(tri);
        }

        result.object = obj;

        result

    }
}

impl Surface for BoundingBox {
    fn get_intersection(&self, ray: &mut Ray) -> Option<Fragment> {

        None
    }
}
