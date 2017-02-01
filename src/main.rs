#![feature(proc_macro)]
#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
mod math;
#[allow(non_snake_case)]
mod obj3D;
mod scene;
mod render;
mod ray;
mod color;
mod img;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate image;

use scene::World;
use img::Image;
use math::Vector3;
use color::RGBA32;

fn test_image() {
    let image = Image::<RGBA32>::new(500, 600);
    image.write_to_file("object.png");
}

fn main() {
    let mut world = World::new_empty();

    //test_image();

    world.add_object(RGBA32::new_black(),
                    Vector3::new(42_f32,0.56_f32,23.2_f32),
                    "models/plane_no_uv.obj".to_string());

    world.save_world_to_file("world1.json");
}
