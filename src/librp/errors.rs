use std::fmt;
use std::fmt::Display;
use std::result::Result as StdResult;
use std::io;

pub type Result<T> = StdResult<T, PrefetchError>;

#[derive(Debug)]
pub enum ErrorKind {
    InvalidFileSignature,
    IoError,
}
/// USN Record Parsing Error
#[derive(Debug)]
pub struct PrefetchError {
    /// Formated error message
    pub message: String,
    /// The type of error
    pub kind: ErrorKind
}

impl PrefetchError{
    #[allow(dead_code)]
    pub fn invalid_file_signature(err: String)->Self{
        PrefetchError {
            message: format!("{}",err),
            kind: ErrorKind::InvalidFileSignature
        }
    }
    #[allow(dead_code)]
    pub fn io_error(err: String)->Self{
        PrefetchError {
            message: format!("{}",err),
            kind: ErrorKind::IoError
        }
    }
}

impl From<io::Error> for PrefetchError {
    fn from(err: io::Error) -> Self {
        PrefetchError {
            message: format!("{}",err),
            kind: ErrorKind::IoError
        }
    }
}

impl Display for PrefetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { writeln!(f, "{:?}: {}",self.kind,self.message) }
}
