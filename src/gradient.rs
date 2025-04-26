mod basis;
mod boxed;
mod catmull_rom;
mod inverse;
mod linear;
mod sharp;

pub use basis::BasisGradient;
pub use catmull_rom::CatmullRomGradient;
pub use inverse::InverseGradient;
pub use linear::LinearGradient;
pub use sharp::SharpGradient;

#[cfg(feature = "ggr")]
mod gimp;

#[cfg(feature = "ggr")]
pub use gimp::{GimpGradient, ParseGgrError};
