/*!
see https://github.com/thewebdevel/codeframe for high level description of the library
*/

pub mod capture;
mod codeframe_builder;
pub mod color;
mod utils;

pub mod codeframe_macro;
pub use crate::codeframe_builder::Codeframe;

pub use color::Color;
