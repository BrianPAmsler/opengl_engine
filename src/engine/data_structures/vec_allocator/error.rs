use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element removed!")]
    ElementRemovedError
}