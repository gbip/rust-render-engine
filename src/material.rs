use color::RGBA8;
use io_utils;
use serde_json;

#[derive(Serialize,Deserialize)]
pub struct Material {
    diffuse: RGBA8,
    specular: RGBA8,
    ambient: RGBA8,
    map_diffuse: String,
}

impl Material {
    pub fn new_empty() -> Material {
        Material {
            diffuse: RGBA8::new(&200u8, &200u8, &200u8, &255u8),
            specular: RGBA8::new(&255u8, &255u8, &255u8, &255u8),
            ambient: RGBA8::new_black(),
            map_diffuse: "".to_string(),
        }
    }

    pub fn get_texture_paths(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        if self.map_diffuse != "" {
            result.push(String::from(self.map_diffuse.as_str()));
        }

        result
    }

    pub fn read_from_file(pathname: &str) -> Result<Material, String> {
        match io_utils::open_file_as_string(pathname) {
            Ok(file_str) => match serde_json::from_str(file_str.as_str()) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("Serde Error !".to_string()),
            },
            Err(_) => Err("IO Error".to_string()), // TODO personaliser les messages d'erreur
        }
    }
}
