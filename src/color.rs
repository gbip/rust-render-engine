use img;
use std::u8;
use std::ops::Mul;

// A struct for all internal color management, but not for textures and objecs colors.
#[derive(Clone,Debug,Copy,Serialize,Deserialize,PartialEq)]
pub struct RGBA32 {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

// A struct to support the classic 8 bit color values that is used to : write to a .png file,
// manage object color through textures.
#[derive(Clone,Debug,Copy,Serialize,Deserialize,PartialEq)]
pub struct RGBA8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn u32_to_u8(v: u32) -> u8 {
    let conversion_factor = ((u32::max_value() as u64 + 1u64) /
                             (u8::max_value() as u64 + 1u64)) as u32;
    (v / conversion_factor) as u8
}

fn u8_to_u32(v: u8) -> u32 {
    let conversion_factor = ((u32::max_value() as u64 + 1u64) /
                             (u8::max_value() as u64 + 1u64)) as u32;
    (v as u32 * conversion_factor)
}

// TODO : Verifier les histoires d'espace de couleur linéaire et tout et tout
// /!\ On ne gère pas la transparence !!!!
pub fn make_average_color(colors: &[RGBA32]) -> RGBA32 {

    // Calcul de la couleur moyenne

    let number_of_colors = colors.len();
    let squared_colors: Vec<(u64, u64, u64)> =
        colors.into_iter().map(|color| (color.r as u64, color.g as u64, color.b as u64)).collect();
    let mut acc: (u64, u64, u64) = (0, 0, 0);
    for c in squared_colors {
        acc.0 += c.0;
        acc.1 += c.1;
        acc.2 += c.2;
    }
    acc.0 /= number_of_colors as u64;
    acc.1 /= number_of_colors as u64;
    acc.2 /= number_of_colors as u64;

    let r = acc.0;
    let g = acc.1;
    let b = acc.2;

    // Calcul de la transparence
    // A FAIRE

    RGBA32::new(&(r as u32), &(g as u32), &(b as u32), &u32::max_value())

}

impl RGBA8 {
    // A fully opaque black color
    pub fn new_black() -> RGBA8 {
        RGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: u8::max_value(),
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn new(r: &u8, g: &u8, b: &u8, a: &u8) -> RGBA8 {
        RGBA8 {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        }
    }

    pub fn to_rgba32(&self) -> RGBA32 {
        RGBA32::new(&u8_to_u32(self.r),
                    &u8_to_u32(self.g),
                    &u8_to_u32(self.b),
                    &u8_to_u32(self.a))
    }
}

impl RGBA32 {
    // A fully opaque black color
    pub fn new_black() -> RGBA32 {
        RGBA32 {
            r: 0,
            g: 0,
            b: 0,
            a: u32::max_value(),
        }
    }

    pub fn r(&self) -> u32 {
        self.r
    }

    pub fn g(&self) -> u32 {
        self.g
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn new(r: &u32, g: &u32, b: &u32, a: &u32) -> RGBA32 {
        RGBA32 {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        }
    }

    // /!\ On ne gère pas la transparence !
    pub fn square(self) -> (u64, u64, u64) {
        ((self.r as u64).pow(2), (self.g as u64).pow(2), (self.b as u64).pow(2))
    }

    pub fn to_rgba8(&self) -> RGBA8 {
        RGBA8::new(&u32_to_u8(self.r),
                   &u32_to_u8(self.g),
                   &u32_to_u8(self.b),
                   &u32_to_u8(self.a))

    }
}

impl img::Pixel for RGBA32 {
    fn to_rgb_pixel(&self) -> (u8, u8, u8) {
        (u32_to_u8(self.r), u32_to_u8(self.g), u32_to_u8(self.b))
    }

    fn to_rgba_pixel(&self) -> (u8, u8, u8, u8) {
        (u32_to_u8(self.r), u32_to_u8(self.g), u32_to_u8(self.b), u32_to_u8(self.a))
    }
}

impl img::Pixel for RGBA8 {
    fn to_rgb_pixel(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    fn to_rgba_pixel(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl Mul<RGBA32> for RGBA32 {
    type Output = RGBA32;
    fn mul(self, other: RGBA32) -> Self::Output {
        RGBA32 {
            r: (self.r as u64 * other.r as u64 / u32::max_value() as u64) as u32,
            g: (self.g as u64 * other.g as u64 / u32::max_value() as u64) as u32,
            b: (self.b as u64 * other.b as u64 / u32::max_value() as u64) as u32,
            a: (self.a as u64 * other.a as u64 / u32::max_value() as u64) as u32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{RGBA8, RGBA32};

    #[test]
    fn test_simple_color_conversion() {
        let a = RGBA8::new(&0, &0, &0, &0);
        assert_eq!(a, a.to_rgba32().to_rgba8());

        let b = RGBA8::new(&128, &128, &128, &128);
        assert_eq!(b, b.to_rgba32().to_rgba8());

        let c = RGBA8::new(&255, &255, &255, &255);
        assert_eq!(c, c.to_rgba32().to_rgba8());

        let d = RGBA32::new(&0, &0, &0, &0);
        assert_eq!(d, d.to_rgba8().to_rgba32());

        let e = RGBA32::new(&16777216, &16777216, &16777216, &16777216);
        assert_eq!(e, e.to_rgba8().to_rgba32());

        let f = RGBA32::new(&4278190080, &4278190080, &4278190080, &4278190080);
        assert_eq!(f, f.to_rgba8().to_rgba32());
    }

    #[test]
    fn test_advanced_color_conversion() {
        // Test des arrondis :
        // Ici on doit arrondir à 0.
        let a = RGBA32::new(&1, &1, &1, &1);
        assert_eq!(a.to_rgba8(), RGBA8::new(&0, &0, &0, &0));

        // Ici on doit arrondir à 0.
        let b = RGBA32::new(&16777215, &16777215, &16777215, &16777215);
        assert_eq!(b.to_rgba8(), RGBA8::new(&0, &0, &0, &0));

        // Ici on doit arrondir à 1.
        let c = RGBA32::new(&16777217, &16777217, &16777217, &16777217);
        assert_eq!(c.to_rgba8(), RGBA8::new(&1, &1, &1, &1));

        // Ici on doit arrondir à 254 => 2^32 - 2^24 - 1
        let d = RGBA32::new(&4278190079, &4278190079, &4278190079, &4278190079);
        assert_eq!(d.to_rgba8(), RGBA8::new(&254, &254, &254, &254));

        // Ici on oit arrondir à 255
        let e = RGBA32::new(&4278190080, &4278190080, &4278190080, &4278190080);
        assert_eq!(e.to_rgba8(), RGBA8::new(&255, &255, &255, &255));
    }
}
