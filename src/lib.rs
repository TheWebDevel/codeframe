/*!
see https://github.com/thewebdevel/codeframe for high level description of the library
*/

pub mod capture;
pub mod codeframe_builder;
pub mod codeframe_macro;
pub mod color;
mod utils;

pub use crate::capture as capture_codeframe;
pub use crate::codeframe_builder::Codeframe;

pub use color::Color;
