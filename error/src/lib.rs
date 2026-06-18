use std::{cell::RefCell, fmt::Display};

use backtrace::Backtrace;

pub use macros::*;

#[derive(Debug)]
pub struct Error<E: std::error::Error> {
    source: E,
    backtrace: RefCell<Backtrace>
}

impl<E: std::error::Error> Error<E> {
    pub fn new(error: E) -> Error<E> {
        Error { source: error, backtrace: RefCell::new(Backtrace::new_unresolved()) }
    }

    pub fn from_existing<E2: std::error::Error + Into<E>>(error: Error<E2>) -> Error<E> {
        let Error { source, backtrace } = error;
        let source = source.into();

        Error { source, backtrace }
    }

    pub fn source(&self) -> &E {
        &self.source
    }

    pub fn backtrace(&self) -> Backtrace {
        self.backtrace.borrow().clone()
    }
}

impl<E: std::error::Error> Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.backtrace.borrow_mut().resolve();
        write!(f, "Error: {}\n\nStack Backtrace\n{:?}", self.source, self.backtrace)
    }
}

impl<E: std::error::Error> std::error::Error for Error<E> {}

pub type Result<T, E> = std::result::Result<T, Error<E>>;

pub mod any {
    use std::{cell::RefCell, fmt::Display};

    use backtrace::Backtrace;

    #[derive(Debug)]
    pub struct Error {
        source: Box<dyn std::error::Error>,
        backtrace: RefCell<Backtrace>
    }

    impl Error {
        pub fn new<E: std::error::Error + 'static>(error: E) -> Error {
            Error { source: Box::new(error), backtrace: RefCell::new(Backtrace::new_unresolved()) }
        }

        pub fn source(&self) -> &dyn std::error::Error {
            & *self.source
        }

        pub fn backtrace(&self) -> Backtrace {
            self.backtrace.borrow().clone()
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.backtrace.borrow_mut().resolve();
            write!(f, "Error: {}\n\nStack Backtrace\n{:?}", self.source, self.backtrace)
        }
    }

    impl std::error::Error for Error {}

    pub type Result<T> = std::result::Result<T, Error>;

    impl<E: std::error::Error + 'static> From<super::Error<E>> for Error {
        fn from(value: super::Error<E>) -> Self {
            Error { source: Box::new(value.source), backtrace: value.backtrace }
        }
    }
}