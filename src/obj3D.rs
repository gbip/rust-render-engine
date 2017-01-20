use std::vec::Vec;
use math::{Vector3, Vector3f};
use std::clone::Clone;
use render::{Color8};

/// This structs only hold references to the vertex that are stocked in the mesh.
#[derive(Clone)]
pub struct Triangle {
    pub vertex: [Vector3<f32>; 3],
}

pub struct Polygon<'a> {
    vertex: Vec<&'a Vector3<f32>>,
}

/// The standard Indexed Face Set data structure for mesh.
pub struct Mesh {
    triangle_list: Vec<Triangle>,
}

impl Mesh {

    fn new(vertex: Vec<Vector3f>,face_indices: Vec<(usize,usize,usize)>) -> Mesh {
        let mut mesh_triangles : Vec<Triangle> = vec!();
        for triangle in face_indices {
            let (v1,v2,v3) = triangle;
            mesh_triangles.push(Triangle{vertex:[vertex[v1],vertex[v2],vertex[v3]]});
        }
        Mesh{triangle_list:mesh_triangles}
    }
    fn new_from_obj(path:String) -> Mesh {
        unimplemented!()/*
        let (vertex, face_indices) = obj::generate_vec_from_obj(path);
        Mesh::new(vertex,face_indices)*/

    }
    fn new_empty() -> Mesh {
        Mesh{triangle_list:vec!()}
    }

    /// A function for Serde, do not rename because it will break Serde.
    fn empty() -> Mesh {
        Mesh::new_empty()
    }
}

#[derive(Serialize,Deserialize)]
pub struct Object {
    #[serde(skip_serializing,skip_deserializing,default = "Mesh::empty")]    
    ///The internal geometry data
    mesh: Mesh,
    
    ///The color of each triangles.
    color: Color8,
    
    ///The position of the object.
    position: Vector3f,
    
    ///The path to a .obj file that will be used to build the mesh.
    obj_path: String,    
}

impl Object {
    pub fn get_triangles(&self) -> Vec<Triangle> {

        let mut result: Vec<Triangle> = vec![];
        for triangles in &self.mesh.triangle_list {
            result.push(triangles.clone());
        }
        return result;
    }
}

mod obj_parser {
    use std::fs::File;
    use super::Mesh;
    use std::io::{BufRead, BufReader};
    
    enum LineType {
        Ignore,
        Face(u32,u32,u32),
        Vertex(f32,f32,f32)
    }

    fn get_obj_data_from_file(path: &str) -> Vec<(f32,f32,f32)> {
        
       unimplemented!(); 
    let objects = split_into_objects(open_obj_file(path));   
        
        
        
    }
    
    
    
    fn open_obj_file(path: &str) -> File {
        match File::open(path) {
            Ok(t) => t,
            Err(e) => panic!("Error while trying to open the file: {} - {}", path,e),
        }
    }
    
    fn split_into_objects(file: File) -> Vec<Vec<String>> {
        let buffer = BufReader::new(&file);
        let mut result : Vec<Vec<String>> = vec!(vec!());
        let mut is_parsing = false;
        let mut temp_object: Vec<String> = vec!();
        // with the map and the Filter we insure 2 things : we unwrap the result of the BufReader
        // which can fail at reading a file and we make sure that each lines isn't empty so that
        // line.chars().nth(0).unwrap() can be called safely
        for line in buffer.lines().map(|r| r.expect("Fatal error while reading the .obj file")).filter(|s| s.len()>0) {
            if line.chars().nth(0).unwrap() == 'o' {
                if is_parsing {
                    result.push(temp_object);
                    temp_object = vec!();
                    continue;
                }
                else {
                    is_parsing = true;
                    continue;
                }
            }
            else {
                if is_parsing {
                    temp_object.push(line);
                }
                else {
                    continue;
                }
            }
        }
        result
    }

    pub fn parse_obj(path : &str) -> super::Mesh {
        open_obj_file(path);
        Mesh::new_empty()
    }
}

