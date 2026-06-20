#![cfg_attr(debug_assertions, allow(dead_code))]
#![warn(clippy::unwrap_used, clippy::expect_used, clippy::empty_structs_with_brackets)]
#![allow(clippy::too_many_arguments, clippy::module_inception)]

extern crate self as opengl_engine;

pub mod engine;

pub mod error {
    pub use ::error::*;

    create_error!();

    #[derive(Error, Debug)]
    #[error("Uninitialized.")]
    pub struct Uninitialized;

    #[derive(Error, Debug)]
    #[error("Index {index:?} out of bounds ({bounds:?}).")]
    pub struct OutOfBounds<Idx: std::fmt::Debug> { pub index: Idx, pub bounds: std::ops::Range<Idx> }

    #[derive(Error, Debug)]
    #[error("Unwrap called on a None value.")]
    pub struct NoneValue;

    pub trait TryUnwrap<T>
    where
        Self: Sized
    {
        fn try_unwrap(self) -> Result<T, NoneValue>;
    }

    impl<T> TryUnwrap<T> for Option<T> {
        fn try_unwrap(self) -> Result<T, NoneValue> {
            Ok(self.ok_or(NoneValue)?)
        }
    }
    
    pub trait ExplicitUnwrap<T> {
        fn explicit_unwrap(self) -> T;
        fn explicit_expect(self, message: &str) -> T;
    }

    impl<T> ExplicitUnwrap<T> for Option<T> {
        #[allow(clippy::unwrap_used)]
        fn explicit_unwrap(self) -> T {
            self.unwrap()
        }
        
        #[allow(clippy::expect_used)]
        fn explicit_expect(self, msg: &str) -> T {
            self.expect(msg)
        }
    }

    impl<T, E: std::fmt::Debug> ExplicitUnwrap<T> for std::result::Result<T, E> {
        #[allow(clippy::unwrap_used)]
        fn explicit_unwrap(self) -> T {
            self.unwrap()
        }
        
        #[allow(clippy::expect_used)]
        fn explicit_expect(self, msg: &str) -> T {
            self.expect(msg)
        }
    }
}