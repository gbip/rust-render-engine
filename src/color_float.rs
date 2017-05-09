use std::ops::{Mul, Add, AddAssign};

// Represente les types de couleur que l'on manipule.
/// Espace linéaire.
#[derive(Debug)]
pub struct LinearColor {
    internal_color: InternalColor,
}
/// Espace `sRGB`.
#[derive(Debug)]
pub struct RGBColor {
    internal_color: InternalColor,
}

impl LinearColor {
    pub fn get_internal_color(&self) -> &InternalColor {
        &self.internal_color
    }
    pub fn get_internal_color_mut(&mut self) -> &mut InternalColor {
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
        LinearColor { internal_color: InternalColor::default() }
    }
}

impl<'a, 'b> Add<&'a LinearColor> for &'b LinearColor {
    type Output = LinearColor;
    fn add(self, other: &LinearColor) -> Self::Output {
        LinearColor {
            internal_color: InternalColor::new(self.internal_color.r + other.internal_color.r,
                                               self.internal_color.g + other.internal_color.g,
                                               self.internal_color.b + other.internal_color.b,
                                               self.internal_color.a + other.internal_color.a),
        }
    }
}

impl<'a> AddAssign<&'a LinearColor> for LinearColor {
    fn add_assign(&mut self, other: &LinearColor) {
        *self = LinearColor {
            internal_color: InternalColor::new(self.internal_color.r + other.internal_color.r,
                                               self.internal_color.g + other.internal_color.g,
                                               self.internal_color.b + other.internal_color.b,
                                               self.internal_color.a + other.internal_color.a),
        };
    }
}

impl Mul<f32> for LinearColor {
    type Output = LinearColor;
    fn mul(self, other: f32) -> Self::Output {
        LinearColor {
            internal_color: InternalColor::new(self.internal_color.r * other,
                                               self.internal_color.b * other,
                                               self.internal_color.g * other,
                                               self.internal_color.a * other),
        }
    }
}

const GAMMA : f32 = 2.2;
const INV_GAMMA : f32 = 1 / GAMMA;

/// Conversion entre l'espace `sRGB` et l'espace linéaire.
impl Into<LinearColor> for RGBColor {
    fn into(self) -> LinearColor {
        LinearColor {
            internal_color: InternalColor::new(self.internal_color.r.pow(INV_GAMMA).min(1f32),
                                                self.internal_color.g.pow(INV_GAMMA).min(1f32),
                                                self.internal_color.b.pow(INV_GAMMA).min(1f32),
                                                self.internal_color.a.pow(INV_GAMMA).min(1f32)),
        }
    }
}

/// Conversion entre un espace linéaire et l'espace `sRGB`.
impl Into<RGBColor> for LinearColor {
    fn into(self) -> RGBColor {
        LinearColor {
            internal_color: InternalColor::new(self.internal_color.r.pow(GAMMA).min(1f32),
                                                self.internal_color.g.pow(GAMMA).min(1f32),
                                                self.internal_color.b.pow(GAMMA).min(1f32),
                                                self.internal_color.a.pow(GAMMA).min(1f32)),
        }
    }
}

impl RGBColor {
    /// Tronque les valeurs afin que toutes les composantes soient dans [0;1].
    fn clamp(&mut self) {
        if self.internal_color.r > 1f32 {
            self.internal_color.r = 1f32;
        }

        if self.internal_color.b > 1f32 {
            self.internal_color.b = 1f32;
        }

        if self.internal_color.g > 1f32 {
            self.internal_color.g = 1f32;
        }

        if self.internal_color.a > 1f32 {
            self.internal_color.a = 1f32;
        }
    }
}

// Deux constantes utiles pour passer d'un entier allant de 0 a 255 à un flot compris entre 0 et 1.
const STEP: f32 = 1f32 / 255f32;
const INV_STEP: f32 = 255f32;

impl Mul<f32> for RGBColor {
    type Output = RGBColor;
    fn mul(self, other: f32) -> Self::Output {
        RGBColor {
            internal_color: InternalColor {
                r: self.internal_color.r * other,
                g: self.internal_color.g * other,
                b: self.internal_color.b * other,
                a: self.internal_color.a * other,
            },
        }
    }
}

impl Into<(u8, u8, u8)> for RGBColor {
    fn into(self) -> (u8, u8, u8) {
        // On se raméne entre 0 et 255
        let r = 255f32.min(self.internal_color.r * INV_STEP);
        let g = 255f32.min(self.internal_color.g * INV_STEP);
        let b = 255f32.min(self.internal_color.b * INV_STEP);
        (r as u8, g as u8, b as u8)
    }
}

impl Into<RGBColor> for (u8, u8, u8) {
    fn into(self) -> RGBColor {
        RGBColor {
            internal_color: InternalColor::new(self.0 as f32 * STEP,
                                               self.1 as f32 * STEP,
                                               self.2 as f32 * STEP,
                                               1f32),
        }
    }
}

/// Trait implémenté par toutes les couleurs.
pub trait IsColor: ColorConversion {
    /// Accesseur en lecture sur la composante rouge.
    fn r(&self) -> f32;
    /// Accesseur en lecture sur la composante bleu.
    fn b(&self) -> f32;
    /// Accesseur en lecture sur la composante verte.
    fn g(&self) -> f32;
    /// Accesseur en lecture sur la composante alpha.
    fn a(&self) -> f32;
    /// Permet de créer une couleur represenant le noir absolu.
    fn new_black() -> Self;
    /// Permet de créer une couleur representant le blanc absolu.
    fn new_white() -> Self;
}

/// Represente la conversion entre différents espaces de couleurs.
pub trait ColorConversion {
    fn srgb_to_linear(self) -> Self;
    fn linear_to_srgb(self) -> Self;
}

/// Represente une couleur avec des champs utilisant des floats.
/// Plus pratique que les entiers pour les overflow.
#[derive(Debug)]
pub struct InternalColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorConversion for InternalColor {
    fn srgb_to_linear(self) -> InternalColor {
        unimplemented!()
    }
    fn linear_to_srgb(self) -> InternalColor {
        unimplemented!()
    }
}


impl IsColor for InternalColor {
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
        self.a
    }

    fn new_black() -> InternalColor {
        InternalColor::default()
    }

    fn new_white() -> InternalColor {
        InternalColor {
            r: 1f32,
            b: 1f32,
            g: 1f32,
            a: 1f32,
        }
    }
}

impl Default for InternalColor {
    /// Le constructeur par défaut construis une couleur noire.
    fn default() -> Self {
        InternalColor {
            r: 0f32,
            g: 0f32,
            b: 0f32,
            a: 0f32,
        }
    }
}

impl InternalColor {
    fn new(r: f32, g: f32, b: f32, a: f32) -> InternalColor {
        InternalColor {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[allow(float_cmp)]
    fn test_color_clamping() {
        let mut c: RGBColor = (255u8, 255u8, 255u8).into();
        c = c * 45f32;
        c.clamp();
        assert_eq!(c.internal_color.r, 1f32);
    }

    #[test]
    fn test_color_import_export_from_u8_u8_u8() {
        let c: RGBColor = (127u8, 127u8, 127u8).into();
        println!("{:?}", c);
        let k: (u8, u8, u8) = c.into();
        println!("{:?}", k);
        assert_eq!(k.0, 127u8);
    }
}
