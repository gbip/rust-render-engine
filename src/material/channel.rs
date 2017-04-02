use color::{RGBA8, RGBA32};
use img::Image;
use std::collections::HashMap;
use ray::Fragment;
use math::VectorialOperations;

/** Represente le fait qu'une structure de donnée soit une texture utilisable dans un canal d'un
 * matériau  */
pub trait Texture {
    fn get_color(&self,
                 frag: &Fragment,
                 u: Option<f32>,
                 v: Option<f32>,
                 texture_registry: Option<&HashMap<String, Image<RGBA8>>>)
                 -> RGBA32;
}


/** Represente une map qui est une texture */
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
    pub fn new_empty() -> Self {
        Self {
            tiling_x: 1.0,
            tiling_y: 1.0,
            map_path: "/empty/map/path".to_string(),
        }
    }
}

impl Texture for TextureMap {
    fn get_color(&self,
                 _: &Fragment,
                 u: Option<f32>,
                 v: Option<f32>,
                 texture_registry: Option<&HashMap<String, Image<RGBA8>>>)
                 -> RGBA32 {

        let texture = &texture_registry
                           .unwrap()
                           .get(self.map_path.as_str())
                           .unwrap();
        texture
            .get_pixel_at(((u.unwrap() * self.tiling_x * texture.width() as f32) as u32 %
                           texture.width()),
                          ((v.unwrap() * self.tiling_y * texture.height() as f32) as u32 %
                           texture.height()))
            .to_rgba32()

    }
}

/** Represente une texture qui prends en entrée de la géométrie et qui retourne une couleur en
 * fonction de la normale*/
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct NormalMap {}

impl Texture for NormalMap {
    fn get_color(&self,
                 frag: &Fragment,
                 _: Option<f32>,
                 _: Option<f32>,
                 _: Option<&HashMap<String, Image<RGBA8>>>)
                 -> RGBA32 {
        let normal = frag.normal / frag.normal.norm();

        //let mut white = RGBA32::new_white();
        let mut white = RGBA8::new(&128, &128, &128, &128);
        white.r = (white.r as f32 * normal.x) as u8;
        white.g = (white.g as f32 * normal.y) as u8;
        white.b = (white.b as f32 * normal.z) as u8;
        white.to_rgba32()

    }
}

// Représente un canal de couleur : soit c'est une texture, soit c'est une couleur
#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged)]
pub enum Channel {
    Solid { color: RGBA8 },
    TextureMap { texture: TextureMap },
    NormalMap { normal: NormalMap },
}

impl Channel {
    // Renvoies la couleur
    // A faire : personnaliser les messages en fonction de l'erreur de l'utilisateur et reconnaître
    // les cas où on a un problème dans le json
    // TODO Normalement il n'y a plus de besoin de faire du pattern_matching...
    pub fn get_color(&self,
                     frag: &Fragment,
                     u: Option<f32>,
                     v: Option<f32>,
                     texture_registry: Option<&HashMap<String, Image<RGBA8>>>)
                     -> RGBA32 {

        match (u, v, texture_registry, self) {
            (Some(u), Some(v), Some(texture_registry), &Channel::TextureMap { ref texture }) => {
                texture.get_color(frag, Some(u), Some(v), Some(texture_registry))
            }
            (None, None, None, &Channel::NormalMap { ref normal }) => {
                normal.get_color(frag, None, None, None)
            }

            (None, None, None, &Channel::Solid { ref color }) => color.to_rgba32(),
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
            Channel::TextureMap { .. } => true,
            _ => false,
        }
    }
    pub fn get_texture_path(&self) -> String {
        match *self {
            Channel::TextureMap { ref texture } => texture.get_map_path(),
            _ => panic!("Erreur, ce n'est pas un canal de texture"),
        }
    }
}
