use math;

use math::Vector3f;
use math::VectorialOperations;

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct IntersectionPoint {
    pub position : Vector3f,
    param : f32,
    unique : bool,
}



pub trait Surface {
    /** @returns the intersection point between the surface and
    the ray given. */
    fn get_intersection_point(&self, ray : &Ray) -> Option<IntersectionPoint>;
}


impl Plane {
    pub fn new(vec1 : &Vector3f, vec2 : &Vector3f, origin : &Vector3f) -> Plane {
        let cross = vec1.cross_product_ref(vec2);
        Plane {a : cross.x, b : cross.y, c : cross.z, d : - origin.dot_product(&cross)}
    }
}

impl Surface for Plane {

    fn get_intersection_point(&self, ray : &Ray) -> Option<IntersectionPoint> {

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
    use math::Vector3;
    use ray::*;

    #[test]
    fn test_plane_equation() {
        let v1 = Vector3f {
            x : 5.0 - 8.0,
            y : 3.0 - 3.0,
            z : 2.0 - 7.0,
        };

        let v2 = Vector3f {
            x : 5.0 - 8.0,
            y : 9.0 - 3.0,
            z : 1.0 - 7.0,
        };

        let origin = Vector3f {
            x : 8.0,
            y : 3.0,
            z : 7.0,
        };

        let test = Vector3f {
            x : 3.56,
            y : 8.21,
            z : 5.0 * 3.56 / 3.0 - 8.21 / 6.0 - 35.0 / 6.0,
        };

        let plane = Plane::new(&v1, &v2, &origin);
        let equation = plane.a * test.x + plane.b * test.y + plane.c * test.z + plane.d;
        assert!((if equation > 0.0 { equation } else { -equation}) < 0.00001);
    }

    #[test]
    fn test_plane_not_intersects_ray() {
        let plane = Plane {
            a : 0.0,
            b : 1.0,
            c : 6.0,
            d : 35.0
        };

        let ray = Ray {
            origin : Vector3f {
                x : 8.0,
                y : 7.0,
                z : 5.0
            },
            slope : Vector3f {
                x : 1.0,
                y : 0.0,
                z : 0.0,
            }
        };

        assert!(match plane.get_intersection_point(&ray) {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_plane_intersects_ray_once() {
        let plane = Plane {
            a : 0.0,
            b : 1.0,
            c : 6.0,
            d : 35.0
        };

        let ray = Ray {
            origin : Vector3f {
                x : 0.0,
                y : 0.0,
                z : 0.0
            },
            slope : Vector3f {
                x : 0.0,
                y : 1.0,
                z : 0.0,
            }
        };

        let intersection = plane.get_intersection_point(&ray);
        assert!(match intersection {
            None => false,
            Some(point) => (point.position - Vector3f {x : 0.0, y : -35.0, z : 0.0}).norm() < 0.00001,
        })
    }
}
