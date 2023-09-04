use std::path::PathBuf;

use async_trait::async_trait;
use structs::{File, List, S3Result};

pub mod structs;

#[async_trait]
pub trait S3 {
    async fn list(&self) -> S3Result<List>;
    async fn get(&self, key: PathBuf) -> S3Result<File>;
    async fn put(&self, key: PathBuf, value: File) -> S3Result<()>;
    async fn delete(&self, key: PathBuf) -> S3Result<()>;
}
