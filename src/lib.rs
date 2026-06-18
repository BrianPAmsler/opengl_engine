#![cfg_attr(debug_assertions, allow(dead_code))]
#![warn(clippy::unwrap_used, clippy::expect_used, clippy::empty_structs_with_brackets)]
#![allow(clippy::too_many_arguments, clippy::module_inception)]

pub mod engine;
pub mod error2;

pub use error;