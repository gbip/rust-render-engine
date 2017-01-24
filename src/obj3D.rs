use std::vec::Vec;
use math::{Vector3, Vector3f};
use std::clone::Clone;
use render::{Color8};


#[derive(Clone)]
pub struct Triangle {
    //Using a vector for easy allocation
    texture: Vec<Vector3f>,
    normals: Vec<Vector3f>,
    pos: Vec<Vector3f>,
}

impl Triangle {
    // Basic constructor
    pub fn new(tex: Vec<Vector3f>, norm: Vec<Vector3f> , pos:Vec<Vector3f>) -> Triangle {
        Triangle{texture: tex,
                normals: norm,
                pos: pos}
    }
}

/// The standard Indexed Face Set data structure for mesh.
pub struct Mesh {

    triangles : Vec<Triangle>,
}

impl Mesh {
    
   
    pub fn add_triangle(&mut self, tri : Triangle) {
        self.triangles.push(tri);
    }

    // Creates a new empty mesh
    pub fn new_empty() -> Mesh {
        Mesh{triangles: vec!()}
    }

    // Returns the triangle-ith triangle of the mesh (where triangle is it's coordinate in the
    // vector. Clone the value
    pub fn get_triangle(&self, triangle : usize) -> Triangle {
        self.triangles[triangle].clone()    
    }

}

#[derive(Serialize,Deserialize)]
pub struct Object {
    #[serde(skip_serializing,skip_deserializing,default = "Mesh::new_empty")]    
    ///The internal geometry data
    mesh: Mesh,
    
    ///The color of each triangles.
    color: Color8,
    
    ///The position of the object.
    position: Vector3f,
    
    ///The path to a .obj file that will be used to build the mesh.
    obj_path: String,    
}

mod obj_parser {
    use std::fs::File;
    use super::Mesh;
    use std::io::{BufRead, BufReader};
   
    enum LineType {
        Ignore,
        Face(u32,u32,Option<u32>),
        Vertex(f32,f32,f32),
        Normal(f32,f32,f32),
        TexCoord(f32,f32),
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

    fn parse_face(line: String ) -> Result<LineType,String> {
    
        unimplemented!() 
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
