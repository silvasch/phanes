#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("winit failed to create a window because of an os error")]
    WinitOsError,

    #[error("the swap chain has been lost and needs to be recreated")]
    SurfaceLost,
    #[error("timed out while trying to aquire the next frame")]
    SurfaceTimeout,
    #[error("the swap chain must be updated")]
    SurfaceOutdated,
    #[error("the program has run out of memory")]
    OutOfMemory,
    #[error("could not create a surface")]
    SurfaceCreateError,
    #[error("could not create a device")]
    RequestDeviceError,

    #[error("could not create adapter")]
    AdapterCreationFailed,
    #[error("unknown error: {0}")]
    UnknownError(String),
}
