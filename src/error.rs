use std::{error::Error, fmt};

pub const ERROR_CLIENT_INIT: u8 = 1u8;

#[derive(Debug)]
pub enum InnerClCatError {}

impl fmt::Display for InnerClCatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl Error for InnerClCatError {}

#[derive(Debug)]
pub struct ClCatError {
    code: u8,
    msg: String,
    inner: Option<InnerClCatError>,
}

impl ClCatError {
    pub fn new(code: u8, msg: String, inner: Option<InnerClCatError>) -> Self {
        Self { code, msg, inner }
    }
}

impl fmt::Display for ClCatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.inner {
            Some(e) => write!(f, "E{}: {} due to {}", self.code, self.msg, e),
            None => write!(f, "E{}: {}", self.code, self.msg),
        }
    }
}

impl Error for ClCatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.as_ref().map(|x| x as &dyn Error)
    }
}
