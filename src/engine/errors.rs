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
    WorldMismatchError { other: &'static str }
}

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("Winow already created!")]
    WindowCreatedError,
    #[error("Shader compile error - {src:?}\n{error_message:?}")]
    ShaderCompileError{ src: String, error_message: String },
    #[error("Graphics not initialized!")]
    GraphicsNotInitializedError,
    #[error("Failed to create window!")]
    WindowCreationFailError
}