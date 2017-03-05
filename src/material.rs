use color::RGBA8;
use io_utils;
use serde_json;
use img::Image;
use std::collections::HashMap;

#[derive(Serialize,Deserialize,Debug)]
pub struct TextureMap {
    map_path: String,
    tiling_x: f32,
    tiling_y: f32,
}

impl TextureMap {
    pub fn get_map_path(&self) -> String {
        self.map_path.clone()
    }
    pub fn new(texture_path: String, tiling_x: f32, tiling_y: f32) -> Self {
        TextureMap {
            map_path: texture_path,
            tiling_x: tiling_x,
            tiling_y: tiling_y,
        }
    }

    pub fn get_color(&self,
                     u: f32,
                     v: f32,
                     texture_registry: &HashMap<String, Image<RGBA8>>)
                     -> RGBA8 {
        let texture = &texture_registry.get(self.map_path.as_str()).unwrap();
        texture.get_pixel_at(((u * self.tiling_x * texture.width() as f32) as u32 % texture.width()),
                             ((v * self.tiling_y * texture.height() as f32) as u32 % texture.height()))

    }
    pub fn new_empty() -> Self {
        Self {
            tiling_x: 1.0,
            tiling_y: 1.0,
            map_path: "/empty/map/path".to_string(),
        }
    }
}

// Représente un canal de couleur : soit c'est une texture, soit c'est une couleur
#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
pub enum Channel {
    Solid { color: RGBA8 },
    Texture { texture: TextureMap },
}

impl Channel {
    // Renvoies la couleur
    // A faire : personnaliser les messages en fonction de l'erreur de l'utilisateur et reconnaître
    // les cas où on a un problème dans le json
    pub fn get_color(&self,
                     u: Option<f32>,
                     v: Option<f32>,
                     texture_registry: Option<&HashMap<String, Image<RGBA8>>>)
                     -> RGBA8 {

        match (u, v, texture_registry, self) {
            (Some(u), Some(v), Some(texture_registry), &Channel::Texture { ref texture }) => {
                texture.get_color(u, v, texture_registry)
            }
            (None, None, None, &Channel::Solid { color }) => color,
            _ => panic!("Error get_color"),
        }
    }

    pub fn is_solid(&self) -> bool {
        match *self {
            Channel::Solid { .. } => true,
            _ => false,
        }
    }
    pub fn is_texture(&self) -> bool {
        match *self {
            Channel::Texture { .. } => true,
            _ => false,
        }
    }
    pub fn get_texture_path(&self) -> String {
        match *self {
            Channel::Texture { ref texture } => texture.get_map_path(),
            _ => panic!("Erreur, ce n'est pas un canal de texture"),
        }
    }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Material {
    pub diffuse: Channel,
    pub specular: Channel,
    pub ambient: Channel,
}

impl Material {
    pub fn new_empty() -> Material {
        Material {
            diffuse: Channel::Solid { color: RGBA8::new(&200u8, &200u8, &200u8, &255u8) },
            specular: Channel::Solid { color: RGBA8::new(&255u8, &255u8, &255u8, &255u8) },
            ambient: Channel::Solid { color: RGBA8::new_black() },
        }
    }

    pub fn channels(&self) -> Vec<&Channel> {
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

    pub fn read_from_file(pathname: &str) -> Result<Material, String> {
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
