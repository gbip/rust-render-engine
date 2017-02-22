use std::vec::Vec;
use std::fmt;
use std::f32;
use math::{Vector3, Vector3f, Vector2f, VectorialOperations, AlmostEq};
use color::{RGBA8, RGBA32};
use ray::{Ray, Plane, Surface, Fragment};
use std::slice::Iter;
use angle::{Rad, Deg};
// The Raw Point represents a triangle point where each coordinate is an index to the real value
// stored in a vector
#[derive(Debug)]

struct RawPoint(usize, usize, Option<usize>);

// The Raw Data represents a a type of value for each points of the triangle. It could be position,
// normals or textures.
struct RawData(u32, u32, u32);

// A simple structure to hold data before initializing the triangle.
#[derive(Debug)]
struct RawTriangle(RawPoint, RawPoint, RawPoint);



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
}


impl Surface for Triangle {
    fn get_intersection(&self, ray: &mut Ray, color: &RGBA32) -> Option<Fragment> {
        let ptA = self.u.pos;
        let ptB = self.v.pos;
        let ptC = self.w.pos;

        // Calcul des vecteurs du repère barycentrique
        let vecAB = &ptB - &ptA;
        let vecBC = &ptC - &ptB;
        let vecCA = &ptA - &ptC;

        let plane = Plane::new(&vecAB, &vecBC, &ptA);

        let mut result = plane.get_intersection(ray, color);

        if let Some(ref mut point) = result {
            // On calcule si le point appartient à la face triangle
            let vecAP = point.position - ptA;
            let vecBP = point.position - ptB;
            let vecCP = point.position - ptC;

            let cpA = vecAB.cross_product(&vecAP);
            let cpB = vecBC.cross_product(&vecBP);
            let cpC = vecCA.cross_product(&vecCP);
            let N = vecAB.cross_product(&vecCA);

            if !(N.dot_product(&cpA) <= 0.0 && N.dot_product(&cpB) <= 0.0 &&
                 N.dot_product(&cpC) <= 0.0) {
                return None;
            }

            ray.max_t = point.param;

            let global_area_x2: f32 = vecAB.cross_product(&vecBC).norm();
            let u = cpA.norm() / global_area_x2;
            let v = cpB.norm() / global_area_x2;
            let w = cpC.norm() / global_area_x2;

            // Interpolation des normales et textures
            point.normal = self.u.norm * u + self.v.norm * v + self.w.norm * w;
            point.tex = match (self.u.tex, self.v.tex, self.w.tex) {
                (Some(ref texu), Some(ref texv), Some(ref texw)) => {
                    Some(texu * u + texv * v + texw * w)
                }
                _ => None,
            }
        }
        result
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

    fn create_point(pos: usize,
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

    fn add_triangles(&mut self,
                     tris: Vec<RawTriangle>,
                     pos: Vec<Vector3f>,
                     norm: Vec<Vector3f>,
                     tex: Option<Vec<Vector2f>>) {
        for triangle in tris {
            let p1 = triangle.0;
            let p2 = triangle.1;
            let p3 = triangle.2;
            let u = Mesh::create_point(p1.0, p1.1, p1.2, &pos, &norm, &tex);
            let v = Mesh::create_point(p2.0, p2.1, p2.2, &pos, &norm, &tex);
            let w = Mesh::create_point(p3.0, p3.1, p3.2, &pos, &norm, &tex);
            self.triangles.push(Triangle::new(u, v, w));
        }

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

#[derive(Serialize,Deserialize)]
pub struct Object {
    #[serde(skip_serializing,skip_deserializing,default = "Mesh::new_empty")]
    mesh: Mesh,

    // La couleur (diffus) de l'objet. Deviendra certainement par la suite le shadeR.
    color: RGBA8,

    // La position de l'objet (offset qui se propagera ensuite aux triangles)
    position: Vector3f,

    // L'échelle de l'objet selon les trois axes
    scale: Vector3f,

    // La rotation de l'objet selon les trois axes
    rotation: Vector3<Deg<f32>>,

    // Le chemin vers un .obj qui permettra de charger l'objet
    obj_path: String,

    // Le nom de l'objet
    name: String,

    // Le barycentre
    #[serde(skip_serializing,skip_deserializing,default = "Vector3f::zero")]
    barycenter: Vector3f,

    // La visibilité de l'objet
    visible: bool,
}

impl Object {
    // Crée un nouvel objet
    pub fn new(color: RGBA8, position: Vector3f, path: String, name: String) -> Object {
        let mut result = Object::new_empty();
        result.color = color;
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

    // Initialise un objet. Pour l'instant cela ne fait que charger le mesh, mais on peut imaginer
    // d'autres traitements.
    pub fn initialize(&mut self) {
        // Important, on charge le mesh avant de commencer à rendre car sinon le calcul du
        // barycentre est débile.
        self.load_mesh();
        let barycenter = self.compute_barycenter();
        let old_pos: Vector3f = self.position;
        if !barycenter.aeq(&Vector3f::zero()) {
            println!("Warning, the object {} is not centered in (0,0,0) but in {}",
                     self.name,
                     &barycenter);
            // On centre l'objet à l'origine.
            self.position = -barycenter;
            self.apply_position();
            self.position = old_pos;
        }
        self.barycenter = barycenter;
        self.apply_scale();
        self.apply_rotation();
        self.apply_position();
    }
    // Crée un objet vide
    pub fn new_empty() -> Object {
        Object {
            mesh: Mesh::new_empty(),
            color: RGBA8::new_black(),
            position: Vector3::new(0_f32, 0_f32, 0_f32),
            scale: Vector3f::new(1f32, 1f32, 1f32),
            rotation: Vector3 {
                x: deg!(0.0f32),
                y: deg!(0.0f32),
                z: deg!(0.0f32),
            },
            obj_path: "".to_string(),
            name: "untitled".to_string(),
            barycenter: Vector3f::zero(),
            visible: true,
        }
    }

    // Renvoie un iterator sur des refs vers les triangles de l'objet (lecture seule).
    pub fn triangles(&self) -> Iter<Triangle> {
        self.mesh.triangles()
    }

    pub fn color(&self) -> RGBA8 {
        self.color
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Object {{color: {:?}, position: {:?}, model : {:?} }}",
               self.color,
               self.position,
               self.obj_path)
    }
}

mod obj_parser {
    use super::{Mesh, RawPoint, RawTriangle, RawData};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use math::{Vector2f, Vector3f};
    enum LineType {
        Ignore,
        Face(RawData, RawData, Option<RawData>),
        Vertex(f32, f32, f32),
        Normal(f32, f32, f32),
        TexCoord(f32, f32),
    }

    // A function to convert an Option<(u32,u32,u32)> to (Option<usize>,..).
    // Useful for parsing texture coordinates
    fn propagate_option(val: Option<RawData>) -> (Option<usize>, Option<usize>, Option<usize>) {
        match val {
            Some(tuple) => (Some(tuple.0 as usize), Some(tuple.1 as usize), Some(tuple.2 as usize)),
            None => (None, None, None),
        }
    }

    // Open an obj file and return a mesh with the data.
    pub fn open_obj(file: &str) -> Mesh {

        let reader = BufReader::new(open_obj_file(file));

        let mut tris: Vec<(RawData, RawData, Option<RawData>)> = vec![];
        let mut pos: Vec<Vector3f> = vec![];
        let mut normals: Vec<Vector3f> = vec![];
        let mut tex: Option<Vec<Vector2f>> = None;

        // We clean the reader of all useless lines before iterating over it.
        for line in reader.lines()
            .map(|l| l.expect("Error while reading line"))
            .collect::<Vec<String>>() {
            let parsed_line = match parse_line(line) {
                Ok(t) => t,
                Err(e) => panic!(e),
            };

            // We parse the file line by line and fill the vectors
            match parsed_line {
                LineType::Ignore => continue,
                LineType::Face(pos, norm, tex) => tris.push((pos, norm, tex)),
                LineType::Normal(x, y, z) => normals.push(Vector3f::new(x, y, z)),
                LineType::Vertex(x, y, z) => pos.push(Vector3f::new(x, y, z)),
                LineType::TexCoord(u, v) => {
                    match tex {
                        Some(ref mut vec) => {
                            vec.push(Vector2f::new(u, v));
                        }
                        None => {
                            tex = Some(vec![]);
                        }
                    }
                }
            };
        }

        let mut mesh = Mesh::new_empty();


        let mut raw_triangles: Vec<RawTriangle> = vec![];

        // We just convert the Option<RawData> to a (Option<usize>,...,...)
        #[allow(type_complexity)]
        let tris : Vec<(RawData,RawData,(Option<usize>,Option<usize>,Option<usize>))> =
                    tris.into_iter()
                    .map(|t| (t.0,t.1,propagate_option(t.2)))
                    .collect::<Vec<(RawData,RawData,(Option<usize>,Option<usize>,Option<usize>))>>();
        for t in tris {
            let (p1, p2, p3) = (RawPoint((t.0).0 as usize, (t.1).0 as usize, (t.2).0),
                                RawPoint((t.0).1 as usize, (t.1).1 as usize, (t.2).1),
                                RawPoint((t.0).2 as usize, (t.1).2 as usize, (t.2).2));
            raw_triangles.push(RawTriangle(p1, p2, p3))
        }

        mesh.add_triangles(raw_triangles, pos, normals, tex);
        mesh
    }




    //Split a given line and parse each float value inside.
    fn get_floats(line: String) -> Vec<f32> {
        //We split the string by the whitespaces | parse each substring as a f32 | throw away
        //things that doesn't parse to f32
        line.split_whitespace()
            .filter_map(|val: &str| val.parse::<f32>().ok())
            .collect()
    }

    // Input : " 2//1 4//1 3//1"
    // Output : [["2", "1"], ["4", "1"], ["3", "1"]]
    fn get_face(str: String) -> Vec<Vec<String>> {
        str.split(' ').map(|x| x.split('/') // we split the line by the '/' character
                                .map(|x| x.to_string()) // we convert the char to a string
                                .filter(|x| x!="") // we remove the empty strings
                                .collect())
                      .filter(|x| x[0]!="f") // we remove useless junk
                      .collect()

    }

    fn convert_to_u32(string: &str) -> u32 {
        str::parse::<u32>(string).expect("Error while parsing integer indices")
    }


    // Take the first 3 elements of a vector, and returns a RawData tuple.
    fn make_tuple(vec: Vec<u32>) -> RawData {
        RawData(vec[0], vec[1], vec[2])
    }

    // We know two things : either there is position + normal, or there is position + normal +
    // textures. Plus, we only have vertex per triangle.
    fn extract_indexes(line: String) -> Result<(RawData, RawData, Option<RawData>), String> {
        let data = get_face(line.clone());
        let mut id_pos: Vec<u32> = vec![];
        let mut id_norm: Vec<u32> = vec![];
        let mut id_tex: Vec<u32> = vec![];

        //Represents the fact that a line may contain an invalid number of indices
        let mut error = false;
        let parsed_data: Vec<Vec<u32>> = data.iter()
            .map(|u| {
                u.iter()
                    .map(|val| convert_to_u32(val))
                    .collect()
            })
            .collect();

        //Splitting the case between the position + normal or the pos + norm + tex scenario
        for elem in parsed_data {
            match elem.len() {
                3 => {
                    id_pos.push(elem[0]);
                    id_norm.push(elem[1]);
                    id_tex.push(elem[2])
                }
                2 => {
                    id_pos.push(elem[0]);
                    id_norm.push(elem[1]);
                }
                _ => {
                    error = true;
                }
            }
        }

        if error {
            Err(format!("Incorrect number of indices, line : {}", line))
        } else {
            Ok((make_tuple(id_pos),
                make_tuple(id_norm),
                match id_tex.len() {
                    3 => Some(make_tuple(id_tex)),
                    _ => None,
                }))
        }
    }


    fn parse_normal(line: String) -> Result<LineType, String> {
        //We clone the line, to use line after for debugging.
        let floats = get_floats(line.clone());
        if floats.len() == 3 {
            Ok(LineType::Normal(floats[0], floats[1], floats[2]))
        } else {
            Err(format!("Invalide number of float value, expected 3, found : {} | Line parsed : \
                         {} ",
                        floats.len(),
                        line))
        }
    }

    fn parse_vertex(line: String) -> Result<LineType, String> {
        match parse_normal(line) {
            Ok(LineType::Normal(u, v, w)) => Ok(LineType::Vertex(u, v, w)),
            Err(t) => Err(t),
            _ => unreachable!(),
        }
    }

    // Get the first 3 elements from a vector and returns them in a tuple
    fn vec_to_tuple3(vec: Vec<u32>) -> (u32, u32, u32) {
        (vec[0], vec[1], vec[2])
    }

    fn parse_face(line: String) -> Result<LineType, String> {
        let indices = extract_indexes(line);
        match indices {
            // Transform the vector into a tuple3
            Ok(i) => Ok(LineType::Face(i.0, i.1, i.2)),
            //In cas of an error, we just propagate it one level further
            Err(e) => Err(e),
        }
    }

    fn parse_tex_coord(line: String) -> Result<LineType, String> {
        let floats = get_floats(line.clone());
        if floats.len() == 2 {
            Ok(LineType::TexCoord(floats[0], floats[1]))
        } else {
            Err(format!("Invalide number of float value, expected 3, found : {} | Line parsed : \
                         {} ",
                        floats.len(),
                        line))
        }
    }


    fn parse_line(line: String) -> Result<LineType, String> {
        match line.chars().nth(0).expect("Error while reading line") {
            //Trivial cas, we just doesn't support groups, or individual objects
            'o' | 'g' | '#' | 'u' | 'm' | 's' => Ok(LineType::Ignore),
            'f' => parse_face(line),
            'v' => {
                match line.chars().nth(1).expect("Error while reading line") {
                    ' ' => parse_vertex(line),
                    'n' => parse_normal(line),
                    't' => parse_tex_coord(line),
                    _ => Err("Unexpected symbol".to_string()),
                }
            }
            _ => Err("Unexpected symbol".to_string()),
        }
    }

    fn open_obj_file(path: &str) -> File {
        match File::open(path) {
            Ok(t) => t,
            Err(e) => panic!("Error while trying to open the file: {} - {}", path, e),
        }
    }

    #[cfg(test)]
    mod test {
        use math::{Vector3, Vector3f};
        use super::super::*;

        #[test]
        fn test_obj_parsing_plane() {
            let a: f32 = 8.555269;
            let b: f32 = 5.030090;
            let c: f32 = 4.669442;
            let d: f32 = 10.555269;
            let e: f32 = 2.669442;

            let norm1: Vector3f = Vector3::new(0.0, 1.0, 0.0);
            let p1 = GeoPoint::new(Vector3::new(a, b, c), norm1, None);
            let p2 = GeoPoint::new(Vector3::new(d, b, c), norm1, None);
            let p3 = GeoPoint::new(Vector3::new(a, b, e), norm1, None);
            let p4 = GeoPoint::new(Vector3::new(d, b, e), norm1, None);

            let t1 = Triangle::new(p2, p4, p3);
            let t2 = Triangle::new(p1, p2, p3);

            let expected_result = Mesh { triangles: vec![t1, t2] };

            assert_eq!(obj_parser::open_obj("models/plane_no_uv.obj"),
                       expected_result);
        }
    }
}

#[cfg(test)]
mod test {
    use math::{Vector3, Vector3f};
    use ray::{Surface, Ray};
    use super::{GeoPoint, Triangle};
    use color::RGBA32;

    #[test]
    fn test_triangle_ray_intersection() {
        let p1 = GeoPoint::new_pos(Vector3::new(1.0, 0.0, 1.0));
        let p2 = GeoPoint::new_pos(Vector3f::new(-1.0, 0.0, 1.0));
        let p3 = GeoPoint::new_pos(Vector3f::new(0.0, 0.0, -1.0));

        let tri1 = Triangle::new(p1, p2, p3);

        // Ce rayon doit intersecter le triangle en (0,0,0)
        let mut r1 = Ray::new(Vector3f::new(0.0, -1.0, 0.0), Vector3f::new(0.0, 1.0, 0.0));

        let frag1 = tri1.get_intersection(&mut r1, &RGBA32::new_black());
        assert!(frag1 != None);

        // Normalement, l'intersection du triangle est en (0.5,0,0), donc ce rayon ne doit pas
        // intersecter avec le triangle
        let mut r2 = Ray::new(Vector3f::new(0.0, -1.0, 0.0), Vector3f::new(0.51, 1.0, 0.0));

        let frag2 = tri1.get_intersection(&mut r2, &RGBA32::new_black());
        assert!(frag2 == None);

        // Celui là par contre devrait :
        let mut r3 = Ray::new(Vector3f::new(0.0, -1.0, 0.0), Vector3f::new(0.5, 1.0, 0.0));

        let frag3 = tri1.get_intersection(&mut r3, &RGBA32::new_black());
        assert!(frag3 != None);
    }




}
