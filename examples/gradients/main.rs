#![allow(unused_imports, unused_variables, dead_code, unreachable_code)]

// cargo run --example gradients --all-features --release

use std::fs;
use std::io::BufReader;
use std::path::Path;

use colorgrad::{Color, GimpGradient, Gradient};

mod gradients;
mod util;

use util::{grad_rgb_plot, gradient_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new("example_output/");

    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("Failed to create example_output/ directory.");
    }

    let css_gradients = [
        "red, blue",
        "red 30%, blue 70%",
        "red, 75%, blue",
        "red, yellow, lime, aqua, blue, magenta, red",
        "tomato 0% 50%, gold 50%, tomato",
        "purple 50%, deeppink 50%, gold, seagreen",
        "blue, cyan, gold, purple 50%, tomato 50%",
        "seagreen 30%, gold 0 70%, deeppink 0",
        "rgb(40, 230, 65) 10%, hotpink, steelblue",
        "rgb(255, 0, 0) 0% 50%, rgb(0, 0, 255), red, lime",
        "red, #f000",
        "red, 75%, #f000",
        "red -100, yellow, lime, aqua, blue, magenta, red 100",
        "red, lime -10, blue 15, gold",
    ];

    println!("--- CSS Gradients");
    println!();

    for (i, s) in css_gradients.iter().enumerate() {
        println!("input \"{s}\"");

        let g = colorgrad::GradientBuilder::new()
            .css(s)
            .mode(colorgrad::BlendMode::Rgb)
            .build::<colorgrad::CatmullRomGradient>();

        if let Ok(grad) = g {
            println!("domain {:?}", grad.domain());
            let imgbuf = grad_rgb_plot(&grad, 1000, 150, 10, None);
            let file_path = format!("example_output/css_{i}.png");
            println!("{file_path}");
            imgbuf.save(file_path)?;
        } else {
            println!("error");
        }
        println!();
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
