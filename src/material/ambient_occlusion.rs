/// Un fichier qui contiens une texture qui affiche l'occlusion ambiante.

use tools::monte_carlo;
use scene::World;
use renderer::TextureRegister;
use color::RGBA32;
use ray::{Ray, Fragment};
use material::channel::Texture;
use math::Vector3f;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct AmbientOcclusionMap {
    /// La distance à partir de laquelle on arrête de regarder l'occlusion ambiante
    radius: f32,

    /// Le nombre de samples utilisé pour calculer l'occlusion ambiante
    samples: u32,
}

impl Texture for AmbientOcclusionMap {
    fn get_color(&self,
                 frag: &Fragment,
                 _: Option<f32>,
                 _: Option<f32>,
                 _: Option<&TextureRegister>,
                 world: &World)
                 -> RGBA32 {
        // Pseudo code :
        let samples: Vec<Vector3f> = monte_carlo::sample_uniform_hemisphere(self.samples, frag);
        let mut rays: Vec<Ray> = vec![];
        for point in samples {
            let mut ray = Ray::new(frag.position, point - frag.position);
            ray.max_t = self.radius;
            rays.push(ray);
        }
        let mut contributions: u32 = 0;
        for ray in &mut rays {
            if world.is_occluded(ray) {
                //println!("max_t is : {}",ray.max_t);
                //println!("radius is : {}",self.radius);
                //if ray.max_t < 0.01 {
                //    println!("self intersect, shit");
                //}
                contributions += 1;
            }
        }
        let mut result = RGBA32::new_white();
        let greyness: f32 = 1.0 - contributions as f32 / self.samples as f32;
        result.r = (result.r as f32 * greyness) as u32;
        result.b = (result.b as f32 * greyness) as u32;
        result.g = (result.g as f32 * greyness) as u32;
        result
    }
}
