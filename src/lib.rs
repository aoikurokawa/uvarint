mod decode;
mod encode;
mod error;
pub mod io;

pub use decode::{decode_u32, decode_u64, decode_u128};
pub use encode::{
    encode_u32, encode_u32_into, encode_u64, encode_u64_into, encode_u128, encode_u128_into,
};
pub use error::UVarintError;
