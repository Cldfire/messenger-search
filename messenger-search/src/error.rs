use std::io;
use tantivy::query::QueryParserError;

#[derive(Debug)]
pub enum Error {
    SerdeJsonError(::serde_json::Error),
    IoError(io::Error),
    TantivyError(::tantivy::Error),
    TantivyQueryParserError(QueryParserError)
}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Self {
        Error::SerdeJsonError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<::tantivy::Error> for Error {
    fn from(err: ::tantivy::Error) -> Self {
        Error::TantivyError(err)
    }
}

impl From<QueryParserError> for Error {
    fn from(err: QueryParserError) -> Self {
        Error::TantivyQueryParserError(err)
    }
}
