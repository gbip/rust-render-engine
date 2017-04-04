/// Un fichier qui contiens une texture qui affiche l'occlusion ambiante.

use tools::monte_carlo;
use scene::World;
use renderer::TextureRegister;
use color::RGBA32;
use ray::Fragment;
use material::channel::Texture;

#[derive(Serialize,Deserialize)]
pub struct AmbientOcclusionMap {
    /// La distance à partir de laquelle on arrête de regarder l'occlusion ambiante
    max_range: f32,

    /// Le nombre de samples utilisé pour calculer l'occlusion ambiante
    samples: u32,

    /// ? J'ai oublié à quoi il sers, si tu lis ce commentaire, supprime ce champ.
    radius: f32,
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
        /*
        let shading_coords : ShadingCoordinateSystem = ShadingCoordinateSystem::new_from_frag(frag);
        let samples : Vec<Vector3f> = sample_uniform_hemisphere(shading_coords,frag,self.samples);
        let mut rays : Vec<Ray> = vec!();
        for point in samples {
            let mut ray = Ray::new(frag.position,point-frag.position);
            ray.max_t = self.max_range;
        }
        let mut contributions : u32 = 0;
        for ray in rays {
            if  world.is_occluded(ray);
            contributions += 1;
        }
        let grey_value = contributions/????
        */












        unimplemented!()
    }
}
