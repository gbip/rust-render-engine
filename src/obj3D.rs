use std::vec::Vec;
use std::fmt;
use math::{Vector3, Vector3f,Vector2f, VectorialOperations};
use color::{RGBA8,RGBA32};
use ray::{Ray, Plane, Surface, Fragment};
use std::slice::Iter;

// The Raw Point represents a triangle point where each coordinate is an index to the real value
// stored in a vector
#[derive(Debug)]

struct RawPoint(usize,usize,Option<usize>);

// The Raw Data represents a a type of value for each points of the triangle. It could be position,
// normals or textures.
struct RawData(u32,u32,u32);

// A simple structure to hold data before initializing the triangle.
#[derive(Debug)]
struct RawTriangle(RawPoint,RawPoint,RawPoint);



#[derive(Clone,Debug,Copy)]
pub struct GeoPoint {
    norm : Vector3f,
    tex: Option<Vector2f>,
    pos: Vector3f,
}

impl GeoPoint {
    pub fn new(pos: Vector3f, norm: Vector3f, tex: Option<Vector2f>) -> GeoPoint {
        GeoPoint{norm:norm,
                tex:tex,
                pos:pos}
    }
}

#[derive(Clone,Debug,Copy)]
pub struct Triangle {
    u : GeoPoint,
    v : GeoPoint,
    w : GeoPoint,
}

impl Triangle {
    pub fn new(u : GeoPoint, v : GeoPoint, w: GeoPoint) -> Triangle {
        Triangle{u:u,v:v,w:w}
    }
}


impl Surface for Triangle {
    fn get_intersection(&self, ray : &Ray, color : &RGBA32) -> Option<Fragment> {
        let u = self.u.pos;
        let v = self.v.pos;
        let w = self.w.pos;

        // Calcul des vecteurs du repère barycentrique
        let vecA = &v - &u;
        let vecB = &w - &u;
        let plane = Plane::new(&vecA, &vecB, &u);

        let mut result = plane.get_intersection(ray,color);

        if let Some(ref mut point) = result {
            // On calcule si le point appartient à la face triangle
            let vecP = point.position - u;
            let a : f32 = vecA.norm();
            let b : f32 = vecB.norm();
            let ap : f32 = vecP.dot_product(&vecA) / a;
            let bp : f32 = vecP.dot_product(&vecB) / b;

            if !(a >= 0.0 && b >= 0.0 && bp / b < 1.0 - ap / a) {
                return None;
            }

            // Interpolation des normales et textures
            let na = &self.v.norm - &self.u.norm;
            let nb = &self.w.norm - &self.u.norm;
            point.normal = self.u.norm + (na * (ap / a)) + (nb * (bp / b));

            // TODO textures
        }
        result
    }
}

#[derive(Clone,Debug)]
pub struct Mesh {
    triangles : Vec<Triangle>,
}

impl Mesh {
    // Crée un nouveau mesh vide
    pub fn new_empty() -> Mesh {
        Mesh{triangles: vec!()}
    }

    fn create_point(pos:usize,norm:usize,tex:Option<usize>,
        list_pos : &[Vector3f],list_norm : &[Vector3f],list_tex : &Option<Vec<Vector2f>>) -> GeoPoint {

        // Très important : il faut décaller chaque indice de 1, car dans la norme .obj le premier
        // indice est 1, alors que dans le vector c'est 0.
        GeoPoint::new(list_pos[pos-1], list_norm[norm-1], match tex {
            Some(index) => match *list_tex {
                Some(ref vec) => Some(vec[index-1]),
                None => panic!("Error, a point as a texture coordinate while the mesh doesn't have one.")
        },
            None => match *list_tex {
                None => None,
                Some(_) => panic!("Error, the mesh has some texture coordinates, but a point doesn't have texture coordinates."),
            }
        })
    }

    fn add_triangles(&mut self,tris:Vec<RawTriangle>,pos : Vec<Vector3f>,norm : Vec<Vector3f> ,tex :Option<Vec<Vector2f>>) {
        for triangle in tris {
            let p1 = triangle.0;
            let p2 = triangle.1;
            let p3 = triangle.2;
            let u = Mesh::create_point(p1.0,p1.1,p1.2, &pos, &norm, &tex);
            let v = Mesh::create_point(p2.0,p2.1,p2.2, &pos, &norm, &tex);
            let w = Mesh::create_point(p3.0,p3.1,p3.2, &pos, &norm, &tex);
            self.triangles.push(Triangle::new(u,v,w));
        }

    }

    // Renvoie un itérateur sur &Triangle. (lecture seule)
    pub fn triangles(&self) -> Iter<Triangle> {
        self.triangles.iter() 
    }

}

#[derive(Serialize,Deserialize)]
pub struct Object {
    #[serde(skip_serializing,skip_deserializing,default = "Mesh::new_empty")]
    // La géométrie de l'objet
    mesh: Mesh,

    // La couleur (diffus) de l'objet. Deviendra certainement par la suite le shadeR.
    color: RGBA8,

    // La position de l'objet (offset qui se propagera ensuite aux triangles)
    position: Vector3f,

    // Le chemin vers un .obj qui permettra de charger l'objet
    obj_path: String,
    
    // Le nom de l'objet
    name : String,
}

impl Object {
    // Crée un nouvel objet
    pub fn new(color:RGBA8,position:Vector3f,path:String,name:String) -> Object {
        let mut result = Object::new_empty();
        result.color=color;
        result.position=position;
        result.obj_path=path;
        result.name=name;
        result.load_mesh();
        result
    }

    // Charge la géométrie donnée par le chemin "obj_path"
    fn load_mesh(& mut self) {
        self.mesh = obj_parser::open_obj(&self.obj_path);
    }
    
    // Initialise un objet. Pour l'instant cela ne fait que charger le mesh, mais on peut imaginer
    // d'autres traitements.
    pub fn initialize(&mut self) {
        self.load_mesh();
    }
    // Crée un objet vide
    pub fn new_empty() -> Object {
        Object{mesh:Mesh::new_empty(),
                color:RGBA8::new_black(),
                position:Vector3::new(0_f32,0_f32,0_f32),
                obj_path:"".to_string(),
                name:"untitled".to_string()}
    }

    // Renvoie un iterator sur des refs vers les triangles de l'objet (lecture seule).
    pub fn triangles(&self) -> Iter<Triangle> {
        self.mesh.triangles()
    }
    
    pub fn color(&self) -> RGBA8 {
        self.color
    }

}

impl fmt::Debug for Object {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Object {{color: {:?}, position: {:?}, model : {:?} }}",self.color,self.position,self.obj_path)
    }
}

mod obj_parser {
    use super::{Mesh,RawPoint,RawTriangle,RawData};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use math::{Vector2f,Vector3f};
    enum LineType {
        Ignore,
        Face(RawData,RawData,Option<RawData>),
        Vertex(f32,f32,f32),
        Normal(f32,f32,f32),
        TexCoord(f32,f32),
    }

    // A function to convert an Option<(u32,u32,u32)> to (Option<usize>,..).
    // Useful for parsing texture coordinates
    fn propagate_option(val:Option<RawData>) -> (Option<usize>,Option<usize>,Option<usize>) {
        match val {
            Some(tuple) => (Some(tuple.0 as usize),Some(tuple.1 as usize),Some(tuple.2 as usize)),
            None => (None,None,None),
        }
    }

    // Open an obj file and return a mesh with the data.
    pub fn open_obj(file: &str) -> Mesh {

        let reader = BufReader::new(open_obj_file(file));

        let mut tris : Vec<(RawData,RawData,Option<RawData>)> = vec!();
        let mut pos : Vec<Vector3f> = vec!();
        let mut normals : Vec<Vector3f> = vec!();
        let mut tex : Option<Vec<Vector2f>> = None;

        // We clean the reader of all useless lines before iterating over it.
        for line in reader.lines().map(|l| l.expect("Error while reading line")).collect::<Vec<String>>() {
            let parsed_line = match parse_line(line) {
                Ok(t) => t,
                Err(e) => panic!(e),
            };

            // We parse the file line by line and fill the vectors
            match parsed_line {
                LineType::Ignore => continue,
                LineType::Face(pos,norm,tex) => tris.push((pos,norm,tex)),
                LineType::Normal(x,y,z) => normals.push(Vector3f::new(x,y,z)),
                LineType::Vertex(x,y,z) => pos.push(Vector3f::new(x,y,z)),
                LineType::TexCoord(u,v) => match tex {
                    Some(ref mut vec) => {vec.push(Vector2f::new(u,v));},
                    None => { tex = Some(vec!());},
                },
            };
        }

        let mut mesh = Mesh::new_empty();


        let mut raw_triangles : Vec<RawTriangle> = vec!();

        // We just convert the Option<RawData> to a (Option<usize>,...,...)
        let tris : Vec<(RawData,RawData,(Option<usize>,Option<usize>,Option<usize>))> =
                    tris.into_iter()
                    .map(|t| (t.0,t.1,propagate_option(t.2)))
                    .collect::<Vec<(RawData,RawData,(Option<usize>,Option<usize>,Option<usize>))>>();
        for t in tris {
            let (p1,p2,p3) = (RawPoint((t.0).0 as usize,(t.1).0 as usize,(t.2).0),
                            RawPoint((t.0).1 as usize,(t.1).1 as usize,(t.2).1),
                            RawPoint((t.0).2 as usize,(t.1).2 as usize,(t.2).2));
            raw_triangles.push(RawTriangle(p1,p2,p3))
        }

        mesh.add_triangles(raw_triangles,pos,normals,tex);
        mesh
    }




    //Split a given line and parse each float value inside.
    fn get_floats(line : String) -> Vec<f32> {
        //We split the string by the whitespaces | parse each substring as a f32 | throw away
        //things that doesn't parse to f32
        line.split_whitespace().filter_map(|val : &str| val.parse::<f32>().ok())
            .collect()
    }

    // Input : " 2//1 4//1 3//1"
    // Output : [["2", "1"], ["4", "1"], ["3", "1"]]
    fn get_face(str : String) -> Vec<Vec<String>> {
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
        RawData(vec[0],vec[1],vec[2])
    }

    // We know two things : either there is position + normal, or there is position + normal +
    // textures. Plus, we only have vertex per triangle.
    fn extract_indexes(line : String) -> Result<(RawData,RawData,Option<RawData>),String> {
        let data = get_face(line.clone());
        let mut id_pos : Vec<u32> = vec!();
        let mut id_norm : Vec<u32> = vec!();
        let mut id_tex : Vec<u32> = vec!();

        //Represents the fact that a line may contain an invalid number of indices
        let mut error = false;
        let parsed_data : Vec<Vec<u32>> = data.iter().map(|u|
                                                u.iter()
                                                .map(|val| convert_to_u32(val))
                                                .collect())
                                                    .collect();

        //Splitting the case between the position + normal or the pos + norm + tex scenario
        for elem in parsed_data {
            match elem.len() {
                3 => {id_pos.push(elem[0]); id_norm.push(elem[1]); id_tex.push(elem[2])},
                2 => {id_pos.push(elem[0]);id_norm.push(elem[1]);},
                _ => {error = true;},
            }
        }

        match error {

        true => Err(format!("Incorrect number of indices, line : {}", line)),
        false => Ok((make_tuple(id_pos),make_tuple(id_norm), match id_tex.len() {
                    3 => Some(make_tuple(id_tex)),
                    _ => None,
        })),
        }
    }

    fn parse_normal(line : String) -> Result<LineType,String> {
        //We clone the line, to use line after for debugging.
        let floats = get_floats(line.clone());
        if floats.len() == 3 {
            Ok(LineType::Normal(floats[0],floats[1],floats[2]))
        }
        else {
            Err(format!("Invalide number of float value, expected 3, found : {} | Line parsed : {} ",floats.len(),line))
        }
    }

    fn parse_vertex(line : String) -> Result<LineType,String> {
        match parse_normal(line) {
            Ok(LineType::Normal(u,v,w)) => Ok(LineType::Vertex(u,v,w)),
            Err(t) => Err(t),
            _ => unreachable!()
        }
    }

    // Get the first 3 elements from a vector and returns them in a tuple
    fn vec_to_tuple3(vec:Vec<u32>) -> (u32,u32,u32) {
        (vec[0],vec[1],vec[2])
    }

    fn parse_face(line: String ) -> Result<LineType,String> {
        let indices = extract_indexes(line);
        match indices {
            // Transform the vector into a tuple3
            Ok(i) =>  Ok(LineType::Face(i.0,i.1,i.2)),
            //In cas of an error, we just propagate it one level further
            Err(e) => Err(e),
        }
    }

    fn parse_tex_coord(line: String) -> Result<LineType,String> {
        let floats = get_floats(line.clone());
        if floats.len() == 2 {
            Ok(LineType::TexCoord(floats[0],floats[1]))
        }
        else {
            Err(format!("Invalide number of float value, expected 3, found : {} | Line parsed : {} ",floats.len(),line))
        }
    }


    fn parse_line(line : String) -> Result<LineType,String> {
        match line.chars().nth(0).expect("Error while reading line") {
            //Trivial cas, we just doesn't support groups, or individual objects
            'o' | 'g' |'#' | 'u' | 'm' | 's' => Ok(LineType::Ignore),
            'f' => parse_face(line),
            'v' => match line.chars().nth(1).expect("Error while reading line") {
                ' ' => parse_vertex(line),
                'n' => parse_normal(line),
                't' => parse_tex_coord(line),
                _ => Err("Unexpected symbol".to_string()),
            },
            _ => Err("Unexpected symbol".to_string()),
        }
    }

    fn open_obj_file(path: &str) -> File {
        match File::open(path) {
            Ok(t) => t,
            Err(e) => panic!("Error while trying to open the file: {} - {}", path,e),
        }
    }
    #[cfg(test)]
    mod test {
        #[test]
        fn test_extract_indices() {
            unimplemented!()
        }
    }
}


#[cfg(test)]
mod tests {

    use obj3D::*;

    #[test]
    fn test_triangle_intersects() {
        unimplemented!();
    }

    // TESTS DU PARSER

}
