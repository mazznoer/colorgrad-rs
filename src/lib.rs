#![cfg_attr(
    all(feature = "preset", feature = "named-colors"),
    doc = r##"
# Overview

Rust color scales library for data visualization, charts, games, maps, generative art and others.

## Usage

Using preset gradient:

```
use colorgrad::Gradient;

let g = colorgrad::preset::rainbow();

assert_eq!(g.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
assert_eq!(g.at(0.5).to_rgba8(), [175, 240, 91, 255]);
assert_eq!(g.at(0.5).to_css_hex().to_string(), "#aff05b");

for color in g.colors(20) {
    println!("{:?}", color.to_rgba8());
}
```

Custom gradient:

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use colorgrad::{Color, GradientBuilder, LinearGradient};
use colorgrad::Gradient;

let g = GradientBuilder::new()
    .colors(&[
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_rgba8(0, 255, 0, 255),
    ])
    .build::<LinearGradient>()?;

for color in g.colors(20) {
    println!("{:?}", color.to_rgba8());
}
# Ok(())
# }
```

Using HTML color format:

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use colorgrad::{GradientBuilder, LinearGradient};
let g = GradientBuilder::new()
    .html_colors(&["red", "#abc", "gold"])
    .build::<LinearGradient>()?;
# Ok(())
# }
```

Using CSS gradient format:

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use colorgrad::{GradientBuilder, LinearGradient};
let g = GradientBuilder::new()
    .css("gold, 35%, #f00")
    .build::<LinearGradient>()?;
# Ok(())
# }
```

## Examples

### Gradient Image

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use colorgrad::Gradient;

let grad = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .build::<colorgrad::CatmullRomGradient>()?;

let width = 1500;
let height = 70;

let imgbuf = image::RgbaImage::from_fn(width, height, |x, _| {
    image::Rgba(grad.at(x as f32 / width as f32).to_rgba8())
});

imgbuf.save("gradient.png")?;
# Ok(())
# }
```

Example output:

![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-gradient.png)

### Colored Noise

```ignore
use colorgrad::Gradient;
use noise::NoiseFn;

let scale = 0.015;
let grad = colorgrad::preset::rainbow().sharp(5, 0.15);
let ns = noise::OpenSimplex::new();

let imgbuf = image::RgbaImage::from_fn(600, 350, |x, y| {
    let t = ns.get([x as f32 * scale, y as f32 * scale]);
    let t = remap(t, -0.5, 0.5, 0.0, 1.0);
    image::Rgba(grad.at(t).to_rgba8())
});

imgbuf.save("noise.png")?;

// Map t which is in range [a, b] to range [c, d]
fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    (t - a) * ((d - c) / (b - a)) + c
}
```

Example output:

![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-noise.png)

## Preset Gradients

[See here](https://github.com/mazznoer/colorgrad-rs/blob/master/PRESET.md)

"##
)]
#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub use csscolorparser::Color;
pub use csscolorparser::ParseColorError;

mod core;
pub use core::BlendMode;
pub use core::Gradient;
pub use core::GradientColors;

mod builder;
pub use builder::GradientBuilder;
pub use builder::GradientBuilderError;

mod gradient;
pub use gradient::*;

#[cfg(feature = "preset")]
pub mod preset;

mod css_gradient;
mod linearize;
mod utils;
