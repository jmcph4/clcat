use std::{error::Error, fmt};

use ethers::prelude::ProviderError; /* TODO: remove */

pub const ERROR_CLIENT_INIT: u8 = 1u8;

#[derive(Debug)]
pub enum InnerFoobarError {
    ClientError(ProviderError),
}

impl fmt::Display for InnerFoobarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientError(e) => write!(f, "ClientError: {}", e),
        }
    }
}

impl Error for InnerFoobarError {}

#[derive(Debug)]
pub struct FoobarError {
    code: u8,
    msg: String,
    inner: Option<InnerFoobarError>,
}

impl FoobarError {
    pub fn new(code: u8, msg: String, inner: Option<InnerFoobarError>) -> Self {
        Self { code, msg, inner }
    }
}

impl fmt::Display for FoobarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.inner {
            Some(e) => write!(f, "E{}: {} due to {}", self.code, self.msg, e),
            None => write!(f, "E{}: {}", self.code, self.msg),
        }
    }
}

impl Error for FoobarError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.as_ref().map(|x| x as &dyn Error)
    }
}

impl From<ProviderError> for FoobarError {
    fn from(value: ProviderError) -> Self {
        Self::new(
            ERROR_CLIENT_INIT,
            "Failed to initialise client".to_string(),
            Some(InnerFoobarError::ClientError(value)),
        )
    }
}
