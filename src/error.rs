use std::{error::Error as StdError, fmt};
use derive_more::{Display, From};
use phenopackets::schema::v1::GenomicInterpretation;
use serde::{Deserialize, Serialize};


pub type Result<T> = core::result::Result<T, Error>;



#[derive(Debug, From, Serialize, Deserialize)]
pub enum Error {
    #[from]
    Custom(String),
    CurieError {
        msg: String,
    },
    GenomicInterpretationError { msg: String},
    IndividualError {
        msg: String,
    },
    TimeElementError {
        msg: String
    }

}



impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> fmt::Result {
        match self {
            Error::CurieError{msg} 
            | Error::GenomicInterpretationError{ msg}
            | Error::TimeElementError{msg} => {
                write!(fmt, "{msg}" )
            },
            _ => write!(fmt, "{self:?}"),
        }
    }
}


impl StdError for Error {}
