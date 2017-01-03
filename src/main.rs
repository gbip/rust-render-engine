#![feature(proc_macro)]
mod math;
mod obj3D;
mod scene;
mod render;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;



fn main() {
    println!("Hello, world!");
}
