use color::RGBA8;
use io_utils;
use serde_json;
use img::Image;
use std::collections::HashMap;

#[derive(Serialize,Deserialize)]
pub struct TextureMap {
    map_path: String,
    tiling_x: f32,
    tiling_y: f32,
}

impl TextureMap {
    pub fn get_map_path(&self) -> &String {
        &self.map_path
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
        texture.get_pixel_at(((u *self.tiling_x * texture.width() as f32) as u32 % texture.width()),
                             ((v * self.tiling_y * texture.height() as f32) as u32 % texture.height()))

    }
}
#[derive(Serialize,Deserialize)]
pub struct Material {
    pub diffuse: RGBA8,
    pub specular: RGBA8,
    pub ambient: RGBA8,
    pub map_diffuse: TextureMap,
}

impl Material {
    pub fn new_empty() -> Material {
        Material {
            diffuse: RGBA8::new(&200u8, &200u8, &200u8, &255u8),
            specular: RGBA8::new(&255u8, &255u8, &255u8, &255u8),
            ambient: RGBA8::new_black(),
            map_diffuse: TextureMap::new("".to_string(), 1.0, 1.0),
        }
    }


    pub fn get_texture_paths(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        if self.map_diffuse.get_map_path() != "" {
            result.push(self.map_diffuse.get_map_path().clone());
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
        io_utils::write_string_to_file(&serde_json::to_string_pretty(&self).unwrap(),
                                       path.to_string())
            .expect("Could not save material");


    }
}
