use std::vec::Vec;
use math::{Vector3,Vector3f};
use obj3D;
use obj3D::Object;
use std::fs::File;
use std::io::{Write,Read};
use std;
use serde_json;
use color::RGBA32;

fn write_string_to_file(j:&str,file_name:String) -> std::io::Result<()> {
        let mut file = File::create(file_name).unwrap();
            file.write_all(j.as_bytes())
}

fn open_file_as_string(file:&str) -> String {
    let mut result : String = "".to_string();
    match File::open(file) {
        Ok(mut val) => val.read_to_string(&mut result),
        Err(e) => panic!("Error could not open file {}, the error is : {}",file,e),
    };
    result
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Camera {
    /// The position fo the camera exprimed in the standard word space coordinates (where {0,0,0} is the
    /// center of the world)
    world_position : Vector3f,

    /// The position of the point at which the camera is aiming, in world space coordinates
    target_position : Vector3f,

    // The horizontal field of view, exprimed in degrees.
    fov : f32,

    // The vector that represents the up direction
    up: Vector3f,

}

const DEFAULT_FOV : f32 = 70.0;

impl Camera {
    fn new(position : Vector3f, target : Vector3f, up: Vector3f) -> Self {
        Camera {world_position: position,
                target_position: target,
                fov : DEFAULT_FOV,
                up: up,
        }
    }

    fn setup(&mut self, fov : f32) {
        self.fov = fov;
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct World {
    /// The base vector of the world :
    /// the 3rd one is UP (aka we are in XYZ configuration)
    base_vector : [Vector3<f32>; 3],
    /// A Vec containing all the cameras in the world
    cameras : Vec<Camera>,

    objects : Vec<obj3D::Object>,
}

impl World {
    /// This method create a camera at <position>, aiming at <target>
    pub fn add_camera(self : & mut World, position : Vector3f, target : Vector3f) {
        self.cameras.push(Camera::new(position, target,self.base_vector[2]));
    }

    /// Load all objects meshes
    fn load_objects(& mut self) {
        for obj in &mut self.objects {
            obj.load_mesh();
        }
    }

    //Generates a new empty world
    pub fn new_empty() -> World {
        let base_vector = [Vector3::new(1_f32,0_f32,0_f32),Vector3::new(0_f32,1_f32,0_f32),Vector3::new(0_f32,0_f32,1_f32)];
        World{base_vector:base_vector,
              cameras:vec!(),
              objects:vec!()}
    }

    //Add an object to the world
    pub fn add_object(& mut self,color:RGBA32,pos:Vector3f,path:String) {
        self.objects.push(Object::new(color,pos,path));
    }

    pub fn save_to_file(&self,file:&str) {
        match write_string_to_file(&serde_json::to_string_pretty(&self).unwrap() ,file.to_string()) {

            Err(e) =>println!("Could not save world. Error : {}",e),

            Ok(_) =>println!("World sucessfully saved"),

        }
    }
    pub fn load_from_file(file: &str) -> World {
        println!("Loading scene from file : {} ", file);
        let file = open_file_as_string(file);
        let mut world : World = match serde_json::from_str(file.as_str()) {
            Ok(val) => val,
            Err(e) => panic!("Error while loading world. Serde error is : {}",e),
       };
       world.load_objects();
       world
    }
}
