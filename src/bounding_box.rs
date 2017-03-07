use obj3D::{Object, Triangle};
use ray::Ray;
use math::Vector3f;
use std::f32;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    min: Vector3f,
    max: Vector3f,
}

fn new_min() -> Vector3f {
    Vector3f::new(f32::MAX, f32::MAX, f32::MAX)
}

fn new_max() -> Vector3f {

    Vector3f::new(f32::MIN, f32::MIN, f32::MIN)
}

impl BoundingBox {
    pub fn new() -> Self {
        BoundingBox {
            min: new_min(),
            max: new_max(),
        }
    }

    pub fn new_from_object(obj: &Object) -> Self {
        let mut result = BoundingBox {
            min: new_min(),
            max: new_max(),
        };
        result.adapt_to(obj);

        result
    }
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
    pub fn adapt_to(&mut self, obj: &Object) {
        for tri in obj.triangles() {
            self.add_triangle(tri);
        }
    }


    // Algorithme issue de : http://people.csail.mit.edu/amy/papers/box-jgt.pdf
    fn fast_intersect(&self, ray: &Ray) -> bool {
        // X
        let mut tmin: f32;
        let mut tmax: f32;

        // Y
        let tymin: f32;
        let tymax: f32;

        // Z
        let tzmin: f32;
        let tzmax: f32;


        let divx: f32 = 1.0 / ray.slope().x;
        let divy: f32 = 1.0 / ray.slope().y;
        let divz: f32 = 1.0 / ray.slope().z;

        // X
        if divx >= 0f32 {
            tmin = (self.min.x - ray.origin().x) * divx;
            tmax = (self.max.x - ray.origin().x) * divx;

        } else {
            tmin = (self.max.x - ray.origin().x) * divx;
            tmax = (self.min.x - ray.origin().x) * divx;
        }

        // Y
        if divy >= 0f32 {
            tymin = (self.min.y - ray.origin().y) * divy;
            tymax = (self.max.y - ray.origin().y) * divy;
        } else {
            tymin = (self.max.y - ray.origin().y) * divy;
            tymax = (self.min.y - ray.origin().y) * divy;
        }

        // Cas facile, pas besoin de traiter les Z
        if tmin > tymax || tymin > tmax {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }


        // Z
        if divz >= 0f32 {
            tzmin = (self.min.z - ray.origin().z) * divz;
            tzmax = (self.max.z - ray.origin().z) * divz;
        } else {
            tzmin = (self.max.z - ray.origin().z) * divz;
            tzmax = (self.min.z - ray.origin().z) * divz;
        }

        if tmin > tzmax || tzmin > tmax {
            return false;
        }

        true
    }

    pub fn intersects(&self, ray : &Ray) -> bool {
        self.fast_intersect(ray)
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox::new()
    }
}
