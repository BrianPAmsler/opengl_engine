use image::ImageError;
use thiserror::Error;

type BT = backtrace::Backtrace;

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
    WindowCreationFailError,
    #[error("{msg}")]
    GLLoadError{msg: &'static str},
    #[error(transparent)]
    GLInitError(#[from] glfw::InitError)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("{source}")]
    ObjectError {
        source: ObjectError,
        backtrace: BT
    },
    #[error("{source}")]
    GraphicsError {
        source: GraphicsError,
        backtrace: BT
    },
    #[error("{source}")]
    ImageError {
        source: image::ImageError,
        backtrace: BT
    },
    #[error("{source}")]
    IoError {
        source: std::io::Error,
        backtrace: BT
    },
    #[error("{msg}")]
    StringError {
        msg: String,
        backtrace: BT
    }
}

impl From<ObjectError> for Error {
    fn from(value: ObjectError) -> Self {
        Error::ObjectError { source: value, backtrace: BT::new() }
    }
}

impl From<GraphicsError> for Error {
    fn from(value: GraphicsError) -> Self {
        Error::GraphicsError { source: value, backtrace: BT::new() }
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Error::ImageError { source: value, backtrace: BT::new() }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError { source: value, backtrace: BT::new() }
    }
}

impl From<glfw::InitError> for Error {
    fn from(value: glfw::InitError) -> Self {
        GraphicsError::from(value).into()
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::StringError { msg: value, backtrace: BT::new() }
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::StringError { msg: value.to_owned(), backtrace: BT::new() }
    }
}

impl Error {
    pub fn backtrace(&self) -> &impl std::fmt::Debug {
        match &self {
            Error::ObjectError { backtrace, .. } => backtrace,
            Error::GraphicsError { backtrace, .. } => backtrace,
            Error::StringError { backtrace, .. } => backtrace,
            Error::ImageError { backtrace, .. } => backtrace,
            Error::IoError { backtrace, .. } => backtrace
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;