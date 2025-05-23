use super::method::Method;

pub struct Request{
    path: String,
    query_string: Option<String>, // It is optional because query string can also be null.
    method: Method,
}