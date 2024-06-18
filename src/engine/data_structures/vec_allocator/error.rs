use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element removed!")]
    ElementRemovedError,
    #[error("Invalid index! (pointer mismatch)")]
    IndexPointerMismatchError
}

pub type Result<T> = core::result::Result<T, Error>;