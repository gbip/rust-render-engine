use std::f64;
use std::f32;
use std::ops::*;
use num::{Float, NumCast};

#[macro_use]

const DEG_TO_RAD: f64 = f64::consts::PI / 180.0_f64;
const RAD_TO_DEG: f64 = 180.0_f64 / f64::consts::PI;

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct Deg<T>(pub T);

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct Rad<T>(pub T);

// OPERATEURS DE CONVERSION \\
// On utilise directement les méthodes from() et into() pour convertir.
impl<T> From<Rad<T>> for Deg<T>
    where T: Float,
          T: NumCast
{
    #[inline]
    fn from(rad: Rad<T>) -> Deg<T> {
        Deg(rad.0 * T::from(RAD_TO_DEG).unwrap())
    }
}

impl<'a, T> From<&'a Rad<T>> for Deg<T>
    where T: Float,
          T: NumCast
{
    #[inline]
    fn from(rad: &'a Rad<T>) -> Deg<T> {
        Deg((*rad).0 * T::from(RAD_TO_DEG).unwrap())
    }
}

impl<T> From<Deg<T>> for Rad<T>
    where T: Float,
          T: NumCast
{
    #[inline]
    fn from(deg: Deg<T>) -> Rad<T> {
        Rad(deg.0 * T::from(DEG_TO_RAD).unwrap())
    }
}

impl<'a, T> From<&'a Deg<T>> for Rad<T>
    where T: Float,
          T: NumCast
{
    #[inline]
    fn from(deg: &'a Deg<T>) -> Rad<T> {
        Rad((*deg).0 * T::from(DEG_TO_RAD).unwrap())
    }
}

impl<T> Add<Deg<T>> for Deg<T>
    where T: Add<T, Output = T>
{
    type Output = Deg<T>;
    #[inline]
    fn add(self, other: Deg<T>) -> Self {
        Deg(self.0 + other.0)
    }
}

macro_rules! impl_operations {

    ($K:ty) => 
    {

        // Conversion 
        
        impl From<$K> for Rad<$K> {
            #[inline]
            fn from(val:$K) -> Rad<$K> {
                Rad(val)
            }
        }
        
        impl<'a> From<&'a $K> for Rad<$K> {
            #[inline]
            fn from(val:&'a $K) -> Rad<$K> {
                Rad(val.clone())
            }
        }

        impl From<$K> for Deg<$K> {
            #[inline]
            fn from(val: $K) -> Deg<$K> {
                Deg(val)
            }
        }

        impl<'a> From<&'a $K> for Deg<$K> {
            #[inline]
            fn from(val: &'a $K) -> Deg<$K> {
                Deg(val.clone())
            }
        }

        // DIVISION

        impl Div<$K> for Deg<$K> {
            type Output=Deg<$K>;
            #[inline]
            fn div(self,other:$K) -> Self::Output {
                Deg(self.0/other)
            }
        }
        impl Div<$K> for Rad<$K> {
            type Output=Rad<$K>;
            #[inline]
            fn div(self,other:$K) -> Self::Output {
                Rad(self.0/other)
            }
        }
        
        // MULTIPLICATION
        
        impl Mul<$K> for Deg<$K> {
            type Output=Deg<$K>;
            #[inline]
            fn mul(self,other:$K) -> Deg<$K> {
                Deg(self.0*other)
            }
        }

        impl Mul<$K> for Rad<$K> {
            type Output=Rad<$K>;
            #[inline]
            fn mul(self,other:$K) -> Rad<$K> {
                Rad(self.0*other)
            }
        }
    }
}

#[macro_export]
macro_rules! deg {
    ($val:expr) => {
        Deg::from($val)
    }
}

#[macro_export]
macro_rules! rad {
    ($val:expr) => {
        Rad::from($val)
    }
}

impl_operations!(f32);
impl_operations!(f64);





#[cfg(test)]
mod test {
    use std::f32;
    use super::{Deg, Rad};
    #[test]
    fn test_conversion_deg_to_rad() {
        let a1 = deg!(360.0);
        let a2: Rad<f32> = rad!(2.0_f32 * f32::consts::PI);
        let a3: Deg<f32> = Deg::from(&a2);

        assert_eq!(a1, a3);

        let b1 = rad!(f32::consts::FRAC_PI_2);
        let b2 = deg!(180_f32);
        let b3: Rad<f32> = b2.into();
        assert_eq!(b1, b3 / 2_f32);
    }
}
