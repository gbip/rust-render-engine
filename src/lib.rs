#![feature(proc_macro)]
#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
pub mod math;


#[macro_use] // Il faut charger angle.rs dans les premiers modules, car sinon obj3D n'aura pas la visibilité sur les macros définies dans angle.
pub mod angle;
#[allow(non_snake_case)]
pub mod obj3D;
pub mod scene;
pub mod render;
pub mod ray;
pub mod color;
pub mod img;
pub mod io_utils;
pub mod material;
#[macro_use]
pub extern crate serde_derive;
pub extern crate serde_json;
pub extern crate serde;
pub extern crate image;
pub extern crate getopts;
pub extern crate num;
pub extern crate rayon;
pub extern crate colored;

pub use scene::Scene;
pub use img::Image;
pub use color::{RGBA8, RGBA32};
pub use math::Vector3;
pub use getopts::Options;
pub use std::env;
pub use io_utils::*;
