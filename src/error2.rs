use thiserror::Error;
use winit::error::EventLoopError;

type BT = backtrace::Backtrace;

pub trait EngineError: std::error::Error {}

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct ErrorMessage(pub &'static str);

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct DynamicErrorMessage(pub String);

impl EngineError for ErrorMessage {}
impl EngineError for DynamicErrorMessage {}
impl EngineError for std::io::Error {}
impl EngineError for image::ImageError {}
impl EngineError for EventLoopError {}

#[derive(Error, Debug)]
pub struct Error<E: EngineError> {
    source: E,
    backtrace: BT
}

impl<E: EngineError> Error<E> {
    pub fn new(error: E) -> Error<E> {
        Error { source: error, backtrace: BT::new_unresolved() }
    }

    pub fn from_existing<E2: EngineError + Into<E>>(error: Error<E2>) -> Error<E> {
        let Error { source, backtrace } = error;
        let source = source.into();

        Error { source, backtrace }
    }

    pub fn source(&self) -> &E {
        &self.source
    }

    pub fn backtrace(&self) -> &BT {
        &self.backtrace
    }
}

impl<E1: EngineError, E2: EngineError + From<E1>> From<E1> for Error<E2> {
    fn from(value: E1) -> Self {
        Self::new(value.into())
    }
}

impl From<&'static str> for Error<ErrorMessage> {
    fn from(value: &'static str) -> Self {
        Error::new(ErrorMessage(value))
    }
}

impl From<String> for Error<DynamicErrorMessage> {
    fn from(value: String) -> Self {
        Error::new(DynamicErrorMessage(value))
    }
}

pub type Result<T, E> = std::result::Result<T, Error<E>>;

#[derive(Error, Debug)]
#[error("Unwrapped a None value.")]
pub struct NoneValue;
impl EngineError for NoneValue {}

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

pub mod dyn_error {
    use thiserror::Error;

    use crate::error2::{DynamicErrorMessage, EngineError, ErrorMessage};

    type BT = backtrace::Backtrace;

    #[derive(Error, Debug)]
    #[error("{source}")]
    pub struct Error {
        source: Box<dyn std::error::Error>,
        backtrace: BT
    }

    impl Error {
        pub fn new<E: EngineError + 'static>(error: E) -> Error {
            Error { source: Box::new(error), backtrace: BT::new_unresolved() }
        }

        pub fn source(&self) -> &dyn std::error::Error {
            & *self.source
        }

        pub fn resolve_backtrace(&mut self) {
            self.backtrace.resolve();
        }

        pub fn backtrace(&self) -> &BT {
            &self.backtrace
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;

    impl<T: EngineError + 'static> From<super::Error<T>> for Error {
        fn from(value: super::Error<T>) -> Self {
            Self { source: Box::new(value.source), backtrace: value.backtrace }
        }
    }

    impl<T: EngineError + 'static> From<T> for Error {
        fn from(value: T) -> Self {
            Error::new(value)
        }
    }

    impl From<&'static str> for Error {
        fn from(value: &'static str) -> Self {
            Error::new(ErrorMessage(value))
        }
    }

    impl From<String> for Error {
        fn from(value: String) -> Self {
            Error::new(DynamicErrorMessage(value))
        }
    }
}

// TODO: Find a better place for this

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

pub mod universal_errors {
    use std::{fmt::Debug, ops::Range};

    use thiserror::Error;

    use crate::error2::EngineError;

    #[derive(Error, Debug)]
    #[error("Index {index:?} out of bounds ({bounds:?}).")]
    pub struct OutOfBounds<Idx: Debug> { pub index: Idx, pub bounds: Range<Idx> }
    impl<Idx: Debug> EngineError for OutOfBounds<Idx> {}

    #[derive(Error, Debug)]
    #[error("Uninitialized.")]
    pub struct Uninitialized;
    impl EngineError for Uninitialized {}
}