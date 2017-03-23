use color::{RGBA8, RGBA32};
use io_utils;
use serde_json;
use material::channel::Channel;
use material::Material;
use scene::World;
use render::TextureRegister;
use ray::Fragment;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct FlatMaterial {
    pub diffuse: Channel,
    pub specular: Channel,
    pub ambient: Channel,
}

impl FlatMaterial {
    pub fn new_empty() -> FlatMaterial {
        FlatMaterial {
            diffuse: Channel::Solid { color: RGBA8::new(&200u8, &200u8, &200u8, &255u8) },
            specular: Channel::Solid { color: RGBA8::new(&255u8, &255u8, &255u8, &255u8) },
            ambient: Channel::Solid { color: RGBA8::new_black() },
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
                 _: &World,
                 texture_data: Option<(f32, f32, &TextureRegister)>)
                 -> RGBA32 {


        let (u, v, tex_reg) = match texture_data {
            Some(data) => (Some(data.0), Some(data.1), Some(data.2)),
            None => (None, None, None),
        };

        self.diffuse.get_color(frag, u, v, tex_reg)


    }
}
