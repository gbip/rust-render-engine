use scene;
use img::Image;
use color::RGBA32;
use math::Vector3f;
use ray::{Ray, Fragment, Surface};


#[derive(Serialize,Deserialize,Debug)]
pub struct Renderer {
    res_x: usize,
    res_y: usize,
    ratio: f32,
}

impl Renderer {
    pub fn new(res_x: usize, res_y: usize) -> Self {
        Renderer {
            res_x: res_x,
            res_y: res_y,
            ratio: (res_x as f32 / res_y as f32),
        }
    }

    /** Calcule les rayons à lancer pour le canvas passé en paramètres.
    Calcule ensuite la couleur finale de chaque rayon et stocke le résultat dans
    le canvas passé en paramètres. */
    pub fn emit_rays(&self, world: &scene::World, camera: &scene::Camera, canvas: &mut Canvas,
                            ray_density_x : u32, ray_density_y : u32) {

        // On crée les rayons à emmettre
        let mut rays : Vec<Ray> = vec!();

        for x in 0..(ray_density_x - 1) {
            for y in 0..(ray_density_y - 1) {
                let target = canvas.origin
                        + canvas.e1 * ((x as f32 + 0.5) / ray_density_x as f32)
                        + canvas.e2 * ((y as f32 + 0.5) / ray_density_y as f32);

                rays.push(Ray::new(camera.world_position, target - camera.world_position));
            }
        }

        canvas.fragments.clear();

        // On calcule chaque point d'intersection
        for ray in rays {
            let mut result : Option<Fragment> = None;

            for object in world.objects() {
                let points : Vec<Option<Fragment>> = object.triangles()
                                .map(|tri| tri.get_intersection(&ray, &RGBA32::new_black())) // TODO changer en la couleur de l'objet
                                .filter(|point| point.is_some())
                                .collect();

                match points.len() {
                    0 => {}
                    n => result = points[n - 1] // TODO ici le fragment est copié. Chercher une façon de juste le déplacer.
                }
            }

            // Comme pour chaque rayon on ajoute un unique fragment, l'ordre est conservé et on peut retrouver la position de chaque fragment
            // dans l'espace à partir de sa position dans la liste.
            // A faire peut-être : stocker les fragments directement en fonction de leur position dans l'image.
            match result {
                // Le dernier fragment trouvé est celui qui correspond à l'objet le plus en avant de la scène par rapport à la caméra
                Some(fragment) => canvas.fragments.push(fragment),
                None => canvas.fragments.push(Fragment::new(
                            Vector3f {x : 0.0, y : 0.0, z : 0.0},
                            0.0,
                            RGBA32::new_black())), // TODO changer le fragment par défaut
            }
        }
    }

    #[allow(unused_variables)]
    pub fn render(&self, world: &scene::World, camera: &scene::Camera) -> Image<RGBA32> {
        // Création de l'image qui résulte du rendu
        let result = Image::<RGBA32>::new(self.res_x, self.res_y);

        let mut canvas: Vec<Canvas> = vec![];
        let rays: Vec<Ray> = vec![];
        let points: Vec<Fragment> = vec![];

        // On crée les "canvas"
        let (origin, vec1, vec2) = camera.get_canvas_basis(self.ratio);
        let e1 = vec1 / self.res_x as f32;
        let e2 = vec2 / self.res_y as f32;

        for x in 0..(self.res_x - 1) {
            for y in 0..(self.res_y - 1) {
                let x1 = x as f32 / self.res_x as f32;
                let y1 = y as f32 / self.res_y as f32;

                canvas.push(Canvas::new(origin + vec1 * x1 + vec2 * y1,
                                        e1,
                                        e2));
            }
        }

        // On emet les rayons

        // Post process
        //for ray in rays {
        //    points=world.objects.iter().map(|obj| obj.triangles().map(|tri| tri.get_intersection_point(ray,&obj.color()))
        //                                                        .collect())
        //                               .collect();
        //                                 }
        // Chaque pixel est recomposé suivant les rayons qui en ont été émis

        //result
        unimplemented!();
    }
}


/** Représente un rectangle en trois dimensions, correspondant à un pixel sur l'image finale à rendre.
Ce carré est décrit par une origine, et deux vecteurs directeurs. */
pub struct Canvas {
    origin: Vector3f,
    e1: Vector3f,
    e2: Vector3f,
    fragments : Vec<Fragment>,
}


impl Canvas {
    pub fn new(origin: Vector3f, e1: Vector3f, e2: Vector3f) -> Canvas {
        Canvas {
            origin: origin,
            e1: e1,
            e2: e2,
            fragments: vec![],
        }
    }
}
