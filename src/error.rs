#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("winit failed to create a window because of an os error")]
    WinitOsError,

    #[error("the swap chain has been lost and needs to be recreated")]
    WgpuSurfaceLost,
    #[error("timed out while trying to aquire the next frame")]
    WgpuSurfaceTimeout,
    #[error("the swap chain must be updated")]
    WgpuSurfaceOutdated,
    #[error("could not create a surface")]
    WgpuSurfaceCreateError,
    #[error("could not create a device")]
    WgpuRequestDeviceError,
    #[error("could not create adapter")]
    WgpuAdapterCreationFailed,
    
    #[error("the program has run out of memory")]
    OutOfMemory,
    #[error("unknown error: {0}")]
    UnknownError(String),
}
