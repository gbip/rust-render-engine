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

use scene::World;

fn main() {
    let world = World::new_empty();
    world.save_world_to_file("world1.json");

}
