use scene;
use img::Image;
use color::{RGBA8, RGBA32};
use color;
use math::Vector3f;
use ray::{Ray, Fragment, Surface};

#[derive(Serialize,Deserialize,Debug)]
pub struct Renderer {
    res_x: usize,
    res_y: usize,
    ratio: f32,
    background_color: RGBA8,
}


impl Renderer {
    pub fn new(res_x: usize, res_y: usize) -> Self {
        Renderer {
            res_x: res_x,
            res_y: res_y,
            ratio: (res_x as f32 / res_y as f32),
            background_color: RGBA8::new_black(),
        }
    }

    pub fn set_resolution(&mut self, res_x: usize, res_y: usize) {
        self.res_x = res_x;
        self.res_y = res_y;
        self.ratio = res_x as f32 / res_y as f32;

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

        for x in 0..(ray_density_x - 1) {
            for y in 0..(ray_density_y - 1) {
                let target = canvas.origin + canvas.e1 * ((x as f32 + 0.5) / ray_density_x as f32) +
                             canvas.e2 * ((y as f32 + 0.5) / ray_density_y as f32);

                rays.push(Ray::new(camera.world_position, target - camera.world_position));
            }
        }

        canvas.fragments.clear();

        // On calcule chaque point d'intersection
        for ray in rays {
            let mut result: Option<Fragment> = None;

            for object in world.objects() {
                let points : Vec<Option<Fragment>> = object.triangles()
                                .map(|tri| tri.get_intersection(&ray, &object.color().to_rgba32()))
                                .filter(|point| point.is_some())
                                .collect();

                match points.len() {
                    0 => {}
                    n => result = points[n - 1], // TODO ici le fragment est copié. Chercher une façon de juste le déplacer.
                }
            }

            // Comme pour chaque rayon on ajoute un unique fragment, l'ordre est conservé et on peut retrouver la position de chaque fragment
            // dans l'espace à partir de sa position dans la liste.
            // A faire peut-être : stocker les fragments directement en fonction de leur position dans l'image.
            match result {
                // Le dernier fragment trouvé est celui qui correspond à l'objet le plus en avant de la scène par rapport à la caméra
                Some(fragment) => canvas.fragments.push(fragment),
                None => {
                    canvas.fragments.push(Fragment::new(Vector3f {
                                                            x: 0.0,
                                                            y: 0.0,
                                                            z: 0.0,
                                                        },
                                                        0.0,
                                                        self.background_color.to_rgba32()))
                } // TODO changer le fragment par défaut
            }
        }
    }

    fn create_canvas(&self, camera: &scene::Camera) -> Vec<Vec<Canvas>> {
        let mut canvas: Vec<Vec<Canvas>> = vec![];

        // On crée les "canvas"
        let (origin, vec1, vec2) = camera.get_canvas_base(self.ratio);
        let e1 = vec1 / self.res_x as f32;
        let e2 = vec2 / self.res_y as f32;

        for x in 0..(self.res_x - 1) {
            let mut line: Vec<Canvas> = vec![];
            for y in 0..(self.res_y - 1) {
                let x1 = x as f32 / self.res_x as f32;
                let y1 = y as f32 / self.res_y as f32;

                line.push(Canvas::new(origin + vec1 * x1 + vec2 * y1, e1, e2));
            }
            canvas.push(line);
        }
        canvas

    }

    pub fn render(&self, world: &scene::World, camera: &scene::Camera) -> Image<RGBA32> {

        let mut canvas: Vec<Vec<Canvas>> = self.create_canvas(camera);

        for line in &mut canvas {
            for pixel in &mut line.iter_mut() {
                self.emit_rays(world, camera, pixel, 5, 5);
            }
        }

        let temp_result: Vec<Vec<RGBA32>> = canvas.into_iter()
            .map(|line| line.into_iter().map(|frag| frag.get_average_color()).collect())
            .collect();

        Image::<RGBA32>::from_vec_vec(&temp_result)

    }
}


/** Représente un rectangle en trois dimensions, correspondant à un pixel sur l'image finale à rendre.
Ce carré est décrit par une origine, et deux vecteurs directeurs. */
pub struct Canvas {
    origin: Vector3f,
    e1: Vector3f,
    e2: Vector3f,
    fragments: Vec<Fragment>,
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
    pub fn get_average_color(&self) -> RGBA32 {

        let colors = self.fragments.iter().map(|f| f.color).collect();
        color::make_average_color(colors)

    }
}

#[cfg(test)]
mod test {
    use scene::Scene;
    use math::Vector3;
    #[test]
    // Cas simple :
    // On met une caméra à l'origine, la cible en (1,1,1) et on génére des canvas pour une caméra
    // avec une résolution de 4x4
    fn test_simple_canvas_generation() {
        let mut scene = Scene::new_empty();
        scene.renderer.set_resolution(4, 4);
        scene.world.add_camera(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        unimplemented!()
    }
}
