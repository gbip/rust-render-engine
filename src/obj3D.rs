use std::vec::Vec;
use math::{Vector3, Vector3f,Vector2f, VectorialOperations};
use std::clone::Clone;
use render::{Color8};
use ray::{Ray, Plane, Surface, IntersectionPoint};

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
    fn getIntersectionPoint(&self, ray : &Ray) -> Option<IntersectionPoint> {
        let u = *self.u.pos;
        let v = *self.v.pos;
        let w = *self.w.pos;

        let vecA = v - u;
        let vecB = w - u;
        let plane = Plane::new(&vecA, &vecB, &u);

        let result = plane.getIntersectionPoint(&ray);

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

            // TODO divers traitements d'interpolation (facilités par le calcul de ap et bp)
        }

        result
    }
}

/// The standard Indexed Face Set data structure for mesh.
pub struct Mesh<'a> {

    //points: Vec<GeoPoint<'a>>,

    list_norm : Vec<Vector3f>,
    list_pos: Vec<Vector3f>,
    list_tex : Option<Vec<Vector2f>>,
    triangles : Vec<Triangle<'a>>,
}

impl<'a> Mesh<'a> {

    pub fn set_list_norm(&mut self, new_list:Vec<Vector3f>) {
        self.list_norm = new_list;
    }

    pub fn set_list_pos(&mut self, new_list:Vec<Vector3f>) {
        self.list_pos = new_list;
    }

    pub fn set_list_tex(&mut self, new_list: Option<Vec<Vector2f>>) {
        self.list_tex=new_list;
    }

    pub fn store_position(&mut self,pos:Vector3f) {
        self.list_pos.push(pos);
    }

    pub fn store_norm(&mut self, norm:Vector3f) {
        self.list_norm.push(norm);
    }

    pub fn gen_point(&'a self,ind_pos:usize, ind_norm:usize,ind_tex:Option<usize>) -> GeoPoint<'a> {
        let tex_coord = match ind_tex {
            //TODO Maybe we should that get(i).unwrap() != None, because this is certainly not a
            //behavior that we want.
            Some(i) => match self.list_tex {
                None => panic!("Error, vertex has a texture coordinate, while mesh is storing no texture coordinate"),
                Some(ref vec) => vec.get(i),
            },
            None => match self.list_tex {
                None => None,
                Some(_) => panic!("Error, did not provide texture coordinate for a point will the mesh is explicitelly havin a texture mapping"),
            },
        };
         // It is safe to call .unwrap() because we know that the indice is in bound : we only
        // creates mesh through .obj file and and out of range index could only come from the file
        let point : GeoPoint<'a> = GeoPoint::new(self.list_pos.get(ind_pos).unwrap(),self.list_norm.get(ind_norm).unwrap(),tex_coord);
        point
    }


    pub fn add_triangle(&'a mut self,(ind_pos1,ind_norm1,ind_tex1):(usize,usize,Option<usize>), (ind_pos2,ind_norm2,ind_tex2) : (usize,usize,Option<usize>), (ind_pos3,ind_norm3,ind_tex3):(usize,usize,Option<usize>)) {
        unimplemented!()
        //let triangle : Triangle<'a> = Triangle::new(;

        //self.triangles.push(Triangle::new(&self.points[ind1],self.points.get_unchecked(ind2),self.points.get_unchecked(ind3)));
    }

    // Creates a new empty mesh
    pub fn new_empty() -> Mesh<'a> {
        Mesh{triangles: vec!(), list_norm: vec!(), list_pos: vec!(), list_tex:None}
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
     pub fn load_mesh(&mut self) {
        self.mesh = obj_parser::open_obj(&self.obj_path);
    }
}

mod obj_parser {
    use std::fs::File;
    use super::Mesh;
    use std::io::{BufRead, BufReader};
    use math::{Vector2f,Vector3f};
    enum LineType {
        Ignore,
        Face((u32,u32,u32),(u32,u32,u32),Option<(u32,u32,u32)>),
        Vertex(f32,f32,f32),
        Normal(f32,f32,f32),
        TexCoord(f32,f32),
    }


    pub fn open_obj<'a>(file: &String) -> Mesh<'a> {

        let mut result = Mesh::new_empty();
        let reader = BufReader::new(open_obj_file(file.as_str()));

        let mut tris : Vec<((u32,u32,u32),(u32,u32,u32),Option<(u32,u32,u32)>)> = vec!();
        let mut pos : Vec<Vector3f> = vec!();
        let mut normals : Vec<Vector3f> = vec!();
        let mut tex : Option<Vec<Vector2f>> = None;

        // We clean the reader of all useless lines before iterating over it.
        for line in reader.lines().map(|l| l.expect("Error while reading line")).collect::<Vec<String>>() {
            let parsed_line = match parse_line(line) {
                Ok(t) => t,
                Err(e) => panic!(e),
            };

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

        result.set_list_pos(pos);
        result.set_list_norm(normals);
        result.set_list_tex(tex);

        for triangle in tris {
            //do triangle stuff here

        }

        result
    }




    //Split a given line and parse each float value inside.
    fn get_floats(line : String) -> Vec<f32> {
        //We split the string by the whitespaces | parse each substring as a f32 | throw away
        //things that doesn't parse to f32
        line.split_whitespace().filter_map(|val : &str| val.parse::<f32>().ok())
            .collect()
    }

    fn get_face(str : String) -> Vec<Vec<String>> {
        let r : Vec<Vec<String>> = str.split(' ').map(|x| x.split('/') // we split the line by the '/' character
                                                            .map(|x| x.to_string()) // we convert the char to a string
                                                            .collect())
                                                .filter(|x| x[0]!="f") // we remove useless junk
                                                .collect();
        r
    }

    fn convert_to_u32(string: &str) -> u32 {
        str::parse::<u32>(string).expect("Error while parsing integer indices")
    }

    // We know two things : either there is position + normal, or there is position + normal +
    // textures. Plus, we only have vertex per triangle.
    //TODO: Maybe this function should return a tuple, because it checks that there is only 3 value.
    fn extract_indexes(line : String) -> Result<(Vec<u32>,Vec<u32>,Option<Vec<u32>>),String> {
        let data = get_face(line.clone());
        let mut id_pos : Vec<u32> = vec!();
        let mut id_norm : Vec<u32> = vec!();
        let mut id_tex : Vec<u32> = vec!();
        let mut error = false;
        let parsed_data : Vec<Vec<u32>> = data.iter().map(|u|
                        u.iter()
                        .map(|val| convert_to_u32(val))
                        .collect())
                                                    .collect();


            //_ => Err(format!("Incorrect number of indices : {} | line : {}", u.len(), line)),
        parsed_data.iter().map(|u| match u.len() {
            //TODO : check that the first indice is pos, the second is normal and the third is
            //texture.
            3 => {id_pos.push(u[0]); id_norm.push(u[1]); id_tex.push(u[2])},
            2 => {id_pos.push(u[0]);id_norm.push(u[1]);},
            _ => {error = true;},
        });
        match error {

        true => Err(format!("Incorrect number of indices, line : {}", line)),
        false => Ok((id_pos,id_norm, match id_tex.len() {
                    3 => Some(id_tex),
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
            Ok(i) => { let pos = vec_to_tuple3(i.0);
                    let norm = vec_to_tuple3(i.1);
                    match i.2 {
                        Some(tex_vec) => Ok(LineType::Face(pos,norm,Some(vec_to_tuple3(tex_vec)))),
                        None => Ok(LineType::Face(pos,norm,None)),
                    }
            },
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
}
