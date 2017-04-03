use std::rc::Rc;
use std::cell::RefCell;
use math::{Vector3f, Vector2f};
use math::VectorialOperations;
use geometry::obj3d::Mesh;
use material::Material;
use scene::World;
use color::RGBA32;
use renderer::TextureRegister;

/** Represente un point d'intresection entre un rayon et de la géometrie */
pub struct Intersection<'a> {
    fragment: Fragment,
    geometry: &'a Mesh,
    material: &'a Material,
    ray: Rc<RefCell<Ray>>,
}

impl<'a> Intersection<'a> {
    /** Un peu de magie sur les lifetime pour que le compilo comprenne ce qu'il se passe*/
    pub fn new<'b: 'a, T: Material>(frag: Fragment,
                                    ray: Rc<RefCell<Ray>>,
                                    geo: &'b Mesh,
                                    mat: &'b T)
                                    -> Intersection<'a> {
        Intersection {
            fragment: frag,
            geometry: geo,
            material: mat,
            ray: ray.clone(),
        }
    }

    pub fn get_point_color(&self, world: &World, texture_register: &TextureRegister) -> RGBA32 {
        // TODO à simplifier (tout en gardant la gestion des cas anormaux)
        match self.fragment.tex {
            Some(_) => {
                self.material
                    .get_color(&self.fragment,
                               &self.ray.borrow(),
                               world,
                               Some(texture_register))
            }
            None => {
                self.material
                    .get_color(&self.fragment, &self.ray.borrow(), world, None)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    // Le vecteur directeur du rayon
    slope: Vector3f,
    // L'origine du rayon
    origin: Vector3f,
    // Un paramètre qui indique l'extrémité du rayon. Par exemple, lorsque le rayon est arrêté
    // par une surface il ne se propage pas sur les surfaces situées derrière.
    pub max_t: f32,

    inv_slope: Vector3f,
}

#[derive(Debug, Clone)]
pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fragment {
    pub position: Vector3f,
    pub normal: Vector3f,
    pub tex: Option<Vector2f>,
    pub param: f32,
}



pub trait Surface {
    /** Retourne le fragment crée par l'interseciton entre un rayon et de la géomètrie. Prends en
     entrée un Rc<RefCell<Ray>> pour éviter d'allouer de la mémoire à chaque création de fragment.
     On aurait pu modifier le Fragment pour permettre d'avoir un borrow sur un rayon, mais à cause
     de la mutabilité d'un rayon, cela n'est pas possible.
     Pourquoi le RefCell ? Il faut lire la doc : https://doc.rust-lang.org/std/cell/index.html
    */
    fn get_intersection_fragment(&self, ray: Rc<RefCell<Ray>>) -> Option<Fragment>;

    /** Il y a une implémentation par défaut, pour éviter de s'amuser à l'implémenter pour les
     * tests unitaires. */
    fn fast_intersection(&self, _: Rc<RefCell<Ray>>) -> bool {
        unreachable!();
    }
}


impl Ray {
    /** Crée un rayon à partir de son origine et d'un vecteur directeur. */
    pub fn new(origin: Vector3f, slope: Vector3f) -> Ray {
        Ray {
            origin: origin,
            slope: slope,
            max_t: -1.0,
            inv_slope: Vector3f::new(1.0 / slope.x, 1.0 / slope.y, 1.0 / slope.z),
        }
    }

    /** Renvoies 1/slope, utile pour accélerer les calcules d'intersection avec les bounding box
    ! */
    pub fn inv_slope(&self) -> Vector3f {
        self.inv_slope
    }

    pub fn slope(&self) -> Vector3f {
        self.slope
    }

    pub fn origin(&self) -> Vector3f {
        self.origin
    }
}

impl Plane {
    // Crée un plan à partir d'une origine et de deux vecteurs directeurs
    pub fn new(vec1: &Vector3f, vec2: &Vector3f, origin: &Vector3f) -> Plane {
        let cross = vec1.cross_product_ref(vec2);
        Plane {
            a: cross.x,
            b: cross.y,
            c: cross.z,
            d: -origin.dot_product(&cross),
        }
    }

    pub fn normal(&self) -> Vector3f {
        Vector3f {
            x: self.a,
            y: self.b,
            z: self.c,
        }
    }
}

impl Surface for Plane {
    fn get_intersection_fragment(&self, ray: Rc<RefCell<Ray>>) -> Option<Fragment> {

        let slope: &Vector3f = &ray.borrow().slope;
        let origin: &Vector3f = &ray.borrow().origin;

        // ax + by + cz + d = 0 <=> m * t = p
        let m = self.a * slope.x + self.b * slope.y + self.c * slope.z;
        let p = -(self.d + self.a * origin.x + self.b * origin.y + self.c * origin.z);

        if m == 0.0 {
            None
        } else {
            let t = p / m;

            if t < 0.0 || (ray.borrow().max_t > 0.0 && t > ray.borrow().max_t) {
                //La surface est "avant" ou "après" le point d'émission du rayon
                None
            } else {
                Some(Fragment::new(t * slope + *origin, t))
            }
        }
    }

    fn fast_intersection(&self, ray: Rc<RefCell<Ray>>) -> bool {
        let slope: &Vector3f = &ray.borrow().slope;
        self.a * slope.x + self.b * slope.y + self.c * slope.z != 0.0
    }
}

impl Fragment {
    pub fn new(position: Vector3f, param: f32) -> Fragment {
        Fragment {
            position: position,
            normal: Vector3f {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            tex: None,
            param: param,
        }
    }
}



#[cfg(test)]
mod tests {
    use ray::*;

    #[test]
    fn test_plane_equation() {
        let v1 = Vector3f {
            x: 5.0 - 8.0,
            y: 0.0,
            z: 2.0 - 7.0,
        };

        let v2 = Vector3f {
            x: 5.0 - 8.0,
            y: 9.0 - 3.0,
            z: 1.0 - 7.0,
        };

        let origin = Vector3f {
            x: 8.0,
            y: 3.0,
            z: 7.0,
        };

        let test = Vector3f {
            x: 3.56,
            y: 8.21,
            z: 5.0 * 3.56 / 3.0 - 8.21 / 6.0 - 35.0 / 6.0,
        };

        let plane = Plane::new(&v1, &v2, &origin);
        let equation = plane.a * test.x + plane.b * test.y + plane.c * test.z + plane.d;
        assert!((if equation > 0.0 { equation } else { -equation }) < 0.00001);
    }

    #[test]
    fn test_plane_not_intersects_ray() {
        let plane = Plane {
            a: 0.0,
            b: 1.0,
            c: 6.0,
            d: 35.0,
        };

        let ray = Rc::new(RefCell::new(Ray::new(Vector3f {
                                                 x: 8.0,
                                                 y: 7.0,
                                                 z: 5.0,
                                             },
                                             Vector3f {
                                                 x: 1.0,
                                                 y: 0.0,
                                                 z: 0.0,
                                             })));

        assert!(match plane.get_intersection_fragment(ray) {
                    None => true,
                    _ => false,
                });
    }

    #[test]
    fn test_plane_not_intersects_ray_due_to_wrong_sense() {
        let plane = Plane {
            a: 0.0,
            b: 1.0,
            c: 6.0,
            d: 35.0,
        };

        let ray = Rc::new(RefCell::new(Ray::new(Vector3f {
                                                 x: 0.0,
                                                 y: 0.0,
                                                 z: 0.0,
                                             },
                                             Vector3f {
                                                 x: 0.0,
                                                 y: 1.0,
                                                 z: 0.0,
                                             })));

        let intersection = plane.get_intersection_fragment(ray);
        assert!(match intersection {
                    None => true,
                    Some(_) => false,
                });
    }

    #[test]
    fn test_plane_intersects_ray_once() {
        let plane = Plane {
            a: 0.0,
            b: 1.0,
            c: 6.0,
            d: 35.0,
        };

        let ray = Rc::new(RefCell::new(Ray::new(Vector3f {
                                                 x: 0.0,
                                                 y: 0.0,
                                                 z: 0.0,
                                             },
                                             Vector3f {
                                                 x: 0.0,
                                                 y: -1.0,
                                                 z: 0.0,
                                             })));

        let intersection = plane.get_intersection_fragment(ray);
        assert!(match intersection {
                    None => false,
                    Some(point) => {
                        (point.position -
                             Vector3f {
                                 x: 0.0,
                                 y: -35.0,
                                 z: 0.0,
                             })
                            .norm() < 0.00001
                    }
                });
    }

    #[test]
    fn test_rc_ray() {
        let rc = Rc::new(RefCell::new(Ray::new(Vector3f {
                                                 x: 0.0,
                                                 y: 0.0,
                                                 z: 0.0,
                                             },
                                             Vector3f {
                                                 x: 0.0,
                                                 y: -1.0,
                                                 z: 0.0,
                                             })));
        rc.borrow_mut().max_t = 0.5;
        assert_eq!(rc.borrow().max_t, 0.5);
    }
}
