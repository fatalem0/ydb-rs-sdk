use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    YdbStatus(ydb_protobuf::generated::ydb::status_ids::StatusCode),
}

impl Error {
    pub fn from_str(s: &str)->Error{
        return Error::Custom(s.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError)->Self{
        return Error::Custom(e.to_string())
    }
}

impl From<tonic::codegen::http::uri::InvalidUri> for Error{
    fn from(e: tonic::codegen::http::uri::InvalidUri) -> Self {
        return Error::Custom(e.to_string())
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error)->Self{
        return Error::Custom(e.to_string())
    }
}

impl From<tonic::Status> for Error {
    fn from(e: tonic::Status)->Self{
        return Error::Custom(e.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError)->Self {
        return Error::Custom(e.to_string());
    }
}

impl From<ydb_protobuf::generated::ydb::status_ids::StatusCode> for Error {
    fn from(e: ydb_protobuf::generated::ydb::status_ids::StatusCode)->Self{
        return Error::YdbStatus(e)
    }
}