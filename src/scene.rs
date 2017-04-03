use std::vec::Vec;
use math::{Vector3, Vector3f, VectorialOperations};
use geometry::obj3d::Object;
use light::LightObject;
use sampler::Sample;
use ray::Ray;
use io_utils;
use serde_json;
use renderer::render::Renderer;
use std::time::Instant;
use ray::Surface;

// Une simple scène
#[derive(Serialize,Deserialize,Debug)]
pub struct Scene {
    pub world: World,
    pub renderer: Renderer,
}

impl Scene {
    // Charge la scène depuis un fichier "file"
    pub fn load_from_file(file: &str) -> Self {
        println!("Loading scene from file : {} ", file);
        let mut scene: Scene = match io_utils::open_file_as_string(file) {
            Ok(file) => {
                match serde_json::from_str(file.as_str()) {
                    Ok(val) => val,
                    Err(e) => panic!("Error while loading world. {}", e),
                }
            }
            Err(e) => panic!("Error while reading file {} : {}", file, e),
        };
        scene.world.load_objects();
        scene.renderer.initialize(&scene.world);
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
    pub fn save_to_file(&self, file: &str) {
        match io_utils::write_string_to_file(&serde_json::to_string_pretty(&self).unwrap(), file) {

            Err(e) => println!("Could not save world. Error : {}", e),

            Ok(_) => println!("World sucessfully saved"),

        }
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.renderer.show_information();
        println!("Starting to render...");
        let now = Instant::now();
        let image = self.renderer
            .render(&self.world, self.world.get_camera(0));
        println!("Render done in {} s, writting result to file {}",
                 now.elapsed().as_secs() as f64 + (now.elapsed().subsec_nanos() as f64 *
                     (1.0/1_000_000_000_f64)),
                 &file_path,);
        image.write_to_file(file_path)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Camera {
    /// The position fo the camera exprimed in the standard word space coordinates (where {0,0,0}
    /// is the center of the world)
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

    /** Donne un repère pour placer le cadre de la caméra. Le premier point
    correspond à l'origine en haut à gauche, les deux autres aux vecteurs
    x et y qui définissent les dimensions et orientations du cadre respectivement */
    pub fn get_canvas_base(&self, ratio: f32) -> (Vector3f, Vector3f, Vector3f) {
        let cam_vector = self.target_position - self.world_position;
        let e1_not_norm = cam_vector.cross_product(&self.up);

        let e1 = e1_not_norm / e1_not_norm.norm();
        let e3 = cam_vector / cam_vector.norm();
        let e2 = e3.cross_product(&e1);

        let fov_tan = (self.fov / 2.0).to_radians().tan();

        let vec1 = e1 * (fov_tan * 2.0 * self.clip);
        let vec2 = e2 * (fov_tan * 2.0 * self.clip / ratio);
        let origin = self.world_position + e3 * self.clip - vec2 / 2.0 - vec1 / 2.0;

        (origin, vec1, vec2)
    }

    // Crée un rayon dont la direction est déterminé par les coordonnées du sample
    // passé en paramètres.
    pub fn create_ray_from_sample(&self,
                                  sample: &Sample,
                                  ratio: f32,
                                  sample_res_x: f32,
                                  sample_res_y: f32)
                                  -> Ray {
        // TODO ici on fait un appel à get_canvas_basis pour chaque sample
        let (origin, e1, e2) = self.get_canvas_base(ratio);
        let sample_coord = sample.position();
        let target = origin + e1 * sample_coord.x / sample_res_x +
                     e2 * sample_coord.y / sample_res_y;
        Ray::new(self.world_position, target - self.world_position)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct World {
    // Les vecteurs de base du monde. Le 3é vecteur indique la verticale.
    base_vector: [Vector3<f32>; 3],
    // Les différentes camera du monde
    cameras: Vec<Camera>,

    objects: Vec<Object>,

    lights: Vec<LightObject>,
}

impl World {
    // Ajoute une caméra dans le monde
    pub fn add_camera(self: &mut World, position: Vector3f, target: Vector3f) {
        self.cameras
            .push(Camera::new(position, target, self.base_vector[2]));
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
            lights: vec![],
        }
    }

    // Ajoute un objet dans le monde
    pub fn add_object(&mut self, pos: Vector3f, path: String, name: String) {
        self.objects.push(Object::new(pos, path, name));
    }

    pub fn get_camera(&self, cam_indice: usize) -> &Camera {
        self.cameras
            .get(cam_indice)
            .expect("Out of bound camera index")
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    // Represente le fait qu'un point soit visible par un autre : on revoie true si le rayon
    // n'intersecte aucun triangle.
    pub fn is_occluded(&self, ray: &mut Ray) -> bool {
        for obj in &self.objects {
            if obj.fast_intersection(ray) {
                return true;
            }
            continue;
        }
        false
    }

    pub fn lights(&self) -> &Vec<LightObject> {
        &self.lights
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
                         z: 4.0 + 2.0_f32.sqrt(),
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
                         y: 0.0,
                         z: -2.0 * 2.0_f32.sqrt(),
                     })
                    .norm() < 0.001);
    }
}
