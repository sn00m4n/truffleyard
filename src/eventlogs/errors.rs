use std::fmt::{Display, Formatter};
use std::io;

use evtx::err::EvtxError;

pub enum Error {
    EvtxError(EvtxError),
    IOError(io::Error),
    SerdeError(serde_xml_rs::Error),
    JsonError(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EvtxError(err) => {
                write!(f, "{err}")
            }
            Error::IOError(err) => {
                write!(f, "{err}")
            }
            Error::SerdeError(err) => {
                write!(f, "{err}")
            }
            Error::JsonError(err) => write!(f, "Json error: {err}"),
        }
    }
}

impl From<EvtxError> for Error {
    fn from(err: EvtxError) -> Self {
        Error::EvtxError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::SerdeError(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
