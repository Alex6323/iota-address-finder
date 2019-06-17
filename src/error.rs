#[derive(Debug)]
pub(crate) enum Error {
    App(&'static str),
    Io(std::io::Error),
    Std(Box<std::error::Error>),
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Self {
        Error::App(message)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<Box<std::error::Error>> for Error {
    fn from(e: Box<std::error::Error>) -> Self {
        Error::Std(e)
    }
}