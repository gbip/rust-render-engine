use ray::{Fragment, Ray};
use scene::World;
use color_float::LinearColor;
use renderer::TextureRegister;
use math::{VectorialOperations, Vector3f};

pub mod channel;
pub mod flat_material;
pub mod bsdf;
pub mod ambient_occlusion;

pub trait Material {
    fn get_color(&self,
                 frag: &Fragment,
                 ray: &Ray,
                 world: &World,
                 texture_data: Option<&TextureRegister>)
                 -> LinearColor;
}

/// Une structure de données qui contiens les méthodes permettant de passer d'un système de
/// coordonnées cartésien à un système de coordonnées dont les vecteurs de bases sont :
/// `n` - le vecteur normal à la surface (normale géométrique), de coordonnées (0,0,1)
/// `t` - un vecteur tangent à la surface
/// `s` - un autre vecteur tangent à la surface tel que `s` soit orthogonal à `t`
/// Ce système de coordonnées est le système de coordonnées de réflexion locale.
pub struct ShadingCoordinateSystem {
    n: Vector3f,
    t: Vector3f,
    s: Vector3f,
}

impl ShadingCoordinateSystem {
    /// Permet de créer un système de coordonnées de reflexion locale à partir d'un fragment.
    pub fn new_from_frag(frag: &Fragment) -> Self {
        let n: Vector3f = frag.du.cross_product_ref(&frag.dv);
        ShadingCoordinateSystem {
            n: n,
            t: frag.du,
            s: frag.dv,
        }
    }

    /// Permet de transformer un vecteur exprimé dans le repère cartésien du monde, en un vecteur
    /// exprimé dans le repère `self`.
    pub fn world_into_local_space(&self, u: &Vector3f) -> Vector3f {
        Vector3f::new(u.dot_product_ref(&self.s),
                      u.dot_product_ref(&self.t),
                      u.dot_product_ref(&self.n))
    }

    /// Permet de transformer un vecteur exprimé dans le système de coordonnée `self` dans le
    /// système de coordonnée cartésien du monde.
    pub fn local_into_world_space(&self, u: &Vector3f) -> Vector3f {
        Vector3f::new(self.s.x * u.x + self.t.x * u.y + self.n.x * u.z,
                      self.s.y * u.x + self.t.y * u.y + self.n.y * u.z,
                      self.s.z * u.z + self.t.z * u.y + self.n.z * u.z)
    }
}
