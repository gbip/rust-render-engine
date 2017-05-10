use scene;
use img::Image;
use color::{RGBA8, RGBA32};
use ray::{Ray, Intersection};
use geometry::obj3d::Object;
use std::collections::HashMap;
use std::fmt;
use renderer::Pixel;
use renderer::block::Block;
use filter::FilterFactory;
use sampler::SamplerFactory;
use std::sync::Mutex;
use std::clone::Clone;
use std::ops::DerefMut;
use std::io::Stdout;
use scoped_pool::Pool;
use colored::*;
use pbr::ProgressBar;
use std::f32;

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

    #[serde(rename = "sampler")]
    sampler_factory: SamplerFactory,

    #[serde(rename = "filter")]
    filter_factory: FilterFactory,

    background_color: RGBA8,

    #[serde(skip_serializing, skip_deserializing, default = "HashMap::new")]
    textures: HashMap<String, Image<RGBA8>>,

    threads: usize,

    bucket_size: usize,
}

impl Renderer {
    pub fn new(res_x: usize, res_y: usize) -> Self {
        Renderer {
            res_x: res_x,
            res_y: res_y,
            ratio: (res_x as f32 / res_y as f32),
            background_color: RGBA8::new_black(),
            textures: HashMap::new(),
            sampler_factory: SamplerFactory::HaltonSampler { subdivision_sampling: 4 },
            filter_factory: FilterFactory::BoxFilter,
            bucket_size: 10,
            threads: 1,
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
        println!("The output resolution is : {} x {}", self.res_x, self.res_y);

        let stri = format!("{} {}",
                           "Rendering with",
                           format!("{} threads", self.threads).yellow());
        println!("{}", stri);
    }

    pub fn calculate_ray_intersection<'b>(&self,
                                          objects: &[&'b Object], // TODO Changer en raytree
                                          ray: &mut Ray)
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
            let mut ray: Ray =
                camera.create_ray_from_sample(sample,
                                              self.ratio,
                                              self.res_x as f32,
                                              self.res_y as f32);

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

    /** Cette fonction permet de générer des blocs pour rendre l'image */
    fn generate_blocks(&self) -> Vec<Block> {
        let bloc_size = self.bucket_size;
        let mut result: Vec<Block> = vec![];

        // Le nombre de pixels qui ne tiendront pas dans des blocs de taille standard (bloc_size x
        // bloc_size)
        let offset_x: u32 = (self.res_x % bloc_size) as u32;
        let offset_y: u32 = (self.res_y % bloc_size) as u32;

        // Le nombre de pixels qui tiennent dans des blocs standards (par opposition aux offsets)
        let size_x: u32 = self.res_x as u32 - offset_x;
        let size_y: u32 = self.res_y as u32 - offset_y;

        // Le nombre de blocs en x et en y sans compter les blocs non standards
        let bloc_count_x: u32 = f32::floor(size_x as f32 / bloc_size as f32) as u32;
        let bloc_count_y: u32 = f32::floor(size_y as f32 / bloc_size as f32) as u32;


        // Gestion des blocs non standards
        if offset_x != 0 {
            for y in 0..bloc_count_y {
                let block = Block::new(offset_x, bloc_size as u32, size_x, y * bloc_size as u32);
                result.push(block);
            }
        }
        if offset_y != 0 {
            for x in 0..bloc_count_x {
                let block = Block::new(bloc_size as u32, offset_y, x * bloc_size as u32, size_y);
                result.push(block);
            }
        }

        //Gestion du bloc en bas à droite
        if offset_y != 0 || offset_x != 0 {
            let block = Block::new(offset_x, offset_y, size_x, size_y);
            result.push(block);
        }

        // Gestion des blocs standards
        for i in 0..bloc_count_x {
            for j in 0..bloc_count_y {
                let block = Block::new(bloc_size as u32,
                                       bloc_size as u32,
                                       i * bloc_size as u32,
                                       j * bloc_size as u32);
                result.push(block);
            }
        }
        result

    }

    /** Fonction principale, qui génére les blocs de l'image et les rends, pour enfin les
     * recombiner dans une image finale. */
    #[allow(let_and_return)]
    pub fn render(&self, world: &scene::World, camera: &scene::Camera) -> Image<RGBA32> {
        let shared_image: Mutex<Image<RGBA32>> = Mutex::new(Image::new(self.res_x, self.res_y));

        // On definit le nombre de threads à utiliser
        let pool = Pool::new(self.threads);

        // Génération des sous bloc de l'image
        let mut blocks = self.generate_blocks();

        // La barre qui affiche le temps d'attente du rendu
        let progress_bar: Mutex<ProgressBar<Stdout>> = Mutex::new(ProgressBar::new(blocks.len() as
                                                                                   u64));
        progress_bar.lock().unwrap().show_speed = false;
        progress_bar.lock().unwrap().show_counter = false;
        progress_bar.lock().unwrap().message("Rendering : ");
        progress_bar.lock().unwrap().format("|▌▌░|");

        // On passe les blocs aux threads
        pool.scoped(|scope| while !blocks.is_empty() {
            let block = blocks.pop().unwrap();
            scope.execute(|| {
                self.render_block(block, world, camera, &shared_image);
                progress_bar.lock().unwrap().inc();
            });
        });

        progress_bar.lock().unwrap().finish();

        // On transforme le Arc<Mutex<Image>> en Image
        let result = shared_image.lock().unwrap().deref_mut().clone();
        result
    }

    /** Cette fonction se charge de rendre un bloc de l'image. */
    pub fn render_block(&self,
                        mut block: Block,
                        world: &scene::World,
                        camera: &scene::Camera,
                        shared_image: &Mutex<Image<RGBA32>>) {

        // Generation des samples
        self.sampler_factory
            .create_sampler()
            .create_samples(&mut block);

        let filter = self.filter_factory
            .create_filter(self.res_x as u32, self.res_y as u32);

        // Emission des rayons
        for pixel in block.pixels_mut() {
            self.calculate_rays(world, camera, pixel);
        }


        let mut temp_result: Vec<Vec<RGBA32>> = vec![];

        // Reconstruction de l'image à partir des samples et du filtre
        for x in 0..block.dimensions().0 {
            let mut col: Vec<RGBA32> = vec![];

            for y in 0..block.dimensions().1 {
                col.push(filter.compute_color(block.get_pixel(x, y),
                                              (block.position_x(), block.position_y())));
            }
            temp_result.push(col);
        }

        // Superposition de l'image rendue à l'image finale
        shared_image.lock()
            .unwrap()
            .deref_mut()
            .superpose_sub_image(Image::<RGBA32>::from_vec_vec(&temp_result),
                                 block.position_x(),
                                 block.position_y());
    }
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Renderer : resolution = {}x{}, background_color = {:?}, threads = {}",
               self.res_x,
               self.res_y,
               self.background_color,
               self.threads)
    }
}
