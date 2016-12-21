use std::vec::Vec;
use std::ops::{Add, Sub, Mul, Div};
// A basic module that implements some usefull mathematics tools
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Macro helper to implement for us basic arithmetic operations for all types that can
// represent a real number (f32, f64, u8, etc.)
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
        }
    }

// Generating the implementation
impl_operations!(f32);
impl_operations!(f64);
impl_operations!(u8);
impl_operations!(u16);
impl_operations!(u32);
impl_operations!(u64);

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


// A trait for basic vectorial arithmetic
pub trait VectorialOperations<T> {
    fn norm(self) -> f32;
    fn norm_ref(&self) -> f32;
    fn cross_product(self, other: Vector3<T>) -> Vector3<T>;
}

// The implementation is pretty straight forward
impl<T> VectorialOperations<T> for Vector3<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> + Into<f32> + Sub<Output = T>
{
    // What about f64 ?
    fn norm(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).into()
    }

    fn norm_ref(&self) -> f32 {
        Self::norm(Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        })
    }
    // The formula comes from https://fr.wikipedia.org/wiki/Produit_vectoriel
    fn cross_product(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}