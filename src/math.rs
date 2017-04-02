use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign};
use std::cmp::PartialEq;
use std::fmt;
use std::f32;
// A basic module that implements some usefull mathematics tools
#[derive(Debug, Copy, Clone,Serialize,Deserialize)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// A square matrix of size 3.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Matrix3<T>
    where T: Copy + PartialEq
{
    data: [[T; 3]; 3],
}

// A bery basic structure for handling 2D stuff(projection, etc.)
#[derive(Debug, Eq, Clone, PartialEq, Copy,Serialize,Deserialize)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// L'égalité pour les nombres flottants avec un seuil
pub trait AlmostEq<T> {
    // Si le seuil (threshold) est absent, alors l'implémentation doit le mettre à la plus petite
    // valeur possible.
    fn equal_with_threshold(&self, other: &Self, threshold: Option<T>) -> bool;

    // De même que pour equal_with_threshold, l'implémentation doit spécifier la plus petite
    // valeur possible si le seuil est absent
    fn not_equal_with_threshold(&self, other: &Self, threshold: Option<T>) -> bool;

    // Lire "almost non equal"
    fn ane(&self, other: &Self) -> bool;

    // Lire "almost equal". Le threshold est à f32::EPSILON.
    fn aeq(&self, other: &Self) -> bool;
}

impl<T> Vector3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x: x, y: y, z: z }
    }
}

impl<T> fmt::Display for Vector3<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} , {} , {})", self.x, self.y, self.z)
    }
}

impl<T> Vector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x: x, y: y }
    }
}

impl<T> fmt::Display for Vector2<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} , {})", self.x, self.y)
    }
}
// One basic aliases for implementation convenience in other module.
pub type Vector3f = Vector3<f32>;
pub type Vector2f = Vector2<f32>;

impl Vector3f {
    pub fn zero() -> Self {
        Vector3f::new(0_f32, 0_f32, 0_f32)
    }
}

impl AlmostEq<f32> for Vector3<f32> {
    fn equal_with_threshold(&self, other: &Self, threshold: Option<f32>) -> bool {
        let new_threshold = match threshold {
            Some(thre) => thre,
            None => f32::EPSILON,
        };
        (((self.x - other.x).abs() <= new_threshold) &&
         ((self.y - other.y).abs() <= new_threshold) &&
         ((self.z - other.z).abs() <= new_threshold))
    }

    fn not_equal_with_threshold(&self, other: &Self, threshold: Option<f32>) -> bool {
        !AlmostEq::equal_with_threshold(self, other, threshold)
    }

    fn ane(&self, other: &Self) -> bool {
        AlmostEq::not_equal_with_threshold(self, other, None)
    }

    fn aeq(&self, other: &Self) -> bool {
        AlmostEq::equal_with_threshold(self, other, None)
    }
}

// Implementation of the operator '=='
impl<T> PartialEq<Vector3<T>> for Vector3<T>
    where T: PartialEq
{
    fn eq(&self, other: &Vector3<T>) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl<T> Neg for Vector3<T>
    where T: Neg<Output = T>
{
    type Output = Vector3<T>;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: Neg::neg(self.x),
            y: Neg::neg(self.y),
            z: Neg::neg(self.z),
        }
    }
}

impl<T> AddAssign for Vector3<T>
    where T: AddAssign
{
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}


// Macro helper to implement for us basic arithmetic operations for all types that can
// represent a real number (f32, f64, u8, etc.).
// Maybe it is possible to factorize the macro, but I don't know how to do it for now.
macro_rules! impl_operations {
        ($K:ty) => {
            // Implementation of the multiplication of a vector by a real number
            impl<T> Mul<$K> for Vector3<T> where
                T : Mul<$K, Output=T> {
                type Output = Vector3<T>;
                fn mul(self, other : $K) -> Self::Output {
                    Vector3{x: self.x*other,
                            y: self.y*other,
                            z: self.z*other}
                    }
                }

            // Same but for reference
            impl<'a,T> Mul<$K> for &'a Vector3<T> where
                T : Mul<$K, Output=T> + Copy {
                type Output = Vector3<T>;
                fn mul(self, other : $K) -> Self::Output {
                    Vector3{x: self.x*other,
                            y: self.y*other,
                            z: self.z*other}
                    }
                }

            // Since the trait std::ops::Mul is not reflexive, we have to implement the k*v(x,y,z)
            // and v(x,y,z)*k
            impl<T> Mul<Vector3<T>> for $K where
            T : Mul<$K, Output=T> {
                type Output = Vector3<T>;
                fn mul(self, other : Vector3<T>) -> Self::Output {
                    Vector3{x: other.x*self,
                            y: other.y*self,
                            z: other.z*self}
                    }
                }
            // Again, the reference version
            impl<'a,T> Mul<&'a Vector3<T>> for $K where
            T : Mul<$K, Output=T> + Copy {
                type Output = Vector3<T>;
            fn mul(self, other : &'a Vector3<T>) -> Self::Output {
                    Vector3{x: other.x*self,
                            y: other.y*self,
                            z: other.z*self}
                    }
                }



            // Implementation of the division of a vector by a real number
            impl<T> Div<$K> for Vector3<T> where
                T : Div<$K, Output=T> {
                type Output = Vector3<T>;
                fn div(self, other : $K) -> Self::Output {
                    Vector3{x: self.x/other,
                            y: self.y/other,
                            z: self.z/other}
                }
            }
            // Same but for reference
            impl<'a,T> Div<$K> for &'a Vector3<T> where
                T : Div<$K, Output=T> + Copy{
                type Output = Vector3<T>;
                fn div(self, other : $K) -> Self::Output {
                    Vector3{x: self.x/other,
                            y: self.y/other,
                            z: self.z/other}
                }
            }

            // Implementation of the multiplication of a vector2 by a real number
            impl<T> Mul<$K> for Vector2<T> where
                T : Mul<$K, Output=T> {
                type Output = Vector2<T>;
                fn mul(self, other : $K) -> Self::Output {
                    Vector2{x: self.x*other,
                            y: self.y*other}
                    }
                }

            // Same but for reference
            impl<'a,T> Mul<$K> for &'a Vector2<T> where
                T : Mul<$K, Output=T> + Copy {
                type Output = Vector2<T>;
                fn mul(self, other : $K) -> Self::Output {
                    Vector2{x: self.x*other,
                            y: self.y*other}
                    }
                }

            // Since the trait std::ops::Mul is not reflexive, we have to implement the k*v(x,y)
            // and v(x,y)*k
            impl<T> Mul<Vector2<T>> for $K where
            T : Mul<$K, Output=T> {
                type Output = Vector2<T>;
                fn mul(self, other : Vector2<T>) -> Self::Output {
                    Vector2{x: other.x*self,
                            y: other.y*self}
                    }
                }
            // Again, the reference version
            impl<'a,T> Mul<&'a Vector2<T>> for $K where
            T : Mul<$K, Output=T> + Copy {
                type Output = Vector2<T>;
            fn mul(self, other : &'a Vector2<T>) -> Self::Output {
                    Vector2{x: other.x*self,
                            y: other.y*self}
                }
            }
        }
    }

// Generating the implementation for all the types that interest us
impl_operations!(f32);
impl_operations!(f64);

impl_operations!(u8);
impl_operations!(u16);
impl_operations!(u32);
impl_operations!(u64);

impl_operations!(i8);
impl_operations!(i16);
impl_operations!(i32);
impl_operations!(i64);


impl_operations!(isize);
impl_operations!(usize);

// Implementation of the addition of two vectors
impl<T> Add<Vector3<T>> for Vector3<T>
    where T: Add<Output = T>
{
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Self::Output {

        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Reference version
impl<'a, T> Add<&'a Vector3<T>> for &'a Vector3<T>
    where T: Add<Output = T> + Copy
{
    type Output = Vector3<T>;

    fn add(self, other: &'a Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Basic implementation for the substraction
impl<T> Sub<Vector3<T>> for Vector3<T>
    where T: Sub<Output = T>
{
    type Output = Vector3<T>;
    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
// Same implementation but for reference
impl<'a, T> Sub<&'a Vector3<T>> for &'a Vector3<T>
    where T: Sub<Output = T> + Copy
{
    type Output = Vector3<T>;
    fn sub(self, other: &'a Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<Vector3<T>> for Vector3<T>
    where T: Mul<T, Output = T>
{
    type Output = Vector3<T>;
    fn mul(self, other: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<'a, T> Mul<&'a Vector3<T>> for &'a Vector3<T>
    where T: Mul<T, Output = T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: &'a Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


// Implementation of the addition of two vectors
impl<T> Add<Vector2<T>> for Vector2<T>
    where T: Add<Output = T>
{
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Self::Output {

        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Reference version
impl<'a, T> Add<&'a Vector2<T>> for &'a Vector2<T>
    where T: Add<Output = T> + Copy
{
    type Output = Vector2<T>;

    fn add(self, other: &'a Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Basic implementation for the substraction
impl<T> Sub<Vector2<T>> for Vector2<T>
    where T: Sub<Output = T>
{
    type Output = Vector2<T>;
    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
// Same implementation but for reference
impl<'a, T> Sub<&'a Vector2<T>> for &'a Vector2<T>
    where T: Sub<Output = T> + Copy
{
    type Output = Vector2<T>;
    fn sub(self, other: &'a Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// A trait for basic vectorial arithmetic
pub trait VectorialOperations<T> {
    fn norm(self) -> f32;
    fn norm_ref(&self) -> f32;
    fn cross_product(self, other: &Vector3<T>) -> Vector3<T>;
    fn cross_product_ref(&self, other: &Vector3<T>) -> Vector3<T>;
    fn dot_product(self, other: &Vector3<T>) -> f32;
    fn dot_product_ref(&self, other: &Vector3<T>) -> f32;
}

// This is a macro in case we need to swith to f64 one day. Otherwise there is no reason to make it
// a macro.
macro_rules! impl_vec_operations {
    ($K:tt) => {
        impl<T> VectorialOperations<T> for Vector3<T> where
            T: Copy + Mul<Output = T> + Add<Output = T> + Into<$K> + Sub<Output = T> {



            fn norm(self) -> $K {
                ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).into().sqrt()
            }

            fn norm_ref(&self) -> $K {
                Self::norm(Vector3 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                })
            }
            // The formula comes from https://fr.wikipedia.org/wiki/Produit_vectoriel
            fn cross_product(self, other: &Vector3<T>) -> Vector3<T> where
                T : Copy {
                Vector3 {
                    x: self.y * other.z - self.z * other.y,
                    y: self.z * other.x - self.x * other.z,
                    z: self.x * other.y - self.y * other.x,
                }
            }
            fn cross_product_ref(&self, other : &Vector3<T>) -> Vector3<T> {
                Self::cross_product(self.clone(), other)
            }

            fn dot_product(self, other : &Vector3<T>) -> $K {
                (self.x*other.x + self.y*other.y + self.z*other.z).into()
            }
            fn dot_product_ref(&self, other : &Vector3<T>) -> $K {
                Self::dot_product(self.clone(), other)
            }
        }
    }
}

impl_vec_operations!(f32);


#[cfg(test)]
#[allow(float_cmp)]
mod tests {
    use math::*;

    #[test]
    fn test_vec_norm() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 2_f32,
            z: 3_f32,
        };
        assert_eq!(v1.norm(), 14_f32.sqrt());

        let v2 = Vector3 {
            x: -70_f32,
            y: 35_f32,
            z: 0_f32,
        };
        assert_eq!(v2.norm(), 35.0 * (5.0_f32.sqrt()));

        let v3 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };
        assert_eq!(v3.norm(), 0_f32.sqrt());

        let v4 = Vector3 {
            x: -1_f32,
            y: 0_f32,
            z: 1_f32,
        };
        assert_eq!(v4.norm(), 2_f32.sqrt());

        let v5 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 1_f32,
        };
        assert_eq!(v5.norm(), 1_f32.sqrt());

        let v6 = Vector3 {
            x: 127.18_f32,
            y: 55.456_f32,
            z: 33.333_f32,
        };
        assert_eq!(v6.norm(), 142.69272);

        let v7 = Vector3 {
            x: 2_f32,
            y: 0_f32,
            z: 2_f32,
        };
        assert_eq!(v7.norm(), 2_f32 * 2_f32.sqrt());
    }

    #[test]
    fn test_vec_eq() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 1_f32,
            z: 1_f32,
        };
        let v2 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 1_f32,
        };
        let v3 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };
        assert_eq!(v1,
                   Vector3 {
                       x: 1_f32,
                       y: 1_f32,
                       z: 1_f32,
                   });
        assert!(v2 != v3 && v1 != v3);
    }

    #[test]
    fn test_vec_mul() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 1_f32,
            z: 1_f32,
        };
        assert_eq!(&v1 * 2_f32,
                   Vector3 {
                       x: 2_f32,
                       y: 2_f32,
                       z: 2_f32,
                   });
        assert_eq!(2_f32 * v1,
                   Vector3 {
                       x: 2_f32,
                       y: 2_f32,
                       z: 2_f32,
                   });
        let v2 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };
        assert_eq!(&v2 * 2_f32, v2);
    }

    #[test]
    fn test_vec_vec_mul() {
        let v1 = Vector3 {
            x: 2_f32,
            y: 2_f32,
            z: 2_f32,
        };
        let v2 = Vector3 {
            x: 0_f32,
            y: 1_f32,
            z: 2_f32,
        };

        assert!((v1 * v2).aeq(&Vector3 {
                                   x: 0_f32,
                                   y: 2_f32,
                                   z: 4_f32,
                               }));
        assert!((&v1 * &v2).aeq(&Vector3 {
                                     x: 0_f32,
                                     y: 2_f32,
                                     z: 4_f32,
                                 }));
    }

    #[test]
    fn test_vec_arithmetic() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 1_f32,
            z: 1_f32,
        };
        let v2 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 1_f32,
        };
        let v3 = Vector3 {
            x: 55_f32,
            y: -3_f32,
            z: 9_f32,
        };
        let zero = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };
        assert_eq!(&v1 - &v2,
                   Vector3 {
                       x: 1_f32,
                       y: 1_f32,
                       z: 0_f32,
                   });
        assert_eq!(&v1 + &v2,
                   Vector3 {
                       x: 1_f32,
                       y: 1_f32,
                       z: 2_f32,
                   });
        assert_eq!(&v1 + &zero, v1);
        assert_eq!(&v1 - &zero, v1);
        assert_eq!(&v3 - &v1,
                   Vector3 {
                       x: 54_f32,
                       y: -4_f32,
                       z: 8_f32,
                   });
    }

    #[test]
    fn test_vec_div() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 1_f32,
            z: 1_f32,
        };
        let v3 = Vector3 {
            x: 55_f32,
            y: -3_f32,
            z: 9_f32,
        };
        let zero = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };
        assert_eq!(&zero / 4_f32, zero);
        assert_eq!(&v1 / 1_f32, v1);
        assert_eq!(&v1 / 2_f32,
                   Vector3 {
                       x: 0.5_f32,
                       y: 0.5_f32,
                       z: 0.5_f32,
                   });
        assert_eq!(v3 / 12_f32,
                   Vector3 {
                       x: (55_f32 / 12_f32),
                       y: (-3_f32 / 12_f32),
                       z: (9_f32 / 12_f32),
                   });
    }

    #[test] // TODO : Implement more tests
    fn test_vec_cross_product() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 1_f32,
            z: 1_f32,
        };
        let v2 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 1_f32,
        };
        let zero = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };

        assert_eq!(v1.cross_product_ref(&zero), zero);
        assert_eq!(v1.cross_product_ref(&v2),
                   Vector3 {
                       x: 1_f32,
                       y: -1_f32,
                       z: 0_f32,
                   })
    }

    #[test]
    fn test_vec_dot_product() {
        let v1 = Vector3 {
            x: 1_f32,
            y: 1_f32,
            z: 1_f32,
        };
        let v2 = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: -1_f32,
        };
        let v3 = Vector3 {
            x: 55_f32,
            y: -3_f32,
            z: 9_f32,
        };
        let zero = Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        };

        assert_eq!(v1.dot_product_ref(&zero), 0_f32);
        assert_eq!(v1.dot_product_ref(&v2), -1_f32);
        assert_eq!(v1.dot_product_ref(&v3), 61_f32);
        assert_eq!(v2.dot_product_ref(&v3), -9_f32);
        assert_eq!(v1.dot_product_ref(&v1), 3_f32);
    }
    #[test]
    fn test_almot_eq_vec3() {
        let v1 = Vector3::new(0_f32, 0_f32, 0_f32);
        let v2 = Vector3::new(1_f32, 0_f32, 0_f32);
        let v3 = Vector3::new(0_f32, 1_f32, 0_f32);
        let v4 = Vector3::new(0_f32, 0_f32, 1_f32);
        let v5 = Vector3::new(0_f32, 0_f32, 1_f32 + f32::EPSILON);
        let mut v6 = Vector3::new(f32::EPSILON, 0_f32, f32::EPSILON);
        let v7 = Vector3::new(-f32::EPSILON, -f32::EPSILON, 0_f32);

        assert!(v1.ane(&v2) && v1.ane(&v3) && v1.ane(&v4) && v1.ane(&v5));
        assert!(v2.ane(&v3) && v2.ane(&v4) && v3.ane(&v4));
        assert!(v6.aeq(&v1));
        // transitivité
        assert!(v6.ane(&v7));
        assert!(v7.ane(&v6));
        assert!(v1.aeq(&v6) && v1.aeq(&v7));
        v6 = 2.0 * v6;
        assert!(v6.ane(&v1));

    }
}
