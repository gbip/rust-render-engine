#![feature(proc_macro)]
#![allow(dead_code)]
mod math;
#[allow(non_snake_case)]
mod obj3D;
mod scene;
mod render;
mod ray;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate image;

use scene::World;
use render::{Color8, Color};
use math::Vector3;
use obj3D::{Mesh};

fn main() {
    let mut world = World::new_empty();

    world.add_object(Color8::new_neutral(),
                    Vector3::new(42_f32,0.56_f32,23.2_f32),
                    "/home/paul/repos/rust-render-engine/models/plane_no_uv.obj".to_string());

    //world.save_world_to_file("world1.json");
}
