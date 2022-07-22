# colorgrad-rs

[![Stars](https://img.shields.io/github/stars/mazznoer/colorgrad-rs?logo=github)](https://github.com/mazznoer/colorgrad-rs)
[![License](https://img.shields.io/crates/l/colorgrad)](https://github.com/mazznoer/colorgrad-rs)
[![crates.io](https://img.shields.io/crates/v/colorgrad.svg)](https://crates.io/crates/colorgrad)
[![Documentation](https://docs.rs/colorgrad/badge.svg)](https://docs.rs/colorgrad)
[![Build Status](https://github.com/mazznoer/colorgrad-rs/workflows/Rust/badge.svg)](https://github.com/mazznoer/colorgrad-rs/actions)
[![codecov](https://codecov.io/gh/mazznoer/colorgrad-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/mazznoer/colorgrad-rs)
[![Total Downloads](https://img.shields.io/crates/d/colorgrad.svg)](https://crates.io/crates/colorgrad)

[Rust](https://www.rust-lang.org/) color scales library for data visualization, charts, games, maps, generative art and others.

## Support This Project

[![Donate](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/mazznoer/donate)

## Index

+ [Custom Gradient](#custom-gradient)
+ [Preset Gradients](#preset-gradients)
+ [Parsing GIMP Gradient](#parsing-gimp-gradient)
+ [Using the Gradient](#using-the-gradient)
+ [Examples](#examples)
+ [Similar Projects](#similar-projects)
+ [Projects using `colorgrad`](#projects-using-colorgrad)

## Usage

Add this to your `Cargo.toml`

```toml
colorgrad = "0.6.1"
```

## Custom Gradient

### Basic

```rust
let g = colorgrad::CustomGradient::new().build()?;
```
![img](docs/images/custom-default.png)

### Custom Colors

```rust
use colorgrad::Color;

let g = colorgrad::CustomGradient::new()
    .colors(&[
        Color::from_rgba8(0, 206, 209, 255),
        Color::from_rgba8(255, 105, 180, 255),
        Color::new(0.274, 0.5, 0.7, 1.0),
        Color::from_hsva(50.0, 1.0, 1.0, 1.0),
        Color::from_hsva(348.0, 0.9, 0.8, 1.0),
    ])
    .build()?;
```
![img](docs/images/custom-colors.png)

### Using Web Color Format

`.html_colors()` method accepts [named colors](https://www.w3.org/TR/css-color-4/#named-colors), hexadecimal (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, and `hsv()`.

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#C41189", "#00BFFF", "#FFD700"])
    .build()?;
```
![img](docs/images/custom-hex-colors.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["gold", "hotpink", "darkturquoise"])
    .build()?;
```
![img](docs/images/custom-named-colors.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["rgb(125,110,221)", "rgb(90%,45%,97%)", "hsl(229,79%,85%)"])
    .build()?;
```
![img](docs/images/custom-css-colors.png)

### Domain & Color Position

Default domain is [0..1].

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .build()?;

assert_eq!(g.domain(), (0.0, 1.0));
```
![img](docs/images/domain-default.png)

Set the domain to [0..100].

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0.0, 100.0])
    .build()?;

assert_eq!(g.domain(), (0.0, 100.0));
```
![img](docs/images/domain-100.png)

Set the domain to [-1..1].

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[-1.0, 1.0])
    .build()?;

assert_eq!(g.domain(), (-1.0, 1.0));
```
![img](docs/images/domain-neg1-1.png)

Set exact position for each color. The domain is [0..1].

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0.0, 0.7, 1.0])
    .build()?;

assert_eq!(g.domain(), (0.0, 1.0));
```
![img](docs/images/color-position-1.png)

Set exact position for each color. The domain is [15..80].

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[15.0, 30.0, 80.0])
    .build()?;

assert_eq!(g.domain(), (15.0, 80.0));
```
![img](docs/images/color-position-2.png)

### Blending Mode

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#FFF", "#00F"])
    .mode(colorgrad::BlendMode::Rgb)
    .build()?;
```

![Blending Modes](docs/images/blend-modes.png)

### Interpolation Mode

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#C41189", "#00BFFF", "#FFD700"])
    .interpolation(colorgrad::Interpolation::Linear)
    .build()?;
```

![Interpolation Modes](docs/images/interpolation-modes.png)

## Preset Gradients

All preset gradients are in the domain [0..1]. Uniform B-splines is used to interpolate the colors.

![img](docs/images/rgb-plot.png)

### Diverging

`colorgrad::br_bg()`
![img](docs/images/preset/br_bg.png)

`colorgrad::pr_gn()`
![img](docs/images/preset/pr_gn.png)

`colorgrad::pi_yg()`
![img](docs/images/preset/pi_yg.png)

`colorgrad::pu_or()`
![img](docs/images/preset/pu_or.png)

`colorgrad::rd_bu()`
![img](docs/images/preset/rd_bu.png)

`colorgrad::rd_gy()`
![img](docs/images/preset/rd_gy.png)

`colorgrad::rd_yl_bu()`
![img](docs/images/preset/rd_yl_bu.png)

`colorgrad::rd_yl_gn()`
![img](docs/images/preset/rd_yl_gn.png)

`colorgrad::spectral()`
![img](docs/images/preset/spectral.png)

### Sequential (Single Hue)

`colorgrad::blues()`
![img](docs/images/preset/blues.png)

`colorgrad::greens()`
![img](docs/images/preset/greens.png)

`colorgrad::greys()`
![img](docs/images/preset/greys.png)

`colorgrad::oranges()`
![img](docs/images/preset/oranges.png)

`colorgrad::purples()`
![img](docs/images/preset/purples.png)

`colorgrad::reds()`
![img](docs/images/preset/reds.png)

### Sequential (Multi-Hue)

`colorgrad::turbo()`
![img](docs/images/preset/turbo.png)

`colorgrad::viridis()`
![img](docs/images/preset/viridis.png)

`colorgrad::inferno()`
![img](docs/images/preset/inferno.png)

`colorgrad::magma()`
![img](docs/images/preset/magma.png)

`colorgrad::plasma()`
![img](docs/images/preset/plasma.png)

`colorgrad::cividis()`
![img](docs/images/preset/cividis.png)

`colorgrad::warm()`
![img](docs/images/preset/warm.png)

`colorgrad::cool()`
![img](docs/images/preset/cool.png)

`colorgrad::cubehelix_default()`
![img](docs/images/preset/cubehelix_default.png)

`colorgrad::bu_gn()`
![img](docs/images/preset/bu_gn.png)

`colorgrad::bu_pu()`
![img](docs/images/preset/bu_pu.png)

`colorgrad::gn_bu()`
![img](docs/images/preset/gn_bu.png)

`colorgrad::or_rd()`
![img](docs/images/preset/or_rd.png)

`colorgrad::pu_bu_gn()`
![img](docs/images/preset/pu_bu_gn.png)

`colorgrad::pu_bu()`
![img](docs/images/preset/pu_bu.png)

`colorgrad::pu_rd()`
![img](docs/images/preset/pu_rd.png)

`colorgrad::rd_pu()`
![img](docs/images/preset/rd_pu.png)

`colorgrad::yl_gn_bu()`
![img](docs/images/preset/yl_gn_bu.png)

`colorgrad::yl_gn()`
![img](docs/images/preset/yl_gn.png)

`colorgrad::yl_or_br()`
![img](docs/images/preset/yl_or_br.png)

`colorgrad::yl_or_rd()`
![img](docs/images/preset/yl_or_rd.png)

### Cyclical

`colorgrad::rainbow()`
![img](docs/images/preset/rainbow.png)

`colorgrad::sinebow()`
![img](docs/images/preset/sinebow.png)

## Parsing GIMP Gradient

```rust
use colorgrad::Color;
use std::fs::File;
use std::io::BufReader;

let input = File::open("examples/Abstract_1.ggr")?;
let buf = BufReader::new(input);
let fg = Color::new(0.0, 0.0, 0.0, 1.0);
let bg = Color::new(1.0, 1.0, 1.0, 1.0);
let (grad, name) = colorgrad::parse_ggr(buf, &fg, &bg)?;

assert_eq!(name, "Abstract 1");
```

![img](docs/images/ggr_abstract_1.png)

## Using the Gradient

### Get the domain

```rust
let grad = colorgrad::rainbow();

assert_eq!(grad.domain(), (0.0, 1.0));
```

### Get single color at certain position

```rust
let grad = colorgrad::blues();

assert_eq!(grad.at(0.0).to_rgba8(), [247, 251, 255, 255]);
assert_eq!(grad.at(0.5).to_rgba8(), [109, 174, 213, 255]);
assert_eq!(grad.at(1.0).to_rgba8(), [8,   48,  107, 255]);

assert_eq!(grad.at(0.3).to_rgba8(), grad.repeat_at(0.3).to_rgba8());
assert_eq!(grad.at(0.3).to_rgba8(), grad.reflect_at(0.3).to_rgba8());

assert_eq!(grad.at(0.7).to_rgba8(), grad.repeat_at(0.7).to_rgba8());
assert_eq!(grad.at(0.7).to_rgba8(), grad.reflect_at(0.7).to_rgba8());
```

The difference of `at()`, `repeat_at()` and `reflect_at()`.

![Spread Modes](docs/images/spread-modes.png)

### Get n colors evenly spaced across gradient

```rust
let grad = colorgrad::rainbow();

for c in grad.colors(10) {
    println!("{}", c.to_hex_string());
}
```

Output:

```console
#6e40aa
#c83dac
#ff5375
#ff8c38
#c9d33a
#7cf659
#5dea8d
#48b8d0
#4775de
#6e40aa
```

### Hard-Edged Gradient

Convert gradient to hard-edged gradient with 11 segments and 0 smoothness.

```rust
let g = colorgrad::rainbow().sharp(11, 0.0);
```
![img](docs/images/rainbow-sharp.png)

This is the effect of different smoothness.

![img](docs/images/sharp-gradients.png)

## Examples

### Gradient Image

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grad = colorgrad::CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build()?;

    let width = 1500;
    let height = 70;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgba = grad.at(x as f64 / width as f64).to_rgba8();
        *pixel = image::Rgba(rgba);
    }

    imgbuf.save("gradient.png")?;
    Ok(())
}
```

Example output:

![img](docs/images/example-gradient.png)

### Colored Noise

```rust
use noise::NoiseFn;

fn main() {
    let scale = 0.015;

    let grad = colorgrad::rainbow().sharp(5, 0.15);
    let ns = noise::OpenSimplex::new();
    let mut imgbuf = image::ImageBuffer::new(600, 350);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let t = ns.get([x as f64 * scale, y as f64 * scale]);
        let rgba = grad.at(remap(t, -0.5, 0.5, 0.0, 1.0)).to_rgba8();
        *pixel = image::Rgba(rgba);
    }

    imgbuf.save("noise.png").unwrap();
}

// Map t which is in range [a, b] to range [c, d]
fn remap(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    (t - a) * ((d - c) / (b - a)) + c
}
```

Example output:

![img](docs/images/example-noise.png)

## Default Feature

* __named-colors__: Enables parsing from [named colors](https://www.w3.org/TR/css-color-4/#named-colors). Requires [`phf`](https://crates.io/crates/phf). Can be disabled using `default-features = false`.

## Similar Projects

* [colorgrad](https://github.com/mazznoer/colorgrad) (Go version of this library)
* [colorous](https://github.com/dtolnay/colorous) (Rust)
* [chroma.js](https://gka.github.io/chroma.js/#color-scales) (Javascript)
* [d3-scale-chromatic](https://github.com/d3/d3-scale-chromatic/) (Javascript)

## Projects using `colorgrad`

* [binocle](https://github.com/sharkdp/binocle) - A graphical tool to visualize binary data
* [bytehound](https://github.com/koute/bytehound) - A memory profiler for Linux
* [eruption](https://github.com/X3n0m0rph59/eruption) - A Linux user-mode input and LED driver for keyboards, mice and other devices
* [gradient](https://github.com/mazznoer/gradient-rs) - A command line tool for playing with color gradient
* [lcat](https://github.com/davidkna/lcat-rs) - `lolcat` clone
* [lolcrab](https://github.com/mazznoer/lolcrab) - `lolcat` but with noise (`lcat` fork)
* [rust-fractal](https://github.com/rust-fractal/rust-fractal-core) - Mandelbrot fractal visualizer
* [WezTerm](https://github.com/wez/wezterm) - A GPU-accelerated cross-platform terminal emulator and multiplexer

## Links

* [Color Blindness Simulator](https://www.color-blindness.com/coblis-color-blindness-simulator/)
* [Visual System Simulator](https://github.com/UniStuttgart-VISUS/visual-system-simulator)
