use std::vec::Vec;
use std::f32;
use math::{Vector3, Vector3f, Vector2f, VectorialOperations, AlmostEq};
use material::flat_material::FlatMaterial;
use ray::{Ray, Plane, Surface, Fragment, Intersection};
use std::slice::Iter;
use angle::{Rad, Deg};
use colored::*;
use geometry::bounding_box::BoundingBox;
use geometry::obj_parser;

#[derive(Clone,Debug,Copy,PartialEq)]
pub struct GeoPoint {
    norm: Vector3f,
    tex: Option<Vector2f>,
    pos: Vector3f,
}

impl GeoPoint {
    pub fn new(pos: Vector3f, norm: Vector3f, tex: Option<Vector2f>) -> GeoPoint {
        GeoPoint {
            norm: norm,
            tex: tex,
            pos: pos,
        }
    }

    // Crée un point sans coordonée de texture et avec une normale nulle. Utile pour écrire des test.
    pub fn new_pos(pos: Vector3f) -> GeoPoint {
        GeoPoint {
            norm: Vector3f::new(0.0, 0.0, 0.0),
            pos: pos,
            tex: None,
        }
    }

    pub fn add_position(&mut self, position: &Vector3f) {
        self.pos = &self.pos + position;
    }

    pub fn rotate_around(&mut self, u: &Vector3f, angle: f32) {
        let c = angle.cos();
        let mc = 1.0 - c;
        let s = angle.sin();

        let uxy = u.x * u.y;
        let uyz = u.y * u.z;
        let uzx = u.z * u.x;

        // Formule tirée de Wikipedia : https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
        self.pos =
            Vector3f::new((u.x * u.x * mc + c) * self.pos.x + (uxy * mc - u.z * s) * self.pos.y +
                          (uzx * mc + u.y * s) * self.pos.z,
                          (uxy * mc + u.z * s) * self.pos.x + (u.y * u.y * mc + c) * self.pos.y +
                          (uyz * mc - u.x * s) * self.pos.z,
                          (uzx * mc - u.y * s) * self.pos.x + (uyz * mc + u.x * s) * self.pos.y +
                          (u.z * u.z * mc + c) * self.pos.z);
    }

    // déplace le géopoint de manière à ce que la distance qui le sépare de l'origine
    // soit multipliée par les trois composantes du vecteur scale.
    pub fn scale_from(&mut self, origin: &Vector3f, scale: &Vector3f) {
        let dist = &self.pos - origin;

        self.pos = *origin + &dist * scale;
    }

    pub fn pos(&self) -> Vector3f {
        self.pos
    }
}

#[derive(Clone,Debug,Copy,PartialEq)]
pub struct Triangle {
    u: GeoPoint,
    v: GeoPoint,
    w: GeoPoint,
}

impl Triangle {
    pub fn new(u: GeoPoint, v: GeoPoint, w: GeoPoint) -> Triangle {
        Triangle { u: u, v: v, w: w }
    }

    pub fn add_position(&mut self, position: &Vector3f) {
        self.u.add_position(position);
        self.v.add_position(position);
        self.w.add_position(position);
    }

    pub fn rotate_around(&mut self, axis: &Vector3f, angle: Rad<f32>) {
        self.u.rotate_around(axis, angle.0);
        self.v.rotate_around(axis, angle.0);
        self.w.rotate_around(axis, angle.0);
    }

    // Echelonne le triangle à partir du point d'origine, selon les trois axes
    // x, y et z.
    pub fn scale_from(&mut self, origin: &Vector3f, scale: &Vector3f) {
        self.u.scale_from(origin, scale);
        self.v.scale_from(origin, scale);
        self.w.scale_from(origin, scale);
    }

    pub fn get_barycenter(&self) -> Vector3f {
        (self.u.pos + self.v.pos + self.w.pos) / 3.0
    }

    pub fn min(&self) -> Vector3f {
        unimplemented!()
        //Vector3f::new(f32::min(self.u.pos.x, self.u.pos.x, self.u.pos.x), 0, 0)

    }

    pub fn u_pos(&self) -> Vector3f {
        self.u.pos()
    }

    pub fn v_pos(&self) -> Vector3f {
        self.v.pos()
    }

    pub fn w_pos(&self) -> Vector3f {
        self.w.pos()
    }
}


impl Surface for Triangle {
    fn get_intersection_fragment(&self, ray: &mut Ray) -> Option<Fragment> {
        let pt_a = self.u.pos;
        let pt_b = self.v.pos;
        let pt_c = self.w.pos;

        // Calcul des vecteurs du repère barycentrique
        let vec_ab = &pt_b - &pt_a;
        let vec_bc = &pt_c - &pt_b;
        let vec_ca = &pt_a - &pt_c;

        let plane = Plane::new(&vec_ab, &vec_bc, &pt_a);

        let mut result = plane.get_intersection_fragment(ray);

        if let Some(ref mut point) = result {
            // On calcule si le point appartient à la face triangle
            let vec_ap = point.position - pt_a;
            let vec_bp = point.position - pt_b;
            let vec_cp = point.position - pt_c;

            let cp_a = vec_ab.cross_product(&vec_ap);
            let cp_b = vec_bc.cross_product(&vec_bp);
            let cp_c = vec_ca.cross_product(&vec_cp);
            let n = vec_ab.cross_product(&vec_ca);

            if !(n.dot_product(&cp_a) <= 0.0 && n.dot_product(&cp_b) <= 0.0 &&
                 n.dot_product(&cp_c) <= 0.0) {
                return None;
            }

            ray.max_t = point.param;

            let global_area_x2: f32 = vec_ab.cross_product(&vec_bc).norm();
            let u = cp_c.norm() / global_area_x2;
            let v = cp_a.norm() / global_area_x2;
            let w = cp_b.norm() / global_area_x2;

            // Interpolation des normales et textures
            // P = wA + uB + vC
            point.normal = self.u.norm * w + self.v.norm * u + self.w.norm * v;
            point.tex = match (self.u.tex, self.v.tex, self.w.tex) {
                (Some(ref texu), Some(ref texv), Some(ref texw)) => {
                    Some(texu * w + texv * u + texw * v)
                }
                _ => None,
            }
        }
        result
    }

    /** Source : https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm  */
    #[allow(non_snake_case)]
    fn fast_intersection(&self, ray: &mut Ray) -> bool {

        let e1: Vector3f = self.v.pos() - self.u.pos();
        let e2: Vector3f = self.w.pos() - self.u.pos();
        let P: Vector3f = ray.slope().cross_product(&e2);

        let det: f32 = e1.dot_product(&P);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return false;
        }

        let inv_det: f32 = 1f32 / det;

        let T: Vector3f = ray.origin() - self.u.pos();
        let u: f32 = T.dot_product(&P) * inv_det;

        if u < 0f32 || u > 1f32 {
            return false;
        }

        let Q: Vector3f = T.cross_product(&e1);

        let v: f32 = ray.slope().dot_product(&Q) * inv_det;

        if v < 0f32 || u + v > 1f32 {
            return false;
        }

        let t: f32 = e2.dot_product(&Q);

        if t > f32::EPSILON {
            ray.max_t = t;
            return true;
        }
        false




    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    // Crée un nouveau mesh vide
    pub fn new_empty() -> Mesh {
        Mesh { triangles: vec![] }
    }

    pub fn create_point(pos: usize,
                        norm: usize,
                        tex: Option<usize>,
                        list_pos: &[Vector3f],
                        list_norm: &[Vector3f],
                        list_tex: &Option<Vec<Vector2f>>)
                        -> GeoPoint {

        // Très important : il faut décaller chaque indice de 1, car dans la norme .obj le premier
        // indice est 1, alors que dans le vector c'est 0.
        GeoPoint::new(list_pos[pos - 1],
                      list_norm[norm - 1],
                      match tex {
                          Some(index) => {
                              match *list_tex {
                                  Some(ref vec) => Some(vec[index - 1]),
                                  None => {
                                      panic!("Error, a point as a texture coordinate while the \
                                              mesh doesn't have one.")
                                  }
                              }
                          }
                          None => {
                              match *list_tex {
                                  None => None,
                                  Some(_) => {
                                      panic!("Error, the mesh has some texture coordinates, but a \
                                              point doesn't have texture coordinates.")
                                  }
                              }
                          }
                      })
    }

    pub fn add_triangle(&mut self, tri: Triangle) {
        self.triangles.push(tri);
    }

    // Renvoie un itérateur sur &Triangle. (lecture seule)
    pub fn triangles(&self) -> Iter<Triangle> {
        self.triangles.iter()
    }

    #[allow(float_cmp)]
    fn get_barycenter(&self, name: &str) -> Vector3f {
        let mut sum = Vector3f::new(0.0, 0.0, 0.0);
        let mut count = 0;

        if self.triangles.is_empty() {
            println!("Warning, you are trying to compute the barycenter for an object that is \
                      not yet loaded");
        }
        for tri in &self.triangles {
            sum = sum + tri.get_barycenter();
            count += 1;
        }

        // On vérifie si on a atteinds la valeur maximale d'un f32, auquel notre barycentre ne veut
        // certainement plus rien dire.
        if sum.x == f32::MAX || sum.y == f32::MAX || sum.z == f32::MAX {
            println!("There might be a float overflow while calculating the barycenter of the \
                      object named {}, raw data is : {:?}",
                     name,
                     sum);
        }
        sum / count as f32
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Object {
    #[serde(skip_serializing,skip_deserializing,default = "Mesh::new_empty")]
    mesh: Mesh,

    // Le materiau de l'objet
    #[serde(skip_serializing, skip_deserializing,default = "FlatMaterial::new_empty",rename="do_not_use")]
    material: FlatMaterial,

    // La position de l'objet (offset qui se propagera ensuite aux triangles)
    position: Vector3f,

    // L'échelle de l'objet selon les trois axes
    scale: Vector3f,

    // La rotation de l'objet selon les trois axes
    rotation: Vector3<Deg<f32>>,

    // Le chemin vers un .obj qui permettra de charger l'objet
    obj_path: String,

    // Le chemin vers le materiau
    #[serde(default = "String::new",rename="material")]
    material_path: String,

    // Le nom de l'objet
    name: String,

    // Le barycentre
    #[serde(skip_serializing,skip_deserializing,default = "Vector3f::zero")]
    barycenter: Vector3f,

    // La bounding box
    #[serde(skip_serializing, skip_deserializing, default = "BoundingBox::new")]
    bbox: BoundingBox,

    // La visibilité de l'objet
    visible: bool,
}

impl Object {
    // Crée un nouvel objet
    pub fn new(position: Vector3f, path: String, name: String) -> Object {
        let mut result = Object::new_empty();
        result.position = position;
        result.obj_path = path;
        result.name = name;
        result.initialize();
        result
    }

    // Charge la géométrie donnée par le chemin "obj_path"
    fn load_mesh(&mut self) {
        self.mesh = obj_parser::open_obj(&self.obj_path);
    }

    fn apply_position(&mut self) {
        for tri in &mut self.mesh.triangles {
            tri.add_position(&self.position);
        }

        // On réinitialise car ça n'a aucun sens de l'appliquer deux fois
        self.position = Vector3f::new(0.0, 0.0, 0.0);
    }

    fn apply_rotation(&mut self) {
        for tri in &mut self.mesh.triangles {
            tri.rotate_around(&Vector3f::new(1.0, 0.0, 0.0), (&self.rotation.x).into());
            tri.rotate_around(&Vector3f::new(0.0, 1.0, 0.0), (&self.rotation.y).into());
            tri.rotate_around(&Vector3f::new(0.0, 0.0, 1.0), (&self.rotation.z).into());
        }

        // On réinitialise car ça n'a aucun sens de l'appliquer deux fois
        self.rotation = Vector3::new(deg!(0.0f32), deg!(0.0f32), deg!(0.0f32));
    }

    //TODO La rotation autour d'un point (même si c'est un peu plus compliqué)
    fn apply_scale(&mut self) {
        for tri in &mut self.mesh.triangles {
            tri.scale_from(&Vector3f::new(0.0, 0.0, 0.0), &self.scale);
        }

        // On réinitialise car ça n'a aucun sens de l'appliquer deux fois
        self.scale = Vector3f::new(1.0, 1.0, 1.0);
    }

    fn compute_barycenter(&self) -> Vector3f {
        self.mesh.get_barycenter(&self.name)
    }

    // Chargement du matériau
    fn load_material(&mut self) {
        if self.material_path != "" {
            self.material = match FlatMaterial::read_from_file(self.material_path.as_str()) {
                Ok(value) => value,
                Err(e) => {
                    println!("Can't load the material {} due to error : {:?}",
                             self.material_path,
                             e);
                    FlatMaterial::new_empty()
                }
            }
        } else {
            println!("{} has not material, assigning to it a default material.",
                     self.name);

        }
    }

    // Calcule le barycentre de l'objet, le compare à (0,0,0) et recentre l'objet en fonction
    // /!\ Réinitialise la position de l'objet !!!!
    fn center(&mut self) {
        let barycenter = self.compute_barycenter();
        // Application des transformations
        let old_pos: Vector3f = self.position;
        if !barycenter.aeq(&Vector3f::zero()) {
            println!("{} ",
                     format!("Warning, the object {} is not centered in (0,0,0) but in {}",
                             self.name,
                             &barycenter)
                         .yellow()
                         .dimmed());
            // On centre l'objet à l'origine.
            self.position = -barycenter;
            self.apply_position();
            self.position = old_pos;
        }
        self.barycenter = barycenter;
    }

    // Initialise un objet. Pour l'instant cela ne fait que charger le mesh, mais on peut imaginer
    // d'autres traitements.
    pub fn initialize(&mut self) {
        // Important, on charge le mesh avant de commencer à rendre car sinon le calcul du
        // barycentre est débile.
        self.load_mesh();
        self.center();
        self.apply_scale();
        self.apply_rotation();
        self.apply_position();
        self.bbox = BoundingBox::new_from_object(self);
        self.load_material();
    }
    // Crée un objet vide
    pub fn new_empty() -> Object {
        Object {
            mesh: Mesh::new_empty(),
            material: FlatMaterial::new_empty(),
            position: Vector3::new(0_f32, 0_f32, 0_f32),
            scale: Vector3f::new(1f32, 1f32, 1f32),
            rotation: Vector3 {
                x: deg!(0.0f32),
                y: deg!(0.0f32),
                z: deg!(0.0f32),
            },
            obj_path: "".to_string(),
            material_path: "".to_string(),
            name: "untitled".to_string(),
            barycenter: Vector3f::zero(),
            bbox: BoundingBox::new(),
            visible: true,
        }
    }

    // Renvoie un iterator sur des refs vers les triangles de l'objet (lecture seule).
    pub fn triangles(&self) -> Iter<Triangle> {
        self.mesh.triangles()
    }

    pub fn material(&self) -> &FlatMaterial {
        &self.material
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bbox
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn position(&self) -> &Vector3f {
        &self.position
    }

    pub fn get_intersection_point(&self, ray: &mut Ray) -> Intersection {

        Intersection::new(self.get_intersection_fragment(ray),
                          &self.mesh,
                          &self.material)

    }
}

impl Surface for Object {
    fn get_intersection_fragment(&self, ray: &mut Ray) -> Option<Fragment> {

        let points: Vec<Option<Fragment>> = self.triangles()
            .map(|tri| tri.get_intersection_fragment(ray))
            .filter(|point| point.is_some())
            .collect();

        match points.len() {
            0 => None,
            n => points[n - 1],
        }
    }

    fn fast_intersection(&self, ray: &mut Ray) -> bool {
        if ray.max_t * ray.slope().norm() > self.barycenter.norm() {

            for tri in self.triangles() {
                if tri.fast_intersection(ray) {
                    return true;
                }
                continue;
            }
            false
        } else {
            false

        }



    }
}

#[cfg(test)]
mod test {
    use math::{Vector3, Vector3f};
    use ray::{Surface, Ray};
    use super::{GeoPoint, Triangle};

    #[test]
    fn test_triangle_ray_intersection() {
        let p1 = GeoPoint::new_pos(Vector3::new(1.0, 0.0, 1.0));
        let p2 = GeoPoint::new_pos(Vector3f::new(-1.0, 0.0, 1.0));
        let p3 = GeoPoint::new_pos(Vector3f::new(0.0, 0.0, -1.0));

        let tri1 = Triangle::new(p1, p2, p3);

        // Ce rayon doit intersecter le triangle en (0,0,0)
        let mut r1 = Ray::new(Vector3f::new(0.0, -1.0, 0.0), Vector3f::new(0.0, 1.0, 0.0));

        let frag1 = tri1.get_intersection_fragment(&mut r1);
        assert_ne!(frag1, None);

        // Normalement, l'intersection du triangle est en (0.5,0,0), donc ce rayon ne doit pas
        // intersecter avec le triangle
        let mut r2 = Ray::new(Vector3f::new(0.0, -1.0, 0.0), Vector3f::new(0.51, 1.0, 0.0));

        let frag2 = tri1.get_intersection_fragment(&mut r2);
        assert_eq!(frag2, None);

        // Celui là par contre devrait :
        let mut r3 = Ray::new(Vector3f::new(0.0, -1.0, 0.0), Vector3f::new(0.5, 1.0, 0.0));

        let frag3 = tri1.get_intersection_fragment(&mut r3);
        assert_ne!(frag3, None);
    }




}
