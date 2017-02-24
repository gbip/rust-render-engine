#![feature(test)]
#![feature()]
extern crate lib_render;
extern crate test;
extern crate colored;
use std::time::Instant;
use lib_render::*;
use colored::*;

const OUTPUT_FOLDER: &'static str = "bench_output/";

fn bench_once<T, F>(mut f: F) -> f64
    where F: FnMut() -> T
{
    let now = Instant::now();
    f();
    now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 10f64.powi(-9)
}

fn print_bench<F, T>(name: &str, f: F)
    where F: FnMut() -> T
{
    println!("\n        Executing {} \n", name.bold().yellow());
    println!("\n        Elapsed time for {} : {} s\n",
             name.bold().green(),
             bench_once(f));
}


fn main() {
    let name = "bench_render_suzanne_low";
    let scene: Scene = Scene::load_from_file("bench/scenes/suzanne_low.json".to_string());
    print_bench(name,
                || scene.render_to_file(OUTPUT_FOLDER.to_owned() + name));
}
