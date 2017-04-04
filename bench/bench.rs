#![feature(test)]
#![feature()]
extern crate lib_render;
extern crate test;
extern crate colored;
use std::time::Instant;
use lib_render::*;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;

const OUTPUT_FOLDER: &'static str = "bench_output/";

fn write_bench_to_file(name: &str, time: f64) {

    let file_name = format!("{}/{}_{:?}",
                            OUTPUT_FOLDER,
                            name,
                            std::time::UNIX_EPOCH.elapsed().unwrap().as_secs());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_name)
        .expect(format!("Error : can't open {} for writing benchmarks results",
                        file_name)
                        .as_str());
    file.write_all(format!("{} was executed in: {} \n", name, time).as_bytes())
        .expect(format!("Error while writing the benchmark results into {}",
                        file_name)
                        .as_str());


}

fn bench_once<T, F>(mut f: F) -> f64
    where F: FnMut() -> T
{
    let now = Instant::now();
    f();
    now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 10f64.powi(-9)
}

fn do_bench<F, T>(name: &str, f: F)
    where F: FnMut() -> T
{
    println!("\n        Executing {} \n", name.bold().yellow());
    let time = bench_once(f);
    println!("\n        Elapsed time for {} : {} s\n",
             name.bold().green(),
             time);

    write_bench_to_file(name, time);
}


fn main() {
    let name_suz = "bench_render_suzanne_low";
    let scene_suz: Scene = Scene::load_from_file("bench/scenes/suzanne_low.json");
    let output_path = OUTPUT_FOLDER.to_string() + name_suz + ".png";
    do_bench(name_suz, || scene_suz.render_to_file(&output_path));

    let name_load = "bench_load_1k_sphere";
    do_bench(name_load,
             || Scene::load_from_file("bench/scenes/sphere_1k.json"));
}
