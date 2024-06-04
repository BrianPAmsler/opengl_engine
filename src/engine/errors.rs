use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error("Component is dead!")]
    DeadComponentError,
    #[error("Root object cannot be deleted!")]
    RootObjectDeleteError,
    #[error("Component does not belong to object!")]
    ComponentMismatchError,
    #[error("Object is dead!")]
    DeadObjectError,
    #[error("{other} must belong to the same world!")]
    WorldMismatchError { other: &'static str },
    #[error("Component is not of type {type_name}")]
    ComponentDowncastError { type_name: String },
    #[error("Component not found!")]
    ComponentNotFoundError
}

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("Winow already created!")]
    WindowCreatedError,
    #[error("Shader compile error - {src}\n{error_message}")]
    ShaderCompileError{ src: String, error_message: String },
    #[error("Graphics not initialized!")]
    GraphicsNotInitializedError,
    #[error("Failed to create window!")]
    WindowCreationFailError
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ObjectError(#[from] ObjectError),
    #[error(transparent)]
    GraphicsError(#[from] GraphicsError)
}

pub type Result<T> = core::result::Result<T, Error>;