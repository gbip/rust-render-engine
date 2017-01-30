use std::vec::Vec;
use math::{Vector3, Vector3f,Vector2f, VectorialOperations};
use std::clone::Clone;
use render::{Color8};
use ray::{Ray, Plane, Surface, IntersectionPoint, Fragment};

// The Raw Point represents a triangle point where each coordinate is an index to the real value
// stored in a vector
#[derive(Debug)]
struct Raw_Point(usize,usize,Option<usize>);

// The Raw Data represents a a type of value for each points of the triangle. It could be position,
// normals or textures.
struct Raw_Data(u32,u32,u32);

// A simple structure to hold data before initializing the triangle.
#[derive(Debug)]
struct Raw_Triangle(Raw_Point,Raw_Point,Raw_Point);



#[derive(Clone)]
pub struct GeoPoint<'a> {
    norm : &'a Vector3f,
    tex: Option<&'a Vector2f>,
    pos: &'a Vector3f,
}

impl<'a> GeoPoint<'a> {
    pub fn new(pos: &'a Vector3f, norm: &'a Vector3f, tex: Option<&'a Vector2f>) -> GeoPoint<'a> {
        GeoPoint{norm:norm,
                tex:tex,
                pos:pos}
    }
}

#[derive(Clone)]
pub struct Triangle<'a> {
    u : GeoPoint<'a>,
    v : GeoPoint<'a>,
    w : GeoPoint<'a>,
}

impl<'a> Triangle<'a> {
    pub fn new(u : GeoPoint<'a>, v : GeoPoint<'a>, w: GeoPoint<'a>) -> Triangle<'a> {
        Triangle{u:u,v:v,w:w}
    }
}

impl<'a> Surface for Triangle<'a> {
    fn get_intersection_point(&self, ray : &Ray) -> Option<IntersectionPoint> {
        let u = *self.u.pos;
        let v = *self.v.pos;
        let w = *self.w.pos;

        let vecA = v - u;
        let vecB = w - u;
        let plane = Plane::new(&vecA, &vecB, &u);

        let mut result = plane.get_intersection_point(&ray);
        let fragment = Fragment::new_empty();

        if let Some(ref point) = result {
            // On calcule si le point appartient à la face triangle
            let vecP = point.position - u;
            let a : f32 = vecA.norm();
            let b : f32 = vecB.norm();
            let ap : f32 = vecP.dot_product(&vecA) / a;
            let bp : f32 = vecP.dot_product(&vecB) / b;

            if !(a >= 0.0 && b >= 0.0 && bp / b < 1.0 - ap / a) {
                return None;
            }

            // TODO Interpolation des normales et textures
        }

        result
    }
}

pub fn gen_point<'a>(vec_pos: &'a Vec<Vector3f>,
                  vec_norm: &'a Vec<Vector3f>,
                  vec_tex: &'a Option<Vec<Vector2f>>,
                  ind_pos:usize,
                  ind_norm:usize,
                  ind_tex:Option<usize>)-> GeoPoint<'a> {

         let point : GeoPoint<'a> = GeoPoint::new(vec_pos.get(ind_pos-1).unwrap(),vec_norm.get(ind_norm-1).unwrap(),None);
         point
}

fn add_triangles<'a>(triangles : Vec<Raw_Triangle>,
                        list_triangles : &'a mut Vec<Triangle<'a>>,
                        list_pos : &'a Vec<Vector3f>,
                        list_norm : &'a Vec<Vector3f>,
                        list_tex: &'a Option<Vec<Vector2f>>) {
    for tr in triangles {

        let (ind_pos1,ind_norm1,ind_tex1) = ((tr.0).0,(tr.0).1,(tr.0).2);

        let p1 : GeoPoint<'a> = gen_point(list_pos,list_norm,list_tex,ind_pos1,ind_norm1,ind_tex1); //Immutable borrow

        let p2 : GeoPoint<'a> = gen_point(list_pos,list_norm,list_tex,ind_pos1,ind_norm1,ind_tex1); //Immutable borrow

        let p3 : GeoPoint<'a> = gen_point(list_pos,list_norm,list_tex,ind_pos1,ind_norm1,ind_tex1); //Immutable borrow

        list_triangles.push(Triangle::new(p1,p2,p3)) //Mutable borrow
    }
}

/// The standard Indexed Face Set data structure for mesh.
pub struct Mesh<'a> {

    list_norm : Vec<Vector3f>,

    list_pos: Vec<Vector3f>,

    list_tex : Option<Vec<Vector2f>>,

    triangles : Vec<Triangle<'a>>,
}

impl<'a> Mesh<'a> {
    pub fn set_pos(& mut self, vec:Vec<Vector3f>) {
        self.list_pos=vec;
    }

    pub fn set_norm(& mut self, vec:Vec<Vector3f>) {
        self.list_norm=vec;
    }

    pub fn set_tex(& mut self, vec:Option<Vec<Vector2f>>) {
        self.list_tex=vec;
    }

    // Creates a new empty mesh
    pub fn new_empty() -> Mesh<'a> {
        Mesh{triangles: vec!(), list_norm: vec!(), list_pos: vec!(), list_tex:None}
    }

    pub fn new(pos: Vec<Vector3f>,norm: Vec<Vector3f>, tex:Option<Vec<Vector2f>>) -> Mesh<'a> {
        Mesh{list_norm:norm,
            list_pos:pos,
            list_tex:tex,
            triangles:vec!()}
        }
}

#[derive(Serialize,Deserialize)]
pub struct Object<'a> {
    #[serde(skip_serializing,skip_deserializing,default = "Mesh::new_empty")]
    ///The internal geometry data
    mesh: Mesh<'a>,

    ///The color of each triangles.
    color: Color8,

    ///The position of the object.
    position: Vector3f,

    ///The path to a .obj file that will be used to build the mesh.
    obj_path: String,
}

impl<'a> Object<'a> {
    pub fn initialize(&'a mut self,color:Color8,position:Vector3f,path:String) {
       self.color=color;
       self.position=position;
       self.obj_path=path;
       self.load_mesh();
    }

    pub fn load_mesh(&'a mut self) {
        obj_parser::open_obj(&mut self.mesh,&self.obj_path);
    }

    pub fn new_empty() -> Object<'a> {
        Object{mesh:Mesh::new_empty(),
                color:Color8::new_black(),
                position:Vector3::new(0_f32,0_f32,0_f32),
                obj_path:"".to_string()}
    }
}

mod obj_parser {
    use super::{Mesh,Raw_Point,Raw_Triangle,Raw_Data,add_triangles};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use math::{Vector2f,Vector3f};
    enum LineType {
        Ignore,
        Face(Raw_Data,Raw_Data,Option<Raw_Data>),
        Vertex(f32,f32,f32),
        Normal(f32,f32,f32),
        TexCoord(f32,f32),
    }

    // A function to convert an Option<(u32,u32,u32)> to (Option<usize>,..).
    // Useful for parsing texture coordinates
    fn propagate_option(val:Option<Raw_Data>) -> (Option<usize>,Option<usize>,Option<usize>) {
        match val {
            Some(tuple) => (Some(tuple.0 as usize),Some(tuple.1 as usize),Some(tuple.2 as usize)),
            None => (None,None,None),
        }
    }

    // Open an obj file and return a mesh with the data.
    pub fn open_obj<'a>(mesh: &'a mut Mesh<'a>,file: &String) {

        let reader = BufReader::new(open_obj_file(file.as_str()));

        let mut tris : Vec<(Raw_Data,Raw_Data,Option<Raw_Data>)> = vec!();
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


        let mut raw_triangle : Vec<Raw_Triangle> = vec!();

        // We just convert the Option<Raw_Data> to a (Option<usize>,...,...)
        let mut tris : Vec<(Raw_Data,Raw_Data,(Option<usize>,Option<usize>,Option<usize>))> =
                    tris.into_iter()
                    .map(|t| (t.0,t.1,propagate_option(t.2)))
                    .collect::<Vec<(Raw_Data,Raw_Data,(Option<usize>,Option<usize>,Option<usize>))>>();
        for t in tris {
            let (p1,p2,p3) = (Raw_Point((t.0).0 as usize,(t.1).0 as usize,(t.2).0),
                            Raw_Point((t.0).1 as usize,(t.1).1 as usize,(t.2).1),
                            Raw_Point((t.0).2 as usize,(t.1).2 as usize,(t.2).2));
            raw_triangle.push(Raw_Triangle(p1,p2,p3))
        }

        mesh.set_pos(pos);
        mesh.set_norm(normals);
        mesh.set_tex(tex);

        add_triangles(raw_triangle,&mut mesh.triangles,&mesh.list_pos, &mesh.list_norm,&mesh.list_tex)
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
        let r : Vec<Vec<String>> = str.split(' ').map(|x| x.split('/') // we split the line by the '/' character
                                                           .map(|x| x.to_string()) // we convert the char to a string
                                                           .filter(|x| x!="") // we remove the empty strings
                                                           .collect())
                                                .filter(|x| x[0]!="f") // we remove useless junk
                                                .collect();
        r
    }

    fn convert_to_u32(string: &str) -> u32 {
        str::parse::<u32>(string).expect("Error while parsing integer indices")
    }


    // Take the first 3 elements of a vector, and returns a Raw_Data tuple.
    fn make_tuple(vec: Vec<u32>) -> Raw_Data {
        Raw_Data(vec[0],vec[1],vec[2])
    }

    // We know two things : either there is position + normal, or there is position + normal +
    // textures. Plus, we only have vertex per triangle.
    fn extract_indexes(line : String) -> Result<(Raw_Data,Raw_Data,Option<Raw_Data>),String> {
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

    mod test {
        use super::*;

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
