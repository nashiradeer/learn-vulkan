use std::result::Result as StdResult;

pub mod bindings;

mod error;
mod instance;
mod sys;

pub use error::*;
pub use instance::*;
pub use sys::*;

pub fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

pub type Result<T> = StdResult<T, Error>;
