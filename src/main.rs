#![feature(proc_macro)]
#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate lib_render;
use lib_render::*;
// Generate a template file at the location [path]
fn generate_template(path: &str) {
    let mut scene = Scene::new_empty();
    scene
        .world
        .add_camera(Vector3::new(0_f32, 0_f32, 5_f32),
                    Vector3::new(10_f32, 0_f32, 0_f32));

    scene
        .world
        .add_object(Vector3::new(10_f32, 0.56_f32, 2_f32),
                    "models/plane_no_uv.obj".to_string(),
                    "Example".to_string());

    scene.save_to_file(path);

    let material_solid = material::flat_material::FlatMaterial::new_empty();
    material_solid.save_to_file("template_material_solid.json");

    let mut material_tex = material::flat_material::FlatMaterial::new_empty();
    material_tex.diffuse = Channel::TextureMap { texture: TextureMap::new_empty() };
    material_tex.specular = Channel::TextureMap { texture: TextureMap::new_empty() };
    material_tex.ambient = Channel::TextureMap { texture: TextureMap::new_empty() };
    material_tex.save_to_file("template_material_texture.json");

}


// Usage of the program :
// 2 functions : generate a template for a scene, and render a scene
//  -> Generate a template :
//      -g [PATH] or --generate [PATH]
//  -> To load a scene :
//       Specify the input scene (needed)
//      -r [PATH] or --read [PATH]
//      Specify the output file
//      -w [PATH] or --write output [PATH] (optional)
fn parse_arg() {
    let mut options = Options::new();

    // Generating a template
    options.optflagopt("g",
                       "generate",
                       "Generate a template for creating a new scene",
                       "FILE");

    // Rendering :
    // -> Load a file
    options.optflagopt("r",
                       "read",
                       "Open a scene file (.json) for rendering",
                       "FILE");

    // -> Set the output file
    options.optflagopt("w", "write", "Save the rendered image to a file", "FILE");

    // Collecting the argument from the environnement
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let matches = match options.parse(args) {
        Ok(val) => val,
        Err(e) => {
            show_usage(&program);
            panic!(e.to_string());
        }
    };

    let at_least_one_option = matches.opt_present("g") || matches.opt_present("w") ||
                              matches.opt_present("r");

    if !at_least_one_option {
        show_usage(&program);
        return;
    }

    // Handling the template case
    if matches.opt_present("g") {
        generate_template(match matches.opt_str("g") {
                              Some(ref path) => path,
                              None => "template.json",
                          });
    }

    // Handling the case where we need to render
    if matches.opt_present("w") {
        let output_path = match matches.opt_str("w") {
            Some(path) => path,
            None => "untitled.png".to_string(),
        };
        if matches.opt_present("r") {
            let input_path = match matches.opt_str("r") {
                Some(path) => path,
                None => {
                    show_usage(&program);
                    return;
                }
            };
            render(&input_path, &output_path);
        } else {
            show_usage(&program);
        }
    }
}

// Affiche dans la console comment effectuer le rendu
fn show_usage(program: &str) {
    println!("Usage : {} -g FILE -r FILE -w FILE", program);
    println!("-g FILE or --generate FILE : Generate a template file in the location FILE for \
              creating a scene");
    println!("-r FILE or --read FILE : Read FILE to load the scene before rendering. Needed for \
              rendering, without a scene specified, the program will not render.");
    println!("-w FILE or --write FILE : Write the output to FILE. The default is 'untitled.png'");
}

fn test_image() {
    let image = Image::<RGBAPixel>::new(500, 600);
    image.write_to_file("object.png");
}

// La fonction que l'on appelle pour effectuer le rendu.
fn render(input: &str, output: &str) {
    let scene = Scene::load_from_file(input);
    scene.render_to_file(output);
}

fn main() {
    parse_arg();
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use super::*;

    // Generate a template, render it, and then remove it.
    #[ignore]
    #[test]
    fn test_template_generation_and_loading() {
        generate_template("test");
        Scene::load_from_file("test");
        Command::new("rm")
            .arg("test")
            .output()
            .expect("Error can't remove file test");

    }
}
