use color::RGBA8;
use img::Image;
use std::collections::HashMap;

#[derive(Serialize,Deserialize,Debug,Clone)]
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
#[derive(Serialize,Deserialize,Debug,Clone)]
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