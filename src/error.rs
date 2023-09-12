use std::{error::Error, fmt};

use libp2p::{gossipsub::SubscriptionError, swarm::DialError, TransportError};

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

impl From<TransportError<std::io::Error>> for ClCatError {
    fn from(value: TransportError<std::io::Error>) -> Self {
        todo!()
    }
}

impl From<libp2p::multiaddr::Error> for ClCatError {
    fn from(value: libp2p::multiaddr::Error) -> Self {
        todo!()
    }
}

impl From<SubscriptionError> for ClCatError {
    fn from(value: SubscriptionError) -> Self {
        todo!()
    }
}

impl From<std::io::Error> for ClCatError {
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}

impl From<&std::io::Error> for ClCatError {
    fn from(value: &std::io::Error) -> Self {
        todo!()
    }
}

impl From<&str> for ClCatError {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<libp2p::noise::Error> for ClCatError {
    fn from(value: libp2p::noise::Error) -> Self {
        todo!()
    }
}

impl From<libp2p::swarm::DialError> for ClCatError {
    fn from(value: libp2p::swarm::DialError) -> Self {
        todo!()
    }
}
