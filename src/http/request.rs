use super::method::Method;
pub struct Request {
    path: String,
    // query string can be 'null' - except rust has no null value
    // instead we have the Option type which can either contain a 'None' value
    // or a Some() value which wraps a value of the specified type, which here is a String
    query_string: Option<String>,
    // method can only be a certain set of strings
    method: Method
}