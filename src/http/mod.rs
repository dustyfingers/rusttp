// these modules being exposed here publicly allows the direct import from the http module in main.rs
// use http::Request; etc
pub use request::Request;
pub use method::Method;
pub use request::ParseError;

pub mod request;
pub mod method;