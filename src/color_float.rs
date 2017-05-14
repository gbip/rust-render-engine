use std::ops::{Mul, Add, AddAssign, Div};
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

// Represente les types de couleur que l'on manipule.
/// Espace linéaire.
#[derive(Debug, Copy, Clone)]
pub struct LinearColor {
    internal_color: FloatColor,
}
/// Espace `sRGB`.
#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    internal_color: FloatColor,
}

impl LinearColor {
    pub fn new_white() -> LinearColor {
        LinearColor {internal_color: FloatColor::new_white(),}
    }

    pub fn new_black() -> LinearColor {
        LinearColor {internal_color: FloatColor::new_black(),}
    }

    pub fn new(color : FloatColor) -> LinearColor {
        LinearColor { internal_color: color }
    }

    pub fn clamp(&mut self) {
        self.internal_color.clamp();
    }

    pub fn get_internal_color(&self) -> &FloatColor {
        &self.internal_color
    }
    pub fn get_internal_color_mut(&mut self) -> &mut FloatColor {
        &mut self.internal_color
    }

    pub fn make_average_color(colors: &[LinearColor]) -> LinearColor {

        let color_number = colors.len();
        let mut result: LinearColor = LinearColor::default();


        for color in colors {
            result += color;
        }

        result * (1f32 / color_number as f32)
    }
}

impl Default for LinearColor {
    fn default() -> LinearColor {
        LinearColor { internal_color: FloatColor::default() }
    }
}

impl RGBColor {
    /// Tronque les valeurs afin que toutes les composantes soient dans [0;1].
    pub fn clamp(&mut self) {
        self.internal_color.clamp();
    }
}

// Serialisation / deserialisation RGBColor

impl Serialize for RGBColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {
        unimplemented!()
    }
}

impl Deserialize for RGBColor {
    fn deserialize<S>(serializer: S) -> Result<Self, S::Error>
            where S: Deserializer {
        unimplemented!()
    }
}


// Operations de base sur les LinearColor

impl<'a, 'b> Add<&'a LinearColor> for &'b LinearColor {
    type Output = LinearColor;
    fn add(self, other: &LinearColor) -> Self::Output {
        LinearColor {
            internal_color: FloatColor::new(self.internal_color.r + other.internal_color.r,
                                               self.internal_color.g + other.internal_color.g,
                                               self.internal_color.b + other.internal_color.b),
        }
    }
}

impl<'a> AddAssign<&'a LinearColor> for LinearColor {
    fn add_assign(&mut self, other: &LinearColor) {
        self.internal_color = FloatColor::new(self.internal_color.r + other.internal_color.r,
                                               self.internal_color.g + other.internal_color.g,
                                               self.internal_color.b + other.internal_color.b);
    }
}

impl Mul<f32> for LinearColor {
    type Output = LinearColor;
    fn mul(self, other: f32) -> Self::Output {
        LinearColor {
            internal_color: FloatColor::new(self.internal_color.r * other,
                                               self.internal_color.b * other,
                                               self.internal_color.g * other),
        }
    }
}

impl Mul<LinearColor> for LinearColor {
    type Output = LinearColor;
    fn mul(self, other: LinearColor) -> Self::Output {
        LinearColor {
            internal_color: FloatColor::new(self.internal_color.r * other.internal_color.r,
            self.internal_color.g * other.internal_color.g,
        self.internal_color.b * other.internal_color.b)
        }
    }
}

impl Div<f32> for LinearColor {
    type Output = LinearColor;
    fn div(self, other: f32) -> Self::Output {
        LinearColor {
            internal_color: FloatColor::new(self.internal_color.r / other,
                                               self.internal_color.b / other,
                                               self.internal_color.g / other),
        }
    }
}

const GAMMA: f32 = 2.2;
const INV_GAMMA: f32 = 1f32 / GAMMA;

/// Conversion entre l'espace `sRGB` et l'espace linéaire.
impl Into<LinearColor> for RGBColor {
    fn into(self) -> LinearColor {
        LinearColor {
            internal_color: FloatColor::new(self.internal_color.r.powf(INV_GAMMA).min(1f32),
                                               self.internal_color.g.powf(INV_GAMMA).min(1f32),
                                               self.internal_color.b.powf(INV_GAMMA).min(1f32)),
        }
    }
}

/// Conversion entre un espace linéaire et l'espace `sRGB`.
impl Into<RGBColor> for LinearColor {
    fn into(self) -> RGBColor {
        RGBColor {
            internal_color: FloatColor::new(self.internal_color.r.powf(GAMMA).min(1f32),
                                               self.internal_color.g.powf(GAMMA).min(1f32),
                                               self.internal_color.b.powf(GAMMA).min(1f32)),
        }
    }
}

// Deux constantes utiles pour passer d'un entier allant de 0 a 255 à un flot compris entre 0 et 1.
const STEP: f32 = 1f32 / 255f32;
const INV_STEP: f32 = 255f32;

// TODO faire une macro
// conversion vers (u8, u8, u8)
impl Into<(u8, u8, u8)> for RGBColor {
    fn into(self) -> (u8, u8, u8) {
        // On se raméne entre 0 et 255
        self.internal_color.into()
    }
}

impl Into<RGBColor> for (u8, u8, u8) {
    fn into(self) -> RGBColor {
        RGBColor {
            internal_color: self.into(),
        }
    }
}

impl Into<(u8, u8, u8)> for LinearColor {
    fn into(self) -> (u8, u8, u8) {
        // On se raméne entre 0 et 255
        self.internal_color.into()
    }
}

impl Into<LinearColor> for (u8, u8, u8) {
    fn into(self) -> LinearColor {
        LinearColor {
            internal_color: self.into(),
        }
    }
}

// Conversion vers (u8, u8, u8, u8)
impl Into<(u8, u8, u8, u8)> for RGBColor {
    fn into(self) -> (u8, u8, u8, u8) {
        // On se raméne entre 0 et 255
        self.internal_color.into()
    }
}

impl Into<RGBColor> for (u8, u8, u8, u8) {
    fn into(self) -> RGBColor {
        RGBColor {
            internal_color: self.into(),
        }
    }
}

impl Into<(u8, u8, u8, u8)> for LinearColor {
    fn into(self) -> (u8, u8, u8, u8) {
        // On se raméne entre 0 et 255
        self.internal_color.into()
    }
}

impl Into<LinearColor> for (u8, u8, u8, u8) {
    fn into(self) -> LinearColor {
        LinearColor {
            internal_color: self.into(),
        }
    }
}

// Float color

impl Into<(u8, u8, u8)> for FloatColor {
    fn into(self) -> (u8, u8, u8) {
        // On se raméne entre 0 et 255
        let r = 255f32.min(self.r * INV_STEP);
        let g = 255f32.min(self.g * INV_STEP);
        let b = 255f32.min(self.b * INV_STEP);
        (r as u8, g as u8, b as u8)
    }
}

impl Into<FloatColor> for (u8, u8, u8) {
    fn into(self) -> FloatColor {
        FloatColor::new(self.0 as f32 * STEP,
                        self.1 as f32 * STEP,
                        self.2 as f32 * STEP)
    }
}

impl Into<(u8, u8, u8, u8)> for FloatColor {
    fn into(self) -> (u8, u8, u8, u8) {
        // On se raméne entre 0 et 255
        let r = 255f32.min(self.r * INV_STEP);
        let g = 255f32.min(self.g * INV_STEP);
        let b = 255f32.min(self.b * INV_STEP);
        (r as u8, g as u8, b as u8, 255u8)
    }
}

impl Into<FloatColor> for (u8, u8, u8, u8) {
    fn into(self) -> FloatColor {
        FloatColor::new(self.0 as f32 * STEP,
                        self.1 as f32 * STEP,
                        self.2 as f32 * STEP)
    }
}

/// Trait implémenté par toutes les couleurs.
pub trait Color: ColorConversion {
    /// Accesseur en lecture sur la composante rouge.
    fn r(&self) -> f32;
    /// Accesseur en lecture sur la composante bleu.
    fn b(&self) -> f32;
    /// Accesseur en lecture sur la composante verte.
    fn g(&self) -> f32;
    /// Accesseur en lecture sur la composante alpha.
    fn a(&self) -> f32;

    /// Tronque les valeurs des composantes de manière qu'elles soient toutes comprises entre 0 et 1
    fn clamp(&mut self);

    /// Permet de créer une couleur represenant le noir absolu.
    fn new_black() -> Self;
    /// Permet de créer une couleur representant le blanc absolu.
    fn new_white() -> Self;
    /// Permet de créer une couleur avec les composantes aux choix
    fn new(r: f32, g: f32, b: f32) -> Self;
}

/// Represente la conversion entre différents espaces de couleurs.
pub trait ColorConversion {
    fn srgb_to_linear(self) -> Self;
    fn linear_to_srgb(self) -> Self;
}

/// Represente une couleur avec des champs utilisant des floats.
/// Plus pratique que les entiers pour les overflow.
#[derive(Debug, Clone, Copy)]
pub struct FloatColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorConversion for FloatColor {
    fn srgb_to_linear(self) -> FloatColor {
        unimplemented!()
    }
    fn linear_to_srgb(self) -> FloatColor {
        unimplemented!()
    }
}


impl Color for FloatColor {
    fn r(&self) -> f32 {
        self.r
    }

    fn b(&self) -> f32 {
        self.b
    }

    fn g(&self) -> f32 {
        self.g
    }

    fn a(&self) -> f32 {
        1f32
    }

    fn clamp(&mut self) {
        if self.r > 1f32 {
            self.r = 1f32;
        }

        if self.b > 1f32 {
            self.b = 1f32;
        }

        if self.g > 1f32 {
            self.g = 1f32;
        }
    }

    fn new(r: f32, g: f32, b: f32) -> FloatColor {
        FloatColor {
            r: r,
            g: g,
            b: b,
        }
    }

    fn new_black() -> FloatColor {
        FloatColor::default()
    }

    fn new_white() -> FloatColor {
        FloatColor {
            r: 1f32,
            b: 1f32,
            g: 1f32,
        }
    }
}

impl Default for FloatColor {
    /// Le constructeur par défaut construit une couleur noire.
    fn default() -> Self {
        FloatColor {
            r: 0f32,
            g: 0f32,
            b: 0f32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[allow(float_cmp)]
    fn test_color_clamping() {
        let mut c: RGBColor = RGBColor { internal_color: FloatColor::new_white() };
        c = c * 45f32;
        c.clamp();
        assert_eq!(c.internal_color.r, 1f32);
    }

    #[test]
    fn test_color_import_export_from_u8_u8_u8() {
        let c = RGBColor {internal_color: (127u8, 127u8, 127u8).into()};
        println!("{:?}", c);
        let k: (u8, u8, u8) = c.get_internal_color().into();
        println!("{:?}", k);
        assert_eq!(k.0, 127u8);
    }
}
