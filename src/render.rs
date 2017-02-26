use scene;
use img::Image;
use color::{RGBA8, RGBA32};
use color;
use math::Vector3f;
use ray::{Ray, Fragment, Surface};
use obj3D::Object;
use std::collections::HashMap;
use std::fmt;

// Le ratio n'est pas enregistré à la deserialization, il faut penser à appeler compute_ratio()
// pour avoir un ratio autre que 0.
#[derive(Serialize,Deserialize)]
pub struct Renderer {
    // Colonne
    res_x: usize,

    // Ligne
    res_y: usize,

    #[serde(skip_serializing,skip_deserializing)]
    ratio: f32,

    subdivision_sampling: u32,

    background_color: RGBA8,

    #[serde(skip_serializing, skip_deserializing, default = "HashMap::new")]
    textures: HashMap<String, Image<RGBA8>>,
}


impl Renderer {
    pub fn new(res_x: usize, res_y: usize) -> Self {
        Renderer {
            res_x: res_x,
            res_y: res_y,
            ratio: (res_x as f32 / res_y as f32),
            background_color: RGBA8::new_black(),
            textures: HashMap::new(),
            subdivision_sampling: 1,
        }
    }

    pub fn set_resolution(&mut self, res_x: usize, res_y: usize) {
        self.res_x = res_x;
        self.res_y = res_y;
        self.ratio = res_x as f32 / res_y as f32;

    }

    pub fn compute_ratio(&mut self) {
        self.ratio = self.res_x as f32 / self.res_y as f32;
    }

    pub fn load_textures(&mut self, world: &scene::World) {
        let mut textures: HashMap<String, Image<RGBA8>> = HashMap::new();
        for object in world.objects() {
            let texture_paths = object.material().get_texture_paths();

            for path in texture_paths {
                let path_str = String::from(path.as_str());
                println!("Ajout de la texture {}", path);
                textures.entry(path)
                    .or_insert_with(|| Image::<RGBA8>::read_from_file(path_str.as_str()));
            }
        }

        self.textures = textures;
    }

    pub fn free_textures(&mut self) {
        self.textures = HashMap::new();
    }

    pub fn show_information(&self) {
        println!("Resolution is : {} x {}", self.res_x, self.res_y);
    }

    pub fn calculate_ray_intersection<'a>(&self,
                                          objects: &[&'a Object], // TODO Changer en raytree
                                          mut ray: &mut Ray)
                                          -> (Option<Fragment>, Option<&'a Object>) {

        let mut fragment: Option<Fragment> = None;
        let mut obj: Option<&Object> = None;

        for object in objects {
            let points: Vec<Option<Fragment>> = object.triangles()
                .map(|tri| tri.get_intersection(&mut ray))
                .filter(|point| point.is_some())
                .collect();

            match points.len() {
                0 => {}
                n => {
                    fragment = points[n - 1]; // TODO ici le fragment est copié. Chercher une façon de juste le déplacer.
                    obj = Some(object);
                }
            }
        }

        (fragment, obj)
    }

    /** Calcule les rayons à lancer pour le canvas passé en paramètres.
    Calcule ensuite la couleur finale de chaque rayon et stocke le résultat dans
    le canvas passé en paramètres. */
    pub fn emit_rays(&self,
                     world: &scene::World,
                     camera: &scene::Camera,
                     canvas: &mut Canvas,
                     ray_density_x: u32,
                     ray_density_y: u32) {

        // On crée les rayons à emmettre
        let mut rays: Vec<Ray> = vec![];

        for x in 0..ray_density_x {
            for y in 0..ray_density_y {
                let target = canvas.origin + canvas.e1 * ((x as f32 + 0.5) / ray_density_x as f32) +
                             canvas.e2 * ((y as f32 + 0.5) / ray_density_y as f32);

                rays.push(Ray::new(camera.world_position, target - camera.world_position));
            }
        }

        canvas.colors.clear();
        let objects =
            world.objects().iter().filter(|obj| obj.is_visible()).collect::<Vec<&Object>>();

        // On calcule chaque point d'intersection
        for mut ray in rays {
            let (opt_frag, opt_obj) = self.calculate_ray_intersection(&objects, &mut ray);

            // Comme pour chaque rayon on ajoute un unique fragment, l'ordre est conservé et on peut retrouver la position de chaque fragment
            // dans l'espace à partir de sa position dans la liste.
            // A faire peut-être : stocker les fragments directement en fonction de leur position dans l'image.
            match (opt_frag, opt_obj) {
                // Le dernier fragment trouvé est celui qui correspond à l'objet le plus en avant de la scène par rapport à la caméra
                (Some(fragment), Some(object)) => {
                    let mut color: RGBA32 = RGBA32::new_black();
                    match fragment.tex {
                        Some(tex_coord) => {
                            color = object.material()
                                .diffuse
                                .get_color(Some(tex_coord.x),
                                           Some(tex_coord.y),
                                           Some(&self.textures))
                                .to_rgba32();
                        }
                        None => {
                            color =
                                object.material().diffuse.get_color(None, None, None).to_rgba32();

                        }

                    }
                    /*
                    if let Some(tex_coord) = fragment.tex {
                        color = object.material()
                            .diffuse
                            .get_color(Some(tex_coord.x), Some(tex_coord.y), Some(&self.textures))
                            .to_rgba32();
                    }
                    else {

                    }
  */
                    canvas.colors.push(color);
                }
                _ => {
                    canvas.colors.push(self.background_color.to_rgba32());
                }
            }
        }
    }

    fn create_canvas(&self, camera: &scene::Camera) -> Vec<Vec<Canvas>> {
        let mut canvas: Vec<Vec<Canvas>> = vec![];

        // On crée les "canvas"
        let (origin, vec1, vec2) = camera.get_canvas_base(self.ratio);
        let e1 = vec1 / self.res_x as f32;
        let e2 = vec2 / self.res_y as f32;

        // Pas besoin du -1 car Rust s'arrête tout seul à -1.
        for x in 0..(self.res_x) {
            let mut line: Vec<Canvas> = vec![];
            for y in 0..(self.res_y) {
                let x1 = x as f32 / self.res_x as f32;
                let y1 = y as f32 / self.res_y as f32;

                line.push(Canvas::new(origin + vec1 * x1 + vec2 * y1, e1, e2));
            }
            canvas.push(line);
        }
        canvas

    }

    pub fn initialize(&mut self, world: &scene::World) {
        self.compute_ratio();
        self.load_textures(world);
    }

    pub fn render(&self, world: &scene::World, camera: &scene::Camera) -> Image<RGBA32> {

        let mut canvas: Vec<Vec<Canvas>> = self.create_canvas(camera);

        for line in &mut canvas {
            for pixel in &mut line.iter_mut() {
                self.emit_rays(world,
                               camera,
                               pixel,
                               self.subdivision_sampling,
                               self.subdivision_sampling);
            }
        }

        let temp_result: Vec<Vec<RGBA32>> = canvas.into_iter()
            .map(|line| line.into_iter().map(|frag| frag.get_average_color()).collect())
            .collect();

        Image::<RGBA32>::from_vec_vec(&temp_result)
    }
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Renderer : resolution = {}x{}, background_color = {:?}",
               self.res_x,
               self.res_y,
               self.background_color)
    }
}


/** Représente un rectangle en trois dimensions, correspondant à un pixel sur l'image finale à rendre.
Ce carré est décrit par une origine, et deux vecteurs directeurs. */
pub struct Canvas {
    origin: Vector3f,
    e1: Vector3f,
    e2: Vector3f,
    colors: Vec<RGBA32>,
}


impl Canvas {
    pub fn new(origin: Vector3f, e1: Vector3f, e2: Vector3f) -> Canvas {
        Canvas {
            origin: origin,
            e1: e1,
            e2: e2,
            colors: vec![],
        }
    }
    pub fn get_average_color(&self) -> RGBA32 {
        color::make_average_color(&self.colors)
    }
}
