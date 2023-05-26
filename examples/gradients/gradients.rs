use colorgrad::{
    BasisGradient, BlendMode, CatmullRomGradient, Color, Gradient, GradientBuilder, LinearGradient,
};

macro_rules! preset {
    ($name:ident) => {
        (Box::new(colorgrad::preset::$name()), stringify!($name))
    };
}

pub fn preset() -> Vec<(Box<dyn Gradient>, &'static str)> {
    vec![
        preset!(sinebow),
        preset!(turbo),
        preset!(cividis),
        preset!(rainbow),
        preset!(cubehelix_default),
        preset!(warm),
        preset!(cool),
        preset!(viridis),
        preset!(inferno),
        preset!(magma),
        preset!(plasma),
        preset!(bu_gn),
        preset!(bu_pu),
        preset!(gn_bu),
        preset!(or_rd),
        preset!(pu_bu_gn),
        preset!(pu_bu),
        preset!(pu_rd),
        preset!(rd_pu),
        preset!(yl_gn_bu),
        preset!(yl_gn),
        preset!(yl_or_br),
        preset!(yl_or_rd),
        preset!(br_bg),
        preset!(pr_gn),
        preset!(pi_yg),
        preset!(pu_or),
        preset!(rd_bu),
        preset!(rd_gy),
        preset!(rd_yl_bu),
        preset!(rd_yl_gn),
        preset!(spectral),
        preset!(blues),
        preset!(greens),
        preset!(greys),
        preset!(oranges),
        preset!(purples),
        preset!(reds),
    ]
}

pub fn blend_mode() -> Vec<(Box<dyn Gradient>, &'static str)> {
    fn grad(mode: BlendMode) -> CatmullRomGradient {
        GradientBuilder::new()
            .html_colors(&["#fff", "#00f", "gold", "deeppink", "#000"])
            .mode(mode)
            .build::<CatmullRomGradient>()
            .unwrap()
    }
    vec![
        (Box::new(grad(BlendMode::Rgb)), "Rgb"),
        (Box::new(grad(BlendMode::LinearRgb)), "LinearRgb"),
        (Box::new(grad(BlendMode::Oklab)), "Oklab"),
        (Box::new(grad(BlendMode::Lab)), "Lab"),
    ]
}

pub fn interpolation() -> Vec<(Box<dyn Gradient>, String)> {
    let mut gradients: Vec<(Box<dyn Gradient>, String)> = Vec::new();
    let colors = ["#C41189", "#00BFFF", "#FFD700"];
    let modes = [
        BlendMode::Rgb,
        BlendMode::LinearRgb,
        BlendMode::Oklab,
        BlendMode::Lab,
    ];

    for mode in modes.iter() {
        let g = GradientBuilder::new()
            .html_colors(&colors)
            .mode(*mode)
            .build::<LinearGradient>()
            .unwrap();
        gradients.push((Box::new(g), format!("Linear_{mode:?}")));

        let g = GradientBuilder::new()
            .html_colors(&colors)
            .mode(*mode)
            .build::<CatmullRomGradient>()
            .unwrap();
        gradients.push((Box::new(g), format!("CatmullRom_{mode:?}")));

        let g = GradientBuilder::new()
            .html_colors(&colors)
            .mode(*mode)
            .build::<BasisGradient>()
            .unwrap();
        gradients.push((Box::new(g), format!("Basis_{mode:?}")));
    }

    gradients
}

pub fn sharp() -> Vec<(Box<dyn Gradient>, String)> {
    let mut gradients: Vec<(Box<dyn Gradient>, String)> = Vec::new();
    let grad = colorgrad::preset::rainbow();

    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let g = grad.sharp(13, t);
        gradients.push((Box::new(g), format!("sharp_{t}")));
    }
    gradients
}
