mod basis;
mod catmull_rom;
mod inverse;
mod linear;
mod sharp;
mod smoothstep;

pub use basis::BasisGradient;
pub use catmull_rom::CatmullRomGradient;
pub use inverse::InverseGradient;
pub use linear::LinearGradient;
pub use sharp::SharpGradient;
pub use smoothstep::SmoothstepGradient;

#[cfg(all(feature = "ggr", feature = "std"))]
mod gimp;

#[cfg(all(feature = "ggr", feature = "std"))]
pub use gimp::{GimpGradient, ParseGgrError};
