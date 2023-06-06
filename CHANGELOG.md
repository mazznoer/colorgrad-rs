# Changelog

## [Unreleased](https://github.com/mazznoer/colorgrad-rs/compare/v0.6.2...HEAD)

### Added

- `BlendMode::Lab`, optional feature, can be enabled using `features = ["lab"]` in Cargo.toml

### Changed

- `f64` -> `f32`.
- `GimpGradient` is now a optional feature, can be enabled using `features = ["ggr"]` in Cargo.toml
- Preset gradients move to submodule `preset`.
- In previous version `Gradient` is a struct holding `LinearGradient`, `BasisGradient`, etc in a `Box`.
  Now `Gradient` is a trait. `LinearGradient`, `BasisGradient`, etc is now exposed directy,
  and they are implementing `Gradient` trait.
- `CustomGradient` renamed to `GradientBuilder`
- `CustomGradientError` renamed to `GradientBuilderError`

### Removed

- `BlendMode::Hsv`

### Fixed

- Error parsing GIMP gradient with UTF-8 BOM.

