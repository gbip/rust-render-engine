#![feature(proc_macro)]
mod math;
#[allow(non_snake_case)] 
mod obj3D;
mod scene;
mod render;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use scene::World;

fn main() {
    let world = World::new();
    world.save_world_to_file("world1.json");    

}
