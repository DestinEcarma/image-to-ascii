#[derive(Debug, derive_more::From)]
pub enum Error {
    /// An error occurred while reading or writing an image.
    ImageError(image::ImageError),
    /// An error occurred while parsing a UTF-8 string.
    Utf8Error(std::str::Utf8Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ImageError(e) => e.fmt(f),
            Error::Utf8Error(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ImageError(e) => e.source(),
            Error::Utf8Error(e) => e.source(),
        }
    }
}
