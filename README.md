<h1 align="center">colorgrad :crab:</h1>

<p align="center">
<a href="https://github.com/mazznoer/colorgrad-rs"><img alt="Stars" src="https://img.shields.io/github/stars/mazznoer/colorgrad-rs?logo=github"></a>
<a href="https://github.com/mazznoer/colorgrad-rs"><img alt="License" src="https://img.shields.io/crates/l/colorgrad"></a>
<a href="https://crates.io/crates/colorgrad"><img alt="crates.io" src="https://img.shields.io/crates/v/colorgrad.svg"></a>
<a href="https://docs.rs/colorgrad"><img alt="Documentation" src="https://docs.rs/colorgrad/badge.svg"></a>
<a href="https://github.com/mazznoer/colorgrad-rs/actions"><img alt="Build Status" src="https://github.com/mazznoer/colorgrad-rs/actions/workflows/rust.yml/badge.svg"></a>
<a href="https://crates.io/crates/colorgrad"><img alt="Total Downloads" src="https://img.shields.io/crates/d/colorgrad.svg"></a>
</p>

<p align="center">
<a href="https://www.rust-lang.org/">Rust</a> color scales library for data visualization, charts, games, maps, generative art and others.
</p>

<p align="center">
    <strong>
        <a href="https://docs.rs/colorgrad">Documentation</a> • <a href="CHANGELOG.md">Changelog</a> • <a href="https://github.com/sponsors/mazznoer/">Donate</a>
    </strong>
</p>

<hr>

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
colorgrad = "0.7.0"
```

## Custom Gradient

### Basic

```rust
let g = colorgrad::GradientBuilder::new().build::<colorgrad::LinearGradient>()?;
```
![img](docs/images/custom-default.png)

### Custom Colors

```rust
use colorgrad::Color;

let g = colorgrad::GradientBuilder::new()
    .colors(&[
        Color::from_rgba8(0, 206, 209, 255),
        Color::from_rgba8(255, 105, 180, 255),
        Color::new(0.274, 0.5, 0.7, 1.0),
        Color::from_hsva(50.0, 1.0, 1.0, 1.0),
        Color::from_hsva(348.0, 0.9, 0.8, 1.0),
    ])
    .build::<colorgrad::LinearGradient>()?;
```
![img](docs/images/custom-colors.png)

### Using Web Color Format

`.html_colors()` method accepts [named colors](https://www.w3.org/TR/css-color-4/#named-colors), hexadecimal (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, and `hsv()`.

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["#C41189", "#00BFFF", "#FFD700"])
    .build::<colorgrad::LinearGradient>()?;
```
![img](docs/images/custom-hex-colors.png)

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["gold", "hotpink", "darkturquoise"])
    .build::<colorgrad::LinearGradient>()?;
```
![img](docs/images/custom-named-colors.png)

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["rgb(125,110,221)", "rgb(90%,45%,97%)", "hsl(229,79%,85%)"])
    .build::<colorgrad::LinearGradient>()?;
```
![img](docs/images/custom-css-colors.png)

### Using CSS Gradient Format

```rust
let g = colorgrad::GradientBuilder::new()
    .css("blue, cyan, gold, purple 70%, tomato 70%, 90%, #ff0")
    .build::<colorgrad::CatmullRomGradient>()?;
```
![img](docs/images/css-gradient.png)

### Domain & Color Position

Default domain is [0..1].

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .build::<colorgrad::LinearGradient>()?;

assert_eq!(g.domain(), (0.0, 1.0));
```
![img](docs/images/domain-default.png)

Set the domain to [0..100].

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0.0, 100.0])
    .build::<colorgrad::LinearGradient>()?;

assert_eq!(g.domain(), (0.0, 100.0));
```
![img](docs/images/domain-100.png)

Set the domain to [-1..1].

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[-1.0, 1.0])
    .build::<colorgrad::LinearGradient>()?;

assert_eq!(g.domain(), (-1.0, 1.0));
```
![img](docs/images/domain-neg1-1.png)

Set exact position for each color. The domain is [0..1].

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0.0, 0.7, 1.0])
    .build::<colorgrad::LinearGradient>()?;

assert_eq!(g.domain(), (0.0, 1.0));
```
![img](docs/images/color-position-1.png)

Set exact position for each color. The domain is [15..80].

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[15.0, 30.0, 80.0])
    .build::<colorgrad::LinearGradient>()?;

assert_eq!(g.domain(), (15.0, 80.0));
```
![img](docs/images/color-position-2.png)

### Blending Mode

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["#FFF", "#00F"])
    .mode(colorgrad::BlendMode::Rgb)
    .build::<colorgrad::LinearGradient>()?;
```

![Blending Modes](docs/images/blend-modes-v0.7.png)

### Interpolation Mode

```rust
let g = colorgrad::GradientBuilder::new()
    .html_colors(&["#C41189", "#00BFFF", "#FFD700"])
    .build::<colorgrad::BasisGradient>()?;
```

![Interpolation Modes](docs/images/interpolation-modes.png)

## Preset Gradients

See [PRESET.md](PRESET.md)

## Parsing GIMP Gradient

```rust
use colorgrad::{Color, GimpGradient};
use std::fs::File;
use std::io::BufReader;

let input = File::open("examples/Abstract_1.ggr")?;
let buf = BufReader::new(input);
let col = Color::default();
let grad = GimpGradient::new(buf, &col, &col)?;

assert_eq!(grad.name(), "Abstract 1");
```

![img](docs/images/ggr_abstract_1.png)

## Using the Gradient

### Get the domain

```rust
let grad = colorgrad::preset::rainbow();

assert_eq!(grad.domain(), (0.0, 1.0));
```

### Get single color at certain position

```rust
use colorgrad::Gradient;

let grad = colorgrad::preset::blues();

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
use colorgrad::Gradient;

let grad = colorgrad::preset::rainbow();

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
let g = colorgrad::preset::rainbow().sharp(11, 0.0);
```
![img](docs/images/rainbow-sharp.png)

This is the effect of different smoothness.

![img](docs/images/sharp-gradients.png)

## Examples

### Gradient Image

```rust
use colorgrad::Gradient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 1300.0;
    let height = 70.0;

    // custom gradient
    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build::<colorgrad::CatmullRomGradient>()?;

    let imgbuf = image::ImageBuffer::from_fn(width as u32, height as u32, |x, _| {
        image::Rgba(grad.at(x as f32 / width).to_rgba8())
    });
    imgbuf.save("gradient.png")?;

    Ok(())
}
```

Example output:

![img](docs/images/example-gradient.png)

### Colored Noise

```rust
use colorgrad::Gradient;
use noise::NoiseFn;

fn main() {
    let scale = 0.015;

    let grad = colorgrad::preset::rainbow().sharp(5, 0.15);
    let ns = noise::OpenSimplex::new(0);

    let imgbuf = image::ImageBuffer::from_fn(600, 350, |x, y| {
        let t = ns.get([x as f64 * scale, y as f64 * scale]);
        image::Rgba(grad.at(norm(t as f32, -0.5, 0.5)).to_rgba8())
    });
    imgbuf.save("noise.png").unwrap();
}

fn norm(t: f32, a: f32, b: f32) -> f32 {
    (t - a) * (1.0 / (b - a))
}
```

Example output:

![img](docs/images/example-noise.png)

## Features

### Default

* __named-colors__: Enables parsing from [named colors](https://www.w3.org/TR/css-color-4/#named-colors). Requires [`phf`](https://crates.io/crates/phf).
* __preset__: Preset gradients.

Can be disabled using `default-features = false`.

### Optional

* __lab__: Blending colors in Lab colorspace.
* __ggr__: Parsing GIMP gradient format.

## Similar Projects

* [colorgrad](https://github.com/mazznoer/colorgrad) (Go version of this library)
* [colorous](https://github.com/dtolnay/colorous) (Rust)
* [chroma.js](https://gka.github.io/chroma.js/#color-scales) (Javascript)
* [d3-scale-chromatic](https://github.com/d3/d3-scale-chromatic/) (Javascript)

## Projects using `colorgrad`

* [binocle](https://github.com/sharkdp/binocle) - A graphical tool to visualize binary data
* [bytehound](https://github.com/koute/bytehound) - A memory profiler for Linux
* [cosmic-bg](https://github.com/pop-os/cosmic-bg/) - COSMIC session service which applies backgrounds to displays
* [eruption](https://github.com/X3n0m0rph59/eruption) - A Linux user-mode input and LED driver for keyboards, mice and other devices
* [gradient](https://github.com/mazznoer/gradient-rs) - A command line tool for playing with color gradient
* [lcat](https://github.com/davidkna/lcat-rs) - `lolcat` clone
* [lolcrab](https://github.com/mazznoer/lolcrab) - `lolcat` but with noise (`lcat` fork)
* [rust-fractal](https://github.com/rust-fractal/rust-fractal-core) - Mandelbrot fractal visualizer
* [WezTerm](https://github.com/wez/wezterm) - A GPU-accelerated cross-platform terminal emulator and multiplexer
* and [many others..](https://github.com/mazznoer/colorgrad-rs/network/dependents)

## Links

* [Color Blindness Simulator](https://www.color-blindness.com/coblis-color-blindness-simulator/)
* [Visual System Simulator](https://github.com/UniStuttgart-VISUS/visual-system-simulator)

