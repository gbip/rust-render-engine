use scene;
use img::Image;
use color::{RGBA8, RGBA32};
use ray::{Ray, Intersection};
use sampler::{DefaultSampler, Sampler};
use geometry::obj3d::Object;
use std::collections::HashMap;
use std::fmt;
use filter::{Filter, filters};
use renderer::Pixel;
use renderer::block::Block;

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
        for obj in world.objects() {
            let texture_paths = obj.material().get_texture_paths();

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

    pub fn calculate_ray_intersection<'b>(&self,
                                          objects: &[&'b Object], // TODO Changer en raytree
                                          mut ray: &mut Ray)
                                          -> Option<Intersection<'b>> {

        let mut intersection_point: Option<Intersection> = None;
        for object in objects {
            // TODO : Peut être virer le branching ici ?
            // TODO : Regarder le geometry/intersection.rs dans tray
            if object.bounding_box().intersects(ray) {
                if let Some(point) = object.get_intersection_point(ray) {
                    intersection_point = Some(point);
                }
            }
        }
        intersection_point
    }

    /** Calcule les rayons à lancer pour le canvas passé en paramètres.
    Calcule ensuite la couleur finale de chaque rayon et stocke le résultat dans
    le canvas passé en paramètres. */
    pub fn calculate_rays(&self, world: &scene::World, camera: &scene::Camera, pixel: &mut Pixel) {

        let objects = world.objects()
            .iter()
            .filter(|bbox| bbox.is_visible())
            .collect::<Vec<&Object>>();

        for sample in &mut pixel.samples {
            // On récupère le rayon à partir du sample
            let mut ray = camera.create_ray_from_sample(self.ratio, sample);

            // CALCUL DE LA COULEUR DU RAYON (TODO à mettre ailleurs)

            let point = self.calculate_ray_intersection(&objects, &mut ray);

            // On détermine la couleur du rayon, simplement à partir du fragment retourné et
            // du matériau associé à l'objet intersecté.
            match point {
                Some(p) => {
                    sample.color = p.get_point_color(world, &self.textures);
                }
                _ => {
                    sample.color = self.background_color.to_rgba32();
                }
            }
        }
    }

    pub fn initialize(&mut self, world: &scene::World) {
        self.compute_ratio();
        self.free_textures();
        self.load_textures(world);
    }

    pub fn render(&self, world: &scene::World, camera: &scene::Camera) -> Image<RGBA32> {
        let mut data = Block::new(self.res_x as u32, self.res_y as u32, 0, 0);

        //Sampling
        let sampler = DefaultSampler { sample_rate: self.subdivision_sampling };
        sampler.create_samples(&mut data);

        let filter = filters::BoxFilter::default();
        //filter.set_image_size(self.res_x as u32, self.res_y as u32);

        // Emission des rayons
        for pixel in data.pixels_mut() {
            self.calculate_rays(world, camera, pixel);
        }

        // Création de l'image
        // filter.get_image(data)

        // TODO plus besoin de ce code quand on aura un filter
        let mut temp_result: Vec<Vec<RGBA32>> = vec![];

        for x in 0..data.dimensions().0 {
            let mut col: Vec<RGBA32> = vec![];

            for y in 0..data.dimensions().1 {
                col.push(filter.compute_color(data.get_pixel(x, y)));
            }

            temp_result.push(col);
        }

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
