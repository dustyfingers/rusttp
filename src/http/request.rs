use super::method::Method;
use std::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    // query string can be 'null' - except rust has no null value
    // instead we have the Option type which can either contain a 'None' value
    // or a Some() value which wraps a value of the specified type, which here is a String
    query_string: Option<String>,
    // method can only be a certain set of strings
    method: Method
}

impl Request {

}

// handle type conversion from buffer byte slice to request
// THIS is the ways rust type conversions are meant to be implmented
// this also gives us the reverse .try_into() implementation out of the box!
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // this macro can be added into unimplemented functions to supress compiler errors
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "nvalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}


impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}