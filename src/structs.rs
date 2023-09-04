use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub(crate) last_modified: DateTime<Utc>,
    pub(crate) data: Vec<u8>,
    pub(crate) etag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    key: String,
    last_modified: DateTime<Utc>,
    size: usize,
    etag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List(pub Vec<ListItem>);

pub type S3Result<A> = Result<A, S3Error>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum S3Error {
    #[error("key not found")]
    NotFound,
}
