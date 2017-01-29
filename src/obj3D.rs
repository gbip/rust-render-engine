use std::vec::Vec;
use math::{Vector3, Vector3f,Vector2f};
use std::clone::Clone;
use render::{Color8};


struct Raw_Point(usize,usize,Option<usize>);
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
pub fn gen_point<'a>(vec_pos: &'a Vec<Vector3f>,
                  vec_norm: &'a Vec<Vector3f>,
                  vec_tex: &'a Option<Vec<Vector2f>>,
                  ind_pos:usize,
                  ind_norm:usize,
                  ind_tex:Option<usize>)-> GeoPoint<'a> {
         //For the simplicity of the example, we juste use "None"
         let point : GeoPoint<'a> = GeoPoint::new(vec_pos.get(ind_pos).unwrap(),vec_norm.get(ind_norm).unwrap(),None);
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
    use super::{Mesh,Raw_Point,Raw_Triangle,add_triangles};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use math::{Vector2f,Vector3f}; 
    enum LineType {
        Ignore,
        Face((u32,u32,u32),(u32,u32,u32),Option<(u32,u32,u32)>),
        Vertex(f32,f32,f32),
        Normal(f32,f32,f32),
        TexCoord(f32,f32),
    }

    // A function to convert an Option<(u32,u32,u32)> to (Option<usize>,..).
    // Useful for parsing texture coordinates
    pub fn propagate_option(val:Option<(u32,u32,u32)>) -> (Option<usize>,Option<usize>,Option<usize>) {
        match val {
            Some(tuple) => (Some(tuple.0 as usize),Some(tuple.1 as usize),Some(tuple.2 as usize)),
            None => (None,None,None),
        }
    }
    
    // Open an obj file and return a mesh with the data.
    pub fn open_obj<'a>(mesh: &'a mut Mesh<'a>,file: &String) {
        

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
        // We propagate the option for text_coord to the underlying tuple and we perform the
        // conversion from u32 to usize through propagate_option
        let mut tris = tris.iter_mut().map(|t| (t.0,t.1,propagate_option(t.2)))
                                        .collect::<Vec<((u32,u32,u32),(u32,u32,u32),(Option<usize>,Option<usize>,Option<usize>))>>();
        for t in tris {
            let (p1,p2,p3) = (Raw_Point((t.0).0 as usize,(t.1).0 as usize,(t.2).0),
                            Raw_Point((t.0).1 as usize,(t.1).1 as usize,(t.2).1),
                            Raw_Point((t.0).2 as usize,(t.1).2 as usize,(t.2).2)); 
            raw_triangle.push(Raw_Triangle(p1,p2,p3))
        }
        add_triangles(raw_triangle,&mut mesh.triangles,&mesh.list_pos, &mesh.list_norm,&mesh.list_tex)
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
