#![allow(unused_imports, unused_variables, dead_code, unreachable_code)]

// cargo run --example gradients --all-features --release

use std::fs;
use std::io::BufReader;
use std::path::Path;

use colorgrad::{Color, GimpGradient};

mod gradients;
mod util;

use util::{grad_rgb_plot, gradient_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new("example_output/");

    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("Failed to create example_output/ directory.");
    }

    for (grad, name) in gradients::preset() {
        let imgbuf = grad_rgb_plot(&*grad, 1000, 150, 10, None);
        let file_path = format!("example_output/preset_{name}.png");
        println!("{file_path}");
        imgbuf.save(file_path)?;
    }

    for (grad, name) in gradients::blend_mode() {
        let imgbuf = grad_rgb_plot(&*grad, 1000, 150, 10, None);
        let file_path = format!("example_output/mode_{name}.png");
        println!("{file_path}");
        imgbuf.save(file_path)?;
    }

    for (grad, name) in gradients::interpolation() {
        let imgbuf = grad_rgb_plot(&*grad, 1000, 150, 10, None);
        let file_path = format!("example_output/interpolation_{name}.png");
        println!("{file_path}");
        imgbuf.save(file_path)?;
    }

    for (grad, name) in gradients::sharp() {
        let imgbuf = grad_rgb_plot(&*grad, 1000, 150, 10, None);
        let file_path = format!("example_output/{name}.png");
        println!("{file_path}");
        imgbuf.save(file_path)?;
    }

    // GIMP gradients

    for item in Path::new("examples/ggr/").read_dir()? {
        let path = item.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "ggr" {
                let fname = path.file_name().unwrap().to_str().unwrap();
                let input = fs::File::open(&path)?;
                let col = Color::default();
                let gradient = GimpGradient::new(BufReader::new(input), &col, &col)?;
                let imgbuf = grad_rgb_plot(&gradient, 1000, 150, 10, None);
                let file_path = format!("example_output/ggr_{fname}.png");
                println!("{file_path} ({})", gradient.name());
                imgbuf.save(file_path)?;
            }
        }
    }

    Ok(())
}
