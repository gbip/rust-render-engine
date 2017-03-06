use obj3D::{Object, Triangle};
use ray::{Surface, Ray, Fragment};
use math::Vector3f;
use std::f32;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug)]
pub struct BoundingBox {
    object: Object,

    min: Vector3f,

    max: Vector3f,
}



// Serialization custom afin de cacher la bbox dans le json
impl Serialize for BoundingBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.object.serialize(serializer)
    }
}

impl Deserialize for BoundingBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {

        Ok(BoundingBox {
            object: Object::deserialize(deserializer)?,
            min: new_min(),
            max: new_max(),
        })
    }
}

fn new_min() -> Vector3f {
    Vector3f::new(f32::MAX, f32::MAX, f32::MAX)
}

fn new_max() -> Vector3f {

    Vector3f::new(f32::MIN, f32::MIN, f32::MIN)
}

impl BoundingBox {
    pub fn new(obj: Object) -> Self {
        BoundingBox {
            object: obj,
            min: new_min(),
            max: new_max(),
        }
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

    pub fn initialize(&mut self) {
        println!("Init");
        self.object.initialize();
        self.make_bbox();
    }

    // Crées une Bounding Box contenant l'objet Object
    // TODO : Eviter la copie de l'objet !!!!
    pub fn make_bbox(&mut self) {
        let obj_copy = self.object.clone();
        for tri in obj_copy.triangles() {
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
