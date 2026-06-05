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
    #[error(transparent)]
    VulkanLoadingError(#[from] vulkano::LoadingError),
    #[error(transparent)]
    ValidatedVulkanError(#[from] vulkano::Validated<vulkano::VulkanError>),
    #[error(transparent)]
    VulkanError(#[from] vulkano::VulkanError),
    #[error(transparent)]
    EventLoopError(#[from] winit::error::EventLoopError),
    #[error(transparent)]
    FromWindowError(#[from] vulkano::swapchain::FromWindowError),
    #[error(transparent)]
    HandleError(#[from] winit::raw_window_handle::HandleError),
    #[error(transparent)]
    AllocatedBufferError(#[from] vulkano::Validated<vulkano::buffer::AllocateBufferError>),
    #[error(transparent)]
    AllocateImageError(#[from] vulkano::Validated<vulkano::image::AllocateImageError>),
    #[error(transparent)]
    HostAccessError(#[from] vulkano::sync::HostAccessError),
    #[error(transparent)]
    VulkanoValidationError(#[from] Box<vulkano::ValidationError>),
    #[error(transparent)]
    CommandBufferExecError(#[from] vulkano::command_buffer::CommandBufferExecError)
}

#[derive(Error, Debug)]
pub enum BasicError {
    #[error("Uninitialized")]
    Uninitialized,
    #[error("Out of bounds")]
    OutOfBounds
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
    BasicError {
        source: BasicError,
        backtrace: BT
    },
    #[error("{source}")]
    IoError {
        source: std::io::Error,
        backtrace: BT
    },
    #[error("Option contained None value.")]
    NoneError {
        backtrace: BT
    },
    #[error("{msg}")]
    StringError {
        msg: String,
        backtrace: BT
    },
    #[error("{msg}")]
    StaticStringError {
        msg: &'static str,
        backtrace: BT
    }
}

pub fn none_error() -> Error {
    Error::NoneError { backtrace: BT::new() }
}

impl From<ObjectError> for Error {
    fn from(value: ObjectError) -> Self {
        Error::ObjectError { source: value, backtrace: BT::new() }
    }
}

impl<T> From<T> for Error
where 
    GraphicsError: From<T> {
    fn from(value: T) -> Self {
        let value = GraphicsError::from(value);
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

impl From<BasicError> for Error {
    fn from(value: BasicError) -> Self {
        Error::BasicError { source: value, backtrace: BT::new() }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::StringError { msg: value, backtrace: BT::new() }
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::StaticStringError { msg: value, backtrace: BT::new() }
    }
}

impl Error {
    pub fn backtrace(&self) -> &impl std::fmt::Debug {
        match &self {
            Error::ObjectError { backtrace, .. } => backtrace,
            Error::GraphicsError { backtrace, .. } => backtrace,
            Error::StringError { backtrace, .. } => backtrace,
            Error::ImageError { backtrace, .. } => backtrace,
            Error::IoError { backtrace, .. } => backtrace,
            Error::StaticStringError { backtrace, .. } => backtrace,
            Error::NoneError { backtrace } => backtrace,
            Error::BasicError { backtrace, .. } => backtrace
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;