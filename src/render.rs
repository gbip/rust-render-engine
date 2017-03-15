use scene;
use img::Image;
use color::{RGBA8, RGBA32};
use color;
use ray::{Ray, Fragment, Surface};
use sampler::{Sample, DefaultSampler, Sampler};
use geometry::obj3d::Object;
use std::collections::HashMap;
use std::fmt;

/** Structure utilisée par le sampler pour stocker les samples, et par le filter
pour les lire et recomposer l'image finale
TODO rename, déplacer ?*/
pub struct RenderData {
    pixels: Vec<Pixel>,
    size_x: u32,
    size_y: u32,
}

impl RenderData {
    pub fn new(size_x: u32, size_y: u32) -> Self {
        let mut result = RenderData {
            pixels: vec![],
            size_x: size_x,
            size_y: size_y,
        };

        // Ajout des pixels (l'ordre des for est important)
        for y in 0..size_y {
            for x in 0..size_x {
                result.pixels.push(Pixel::new(x, y));
            }
        }

        result
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.size_x, self.size_y)
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.pixels[(x + y * self.size_x) as usize]
    }
}

/** Représente un pixel avec des Sample dedans. */
pub struct Pixel {
    x: u32,
    y: u32,
    samples: Vec<Sample>,
}

impl Pixel {
    pub fn new(x: u32, y: u32) -> Pixel {
        Pixel {
            x: x,
            y: y,
            samples: vec![],
        }
    }

    pub fn add_sample(&mut self, sample: Sample) {
        self.samples.push(sample);
    }

    pub fn get_average_color(&self) -> RGBA32 {
        let mut colors: Vec<RGBA32> = vec![];
        for sample in &self.samples {
            colors.push(sample.color);
        }
        color::make_average_color(&colors)
    }
}



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

    pub fn calculate_ray_intersection<'a>(&self,
                                          objects: &[&'a Object], // TODO Changer en raytree
                                          mut ray: &mut Ray)
                                          -> (Option<Fragment>, Option<&'a Object>) {

        let mut fragment: Option<Fragment> = None;
        let mut obj: Option<&Object> = None;
        for object in objects {
            // TODO : Peut être virer le branching ici ?
            // TODO : Regarder le geometry/intersection.rs dans tray
            if object.bounding_box().intersects(ray) {
                if let Some(frag) = object.get_intersection(ray) {
                    fragment = Some(frag);
                    obj = Some(object);
                }
            }
        }
        (fragment, obj)
    }

    /** Calcule les rayons à lancer pour le canvas passé en paramètres.
    Calcule ensuite la couleur finale de chaque rayon et stocke le résultat dans
    le canvas passé en paramètres. */
    pub fn emit_rays(&self, world: &scene::World, camera: &scene::Camera, pixel: &mut Pixel) {

        let objects = world.objects()
            .iter()
            .filter(|bbox| bbox.is_visible())
            .collect::<Vec<&Object>>();

        for sample in &mut pixel.samples {
            // On récupère le rayon à partir du sample
            let mut ray = camera.create_ray_from_sample(self.ratio, sample);

            // CALCUL DE LA COULEUR DU RAYON (TODO à mettre ailleurs)

            let (opt_frag, opt_obj) = self.calculate_ray_intersection(&objects, &mut ray);

            // On détermine la couleur du rayon, simplement à partir du fragment retourné et
            // du matériau associé à l'objet intersecté.
            match (opt_frag, opt_obj) {
                (Some(fragment), Some(object)) => {
                    let color: RGBA32;
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
                    sample.color = color;
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
        let mut data = RenderData::new(self.res_x as u32, self.res_y as u32);

        //Sampling
        let sampler = DefaultSampler { sample_rate: self.subdivision_sampling };
        sampler.create_samples(&mut data);

        // Emission des rayons
        for pixel in &mut data.pixels {
            self.emit_rays(world, camera, pixel);
        }

        // Création de l'image
        // filter.get_image(data)

        // TODO plus besoin de ce code quand on aura un filter
        let mut temp_result: Vec<Vec<RGBA32>> = vec![];

        for x in 0..data.size_x {
            let mut col: Vec<RGBA32> = vec![];

            for y in 0..data.size_y {
                col.push(data.get_pixel(x, y).get_average_color());
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
