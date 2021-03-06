use color_float::{RGBColor, LinearColor, FloatColor, Color};
use img::{Image, RGBAPixel};
use std::collections::HashMap;
use ray::Fragment;
use math::VectorialOperations;
use scene::World;
use material::ambient_occlusion::AmbientOcclusionMap;

/** Represente le fait qu'une structure de donnée soit une texture utilisable dans un canal d'un
 * matériau  */
pub trait Texture {
    fn get_color(&self,
                 frag: &Fragment,
                 u: Option<f32>,
                 v: Option<f32>,
                 texture_registry: Option<&HashMap<String, Image<RGBAPixel>>>,
                 world: &World)
                 -> LinearColor;
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
                 texture_registry: Option<&HashMap<String, Image<RGBAPixel>>>,
                 _: &World)
                 -> LinearColor {

        let texture = &texture_registry
                           .unwrap()
                           .get(self.map_path.as_str())
                           .unwrap();
        texture
            .get_pixel_at(((u.unwrap() * self.tiling_x * texture.width() as f32) as u32 %
                           texture.width()),
                          ((v.unwrap() * self.tiling_y * texture.height() as f32) as u32 %
                           texture.height()))
            .into()

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
                 _: Option<&HashMap<String, Image<RGBAPixel>>>,
                 _: &World)
                 -> LinearColor {
        let normal = frag.normal / frag.normal.norm();

        LinearColor::new(FloatColor::new(0.5 * (1f32 + normal.x),
                                         0.5 * (1f32 + normal.y),
                                         0.5 * (1f32 + normal.z)))
    }
}

// Représente un canal de couleur : soit c'est une texture, soit c'est une couleur
#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged)]
pub enum Channel {
    Solid { color: RGBColor },
    TextureMap { texture: TextureMap },
    NormalMap { normal: NormalMap },
    AmbientOcclusionMap { ambient_occlusion: AmbientOcclusionMap, },
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
                     texture_registry: Option<&HashMap<String, Image<RGBAPixel>>>,
                     world: &World)
                     -> LinearColor {

        match (u, v, texture_registry, self) {
            (Some(u), Some(v), Some(texture_registry), &Channel::TextureMap { ref texture }) => {
                texture.get_color(frag, Some(u), Some(v), Some(texture_registry), world)
            }
            (None, None, None, &Channel::NormalMap { ref normal }) => {
                normal.get_color(frag, None, None, None, world)
            }
            (None, None, None, &Channel::AmbientOcclusionMap { ref ambient_occlusion }) => {
                ambient_occlusion.get_color(frag, None, None, None, world)
            }
            (_, _, _, &Channel::Solid { color }) => color.into(),

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
