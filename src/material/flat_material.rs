use color_float::LinearColor;
use io_utils;
use serde_json;
use material::channel::Channel;
use material::Material;
use scene::World;
use renderer::TextureRegister;
use ray::{Fragment, Ray};
use math::VectorialOperations;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct FlatMaterial {
    pub diffuse: Channel,
    pub specular: Channel,
    pub ambient: Channel,
}

impl FlatMaterial {
    pub fn new_empty() -> FlatMaterial {
        FlatMaterial {
            diffuse: Channel::Solid { color: (200u8, 200u8, 200u8).into() },
            specular: Channel::Solid { color: (255u8, 255u8, 255u8).into() },
            ambient: Channel::Solid { color: (0u8, 0u8, 0u8).into() },
        }
    }

    fn channels(&self) -> Vec<&Channel> {
        vec![&self.diffuse, &self.specular, &self.ambient]
    }

    pub fn get_texture_paths(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for chan in &self.channels() {
            if chan.is_texture() {
                result.push(chan.get_texture_path());
            }

        }
        result
    }

    pub fn read_from_file(pathname: &str) -> Result<FlatMaterial, String> {
        match io_utils::open_file_as_string(pathname) {
            Ok(file_str) => {
                match serde_json::from_str(file_str.as_str()) {
                    Ok(val) => Ok(val),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()), // TODO personaliser les messages d'erreur
        }
    }

    pub fn save_to_file(&self, path: &str) {
        io_utils::write_string_to_file(&serde_json::to_string_pretty(&self).unwrap(), path)
            .expect("Could not save material");


    }
}


impl Material for FlatMaterial {
    fn get_color(&self,
                 frag: &Fragment,
                 _: &Ray,
                 world: &World,
                 texture_data: Option<&TextureRegister>)
                 -> LinearColor {

        let (u, v, tex_reg) = match (frag.tex, texture_data) {
            (Some(tex_coords), Some(texture_register)) => {
                (Some(tex_coords.x), Some(tex_coords.y), Some(texture_register))
            }
            _ => (None, None, None),
        };

        // Calcul de l'intensité totale
        let mut intensity = 0.0;
        let lights = world.lights();
        let light_count = lights.len();

        if light_count == 0 {
            intensity = 1.0;
        }

        for light in lights {
            let mut light_rays = light.as_trait().emit_rays(&frag.position, world);

            for light_ray in &mut light_rays {
                if !world.is_occluded(light_ray) {
                    let ray_vect = -light_ray.slope() / light_ray.slope().norm();
                    //let factor = cmp::max(&0.0, &ray_vect.dot_product(&frag.normal));
                    let factor = ray_vect.dot_product(&(frag.normal / frag.normal.norm()))
                        .abs();
                    intensity += factor * light.as_trait().intensity();
                }
            }
            /*if light.as_trait().visible(&frag.position, world) {
                intensity += 1.0 / light_count as f32;
            }*/
        }

        // Calcul de la couleur du matériau
        self.diffuse.get_color(frag, u, v, tex_reg, world) * intensity
    }
}
