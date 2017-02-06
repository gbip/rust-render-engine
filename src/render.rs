use scene;
use img::Image;
use color::RGBA32;
use math::Vector3f;
use ray::{Ray,Fragment};



pub struct Renderer {
    res_x : usize,
    res_y : usize,
    ratio : f32,
}

impl Renderer {
    pub fn new(res_x : usize, res_y : usize) -> Self {
        Renderer {
            res_x : res_x,
            res_y : res_y,
            ratio : (res_x as f32/res_y as f32)
        }
    }
    #[allow(unused_variables)]
    pub fn emit_rays(&self, world : &scene::World, camera : &scene::Camera, canvas : &mut Canvas) {

    }
    #[allow(unused_variables)]
    pub fn render(&self, world : &scene::World, camera : &scene::Camera) -> Image<RGBA32> {
        // Création de l'image qui résulte du rendu
        let result = Image::<RGBA32>::new(self.res_x, self.res_y);

        let mut canvas : Vec<Canvas> = vec!();
        let rays : Vec<Ray> = vec!();
        let points : Vec<Fragment> = vec!();

        // On crée les "canvas"
        let (origin, vec1, vec2) = camera.get_canvas_basis(self.ratio);

        for x in 0..(self.res_x - 1) {
            for y in 0..(self.res_y - 1) {
                let x1 = x as f32 / self.res_x as f32;
                let x2 = (x + 1) as f32 / self.res_x as f32;
                let y1 = y as f32 / self.res_y as f32;
                let y2 = (y + 1) as f32 / self.res_y as f32;

                canvas.push(Canvas::new(
                    origin + vec1 * x1 + vec2 * y1,
                    origin + vec1 * x2 + vec2 * y1,
                    origin + vec1 * x1 + vec2 * y2
                ));
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

pub struct Canvas {
    u : Vector3f,
    v : Vector3f,
    w : Vector3f,
    rays : Vec<Fragment>,
}

/// Represents a window through which the camera is seeing the world. It depends mainly of the
/// FOV of the camera, and the clipping distance between the virtual point of the camera and
/// this window
impl Canvas {

    /// This method will panic if the canvas is a line or a dot: u1.x = u2.x or u1.y = u2.y . It
    /// will also panic if u1.x > u2.x or u1.y > u2.y
    /// TODO : Make this return a result and not panic!()...
    pub fn new(u: Vector3f, v: Vector3f, w: Vector3f) -> Canvas {
       //assert!(u1.x != u2.x && u1.y != v.y);
       //assert!(u1.x < u2.x && u1.y < u2.y);
       Canvas{u:u,v:v,w:w,rays:vec!()}
    }

    /// Returns the 4 corner of the rendering rectangle, in a clockwise order, starting by the
    /// top left corner.
    pub fn get_corners(&self) -> [Vector3f;4] {
        /*let u1 = Vector2{x: self.u.x,y: self.v.y};
        let u2 = Vector2{x: self.v.x,y: self.v.y};
        let u3 = Vector2{x: self.v.x,y: self.u.x};
        let u4 = Vector2{x: self.u.x,y: self.u.y};
        [u1,u2,u3,u4]*/
        unimplemented!()
    }
}
