#![allow(unused)]
use crate::error::ABUnion;


mod error {
    use error_union::error_union;
    use thiserror::Error;

    pub trait EngineError {}

    #[derive(Error, Debug)]
    #[error("A")]
    pub struct A {}
    #[derive(Error, Debug)]
    #[error("B")]
    pub struct B {}

    error_union!(A, B);
}

fn err(value: std::result::Result<(), ABUnion>) -> Result<(), ABUnion> {
    value?;

    Ok(())
}