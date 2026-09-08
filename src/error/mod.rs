use thiserror::Error;

#[derive(Error, Debug)]
pub enum ViewError {
    #[error("GPU backend initialization failed: {0}")]
    GpuInit(String),
    #[error("Surface creation failed: {0}")]
    SurfaceCreation(String),
    #[error("Font loading failed: {0}")]
    FontLoad(String),
    #[error("JNI error: {0}")]
    Jni(String),
}

pub type Result<T> = std::result::Result<T, ViewError>;
