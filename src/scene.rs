use std::vec::Vec;
use math::{Vector3, Vector3f, VectorialOperations};
use obj3D;
use obj3D::Object;
use std::fs::File;
use std::io::{Write, Read};
use std;
use serde_json;
use color::RGBA8;
use render::Renderer;
use std::time::Instant;

fn write_string_to_file(j: &str, file_name: String) -> std::io::Result<()> {
    let mut file = File::create(file_name).unwrap();
    file.write_all(j.as_bytes())
}

#[allow(unused_must_use)]
fn open_file_as_string(file: &str) -> String {
    let mut result: String = "".to_string();
    match File::open(file) {
        Ok(mut val) => val.read_to_string(&mut result),
        Err(e) => panic!("Error could not open file {}, the error is : {}", file, e),
    };
    result
}

// Une simple scène
#[derive(Serialize,Deserialize,Debug)]
pub struct Scene {
    pub world: World,
    pub renderer: Renderer,
}

impl Scene {
    // Charge la scène depuis un fichier "file"
    pub fn load_from_file(file: String) -> Self {
        println!("Loading scene from file : {} ", file);
        let file = open_file_as_string(file.as_str());
        let mut scene: Scene = match serde_json::from_str(file.as_str()) {
            Ok(val) => val,
            Err(e) => panic!("Error while loading world. Serde error is : {}", e),
        };
        scene.world.load_objects();
        scene.renderer.compute_ratio();
        scene
    }

    // Nouvelle scène vide, avec une résolution de base de 960x540
    pub fn new_empty() -> Self {
        Scene {
            world: World::new_empty(),
            renderer: Renderer::new(960, 540),
        }
    }

    // Ecris la structure de la scène dans le fichier "file" en JSON sans la géomètrie
    pub fn save_to_file(&self, file: String) {
        match write_string_to_file(&serde_json::to_string_pretty(&self).unwrap(), file) {

            Err(e) => println!("Could not save world. Error : {}", e),

            Ok(_) => println!("World sucessfully saved"),

        }
    }

    pub fn render_to_file(&self, file_path: String) {
        self.renderer.show_information();
        println!("Starting to render...");
        let now = Instant::now();
        let image = self.renderer.render(&self.world, self.world.get_camera(0));
        println!("Render done in {} seconds, writting result to file {}",
                 now.elapsed().as_secs(),
                 &file_path);
        image.write_to_file(file_path.as_str())
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Camera {
    /// The position fo the camera exprimed in the standard word space coordinates (where {0,0,0} is the
    /// center of the world)
    pub world_position: Vector3f,

    /// The position of the point at which the camera is aiming, in world space coordinates
    target_position: Vector3f,

    // The vector that represents the up direction
    up: Vector3f,

    // The horizontal field of view, exprimed in degrees.
    fov: f32,

    // La distance entre le canvas et l'origine de la caméra.
    clip: f32,
}

const DEFAULT_FOV: f32 = 70.0;
const DEFAULT_CLIP: f32 = 0.1;

impl Camera {
    pub fn new(position: Vector3f, target: Vector3f, up: Vector3f) -> Self {
        Camera {
            world_position: position,
            target_position: target,
            fov: DEFAULT_FOV,
            up: up,
            clip: DEFAULT_CLIP,
        }
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    pub fn get_canvas_base(&self, ratio: f32) -> (Vector3f, Vector3f, Vector3f) {
        let cam_vector = self.target_position - self.world_position;
        let e1_not_norm = cam_vector.cross_product(&self.up);

        let e1 = e1_not_norm / e1_not_norm.norm();
        let e3 = cam_vector / cam_vector.norm();
        let e2 = e1.cross_product(&e3);

        let fov_tan = (self.fov / 2.0).to_radians().tan();

        let vec1 = e1 * (fov_tan * 2.0 * self.clip);
        let vec2 = e2 * (fov_tan * 2.0 * self.clip / ratio);
        let origin = self.world_position + e3 * self.clip - vec2 / 2.0 - vec1 / 2.0;

        (origin, vec1, vec2)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct World {
    // Les vecteurs de base du monde. Le 3é vecteur indique la verticale.
    base_vector: [Vector3<f32>; 3],
    // Les différentes camera du monde
    cameras: Vec<Camera>,

    objects: Vec<obj3D::Object>,
}

impl World {
    // Ajoute une caméra dans le monde
    pub fn add_camera(self: &mut World, position: Vector3f, target: Vector3f) {
        self.cameras.push(Camera::new(position, target, self.base_vector[2]));
    }

    // Charge la géomètrie de tous les objets. Utilisé uniquement en fin de deserialization.
    fn load_objects(&mut self) {
        for obj in &mut self.objects {
            obj.initialize();
        }
    }

    // Génére un monde vide
    pub fn new_empty() -> World {
        let base_vector = [Vector3::new(1_f32, 0_f32, 0_f32),
                           Vector3::new(0_f32, 1_f32, 0_f32),
                           Vector3::new(0_f32, 0_f32, 1_f32)];
        World {
            base_vector: base_vector,
            cameras: vec![],
            objects: vec![],
        }
    }

    // Ajoute un objet dans le monde
    pub fn add_object(&mut self, color: RGBA8, pos: Vector3f, path: String, name: String) {
        self.objects.push(Object::new(color, pos, path, name));
    }

    pub fn get_camera(&self, cam_indice: usize) -> &Camera {
        self.cameras.get(cam_indice).expect("Out of bound camera index")
    }

    pub fn objects(&self) -> &Vec<obj3D::Object> {
        &self.objects
    }
}

#[cfg(test)]
mod test {
    use scene::Camera;
    use math::{Vector3f, VectorialOperations};

    #[test]
    fn test_camera_canvas_base() {
        let cam: Camera = Camera {
            world_position: Vector3f {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            },
            target_position: Vector3f {
                x: -1.0,
                y: -1.0,
                z: 4.0,
            },
            up: Vector3f {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            fov: 90.0,
            clip: 2.0_f32.sqrt(),
        };

        let (origin, vec1, vec2) = cam.get_canvas_base(1.0);

        assert!((origin -
                 Vector3f {
                x: 4.0,
                y: 2.0,
                z: 4.0 - 2.0_f32.sqrt(),
            })
            .norm() < 0.001);
        assert!((vec1 -
                 Vector3f {
                x: -2.0,
                y: 2.0,
                z: 0.0,
            })
            .norm() < 0.001);
        assert!((vec2 -
                 Vector3f {
                x: 0.0,
                y: -0.0,
                z: 2.0 * 2.0_f32.sqrt(),
            })
            .norm() < 0.001);
    }
}
