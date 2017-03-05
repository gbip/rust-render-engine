use obj3D::{Object, Triangle};
use ray::{Surface, Ray, Fragment};
use math::Vector3f;
use std::f32;

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename="")]
pub struct BoundingBox {
    object: Object,

    #[serde(skip_serializing,skip_deserializing,default="new_max")]
    min: Vector3f,
    #[serde(skip_serializing,skip_deserializing,default="new_max")]
    max: Vector3f,
}

fn new_min() -> Vector3f {
    Vector3f::new(f32::MAX, f32::MAX, f32::MAX)
}

fn new_max() -> Vector3f {

    Vector3f::new(f32::MIN, f32::MIN, f32::MIN)
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

    pub fn initialize(&mut self) {
        self.object.initialize();
    }

    // Crées une Bounding Box contenant l'objet Object
    pub fn make_bbox(obj: Object) -> BoundingBox {
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

    pub fn object(&self) -> &Object {
        &self.object
    }
}

impl Surface for BoundingBox {
    fn get_intersection(&self, ray: &mut Ray) -> Option<Fragment> {
        if self.fast_intersect(ray) {
            self.object.get_intersection(ray)
        } else {
            None
        }
    }
}
