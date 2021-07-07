pub enum Error {
    IO(std::io::Error),
    Unknown,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}