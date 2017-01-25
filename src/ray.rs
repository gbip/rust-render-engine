use math;

use math::Vector3f;
use math::VectorialOperations;

pub struct Ray {
    slope : Vector3f,
    origin : Vector3f,
}

#[derive(Debug, Clone)]
pub struct Plane {
    a : f32,
    b : f32,
    c : f32,
    d : f32,
}

/** Cet objet contient la position d'un point et un paramètre
qui permet de positionner ce point sur le rayon. */
pub struct IntersectionPoint {
    position : Vector3f,
    param : f32,
    unique : bool,
}



pub trait Surface {
    /** @returns the intersection point between the surface and
    the ray given. */
    fn getIntersectionPoint(&self, ray : &Ray) -> Option<IntersectionPoint>;
}


impl Plane {
    fn new(vec1 : &Vector3f, vec2 : &Vector3f, origin : &Vector3f) -> Plane {
        let cross = vec1.cross_product_ref(vec2);
        Plane {a : cross.x, b : cross.y, c : cross.z, d : - origin.dot_product(&cross)}
    }
}

impl Surface for Plane {

    fn getIntersectionPoint(&self, ray : &Ray) -> Option<IntersectionPoint> {

        let slope : &Vector3f = &ray.slope;
        let origin : &Vector3f = &ray.origin;

        // ax + by + cz + d = 0 <=> m * t = p
        let m = self.a * slope.x + self.b * slope.y + self.c * slope.z;
        let p = - (self.d + self.a * origin.x + self.b * origin.y + self.c * origin.z);

        let mut result : Option<IntersectionPoint>;
        if m == 0.0 {
            if p == 0.0 {
                result = Some(IntersectionPoint {
                    position : Vector3f {x : 0.0, y : 0.0, z : 0.0},
                    param : 0.0,
                    unique : false });
            }
            else {
                result = None;
            }
        }
        else {
            let t = p / m;

            result = Some(IntersectionPoint {
                position : Vector3f {
                    x : slope.x * t + origin.x,
                    y : slope.y * t + origin.y,
                    z : slope.z * t + origin.z,
                },
                param : t,
                unique : true,
            });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_plane_equation() {
        unimplemented!()
    }
}
