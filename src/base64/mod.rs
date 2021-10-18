mod encoder;
mod decoder;
mod table;

pub use encoder::base64_encode;
pub use decoder::{base64_decode, DecodeError};