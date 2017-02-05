use math::Vector3f;
use math::VectorialOperations;
use color::RGBA32;

#[derive(Debug, PartialEq)]
pub struct Ray {
    slope : Vector3f,
    origin : Vector3f,
    // Un paramètre qui indique l'extrémité du rayon. Par exemple, lorsque le rayon est arrêté par
    // une surface il ne se propage pas sur les surfaces situées derrière.
    max_t : f32
}

#[derive(Debug, Clone)]
pub struct Plane {
    a : f32,
    b : f32,
    c : f32,
    d : f32,
}

#[derive(Debug, PartialEq)]
pub struct Fragment {
    pub position : Vector3f,
    pub normal : Vector3f,
    pub tex : Option<Vector3f>,
    pub param : f32,
    pub color : RGBA32
}



pub trait Surface {
    /** @returns the intersection point between the surface and
    the ray given. */
    fn get_intersection(&self, ray : &Ray, color : &RGBA32) -> Option<Fragment>;
}


impl Ray {
    fn new(origin : Vector3f, slope : Vector3f) -> Ray {
        Ray {origin : origin, slope : slope, max_t : -1.0}
    }
}

impl Plane {
    pub fn new(vec1 : &Vector3f, vec2 : &Vector3f, origin : &Vector3f) -> Plane {
        let cross = vec1.cross_product_ref(vec2);
        Plane {a : cross.x, b : cross.y, c : cross.z, d : - origin.dot_product(&cross)}
    }
}

impl Surface for Plane {

    fn get_intersection(&self, ray : &Ray, color: &RGBA32) -> Option<Fragment> {

        let slope : &Vector3f = &ray.slope;
        let origin : &Vector3f = &ray.origin;

        // ax + by + cz + d = 0 <=> m * t = p
        let m = self.a * slope.x + self.b * slope.y + self.c * slope.z;
        let p = - (self.d + self.a * origin.x + self.b * origin.y + self.c * origin.z);

        let result : Option<Fragment>;
        if m == 0.0 {
            result = None;
        }
        else {
            let t = p / m;

            if t < 0.0 || t > ray.max_t {
                //La surface est "avant" ou "après" le point d'émission du rayon
                result = None;
            }
            else {
                result = Some(Fragment::new(Vector3f {
                    x : slope.x * t + origin.x,
                    y : slope.y * t + origin.y,
                    z : slope.z * t + origin.z,
                }, t, color.clone()));
            }
        }

        result
    }
}

impl Fragment {
    pub fn new(position : Vector3f, param : f32, color : RGBA32) -> Fragment {
        Fragment {
            position : position,
            normal : Vector3f { x: 0_f32, y: 0_f32, z: 0_f32 },
            tex : None,
            param : param,
            color : color,
        }
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

        assert!(match plane.get_intersection(&ray, &RGBA32::new_black()) {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_plane_not_intersects_ray_due_to_wrong_sense() {
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

        let intersection = plane.get_intersection(&ray,&RGBA32::new_black());
        assert!(match intersection {
            None => true,
            Some(point) => false,
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
                y : -1.0,
                z : 0.0,
            }
        };

        let intersection = plane.get_intersection(&ray,&RGBA32::new_black());
        assert!(match intersection {
            None => false,
            Some(point) => (point.position - Vector3f {x : 0.0, y : -35.0, z : 0.0}).norm() < 0.00001,
        });
    }
}
