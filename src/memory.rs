use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};

use crate::{
    helpers::insert_parents,
    structs::{File, List, ListItem, S3Error, S3Result},
    S3,
};
use async_trait::async_trait;

pub struct S3Memory {
    files: Mutex<BTreeMap<PathBuf, File>>,
}

impl S3Memory {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(BTreeMap::new()),
        }
    }
}

#[async_trait]
impl S3 for S3Memory {
    async fn list(&self) -> S3Result<List> {
        let mut list = BTreeMap::new();
        let files = self.files.lock().unwrap();
        for (k, v) in files.iter() {
            let item = ListItem {
                key: k.to_owned(),
                last_modified: v.last_modified,
                size: v.data.len(),
                etag: v.etag.clone(),
            };
            insert_parents(&mut list, &item);
            list.insert(k.clone(), item);
        }
        Ok(List(list.into_values().collect()))
    }

    async fn get(&self, key: PathBuf) -> S3Result<File> {
        if let Some(v) = self.files.lock().unwrap().get(&key) {
            Ok(v.clone())
        } else {
            Err(S3Error::NotFound)
        }
    }

    async fn put(&self, key: PathBuf, value: File) -> S3Result<()> {
        self.files.lock().unwrap().insert(key, value);
        Ok(())
    }

    async fn delete(&self, key: PathBuf) -> S3Result<()> {
        if self.files.lock().unwrap().remove(&key).is_some() {
            Ok(())
        } else {
            Err(S3Error::NotFound)
        }
    }
}

#[tokio::test]
async fn test_put() {
    let s3 = S3Memory::new();
    let foo = File {
        last_modified: chrono::Utc::now(),
        etag: "foo".to_owned(),
        data: b"foo".to_vec(),
    };
    let bar = File {
        last_modified: chrono::Utc::now(),
        etag: "bar".to_owned(),
        data: b"bar".to_vec(),
    };
    s3.put(PathBuf::from("/foo"), foo.clone()).await.unwrap();
    s3.put(PathBuf::from("/bar"), bar.clone()).await.unwrap();
    let files = s3.files.lock().unwrap().clone();
    assert_eq!(
        files,
        maplit::btreemap! {PathBuf::from("/foo") => foo,PathBuf::from("/bar") => bar}
    )
}

#[tokio::test]
async fn test_get() {
    let s3 = S3Memory::new();
    let foo = File {
        last_modified: chrono::Utc::now(),
        etag: "foo".to_owned(),
        data: b"foo".to_vec(),
    };
    let bar = File {
        last_modified: chrono::Utc::now(),
        etag: "bar".to_owned(),
        data: b"bar".to_vec(),
    };
    *s3.files.lock().unwrap() = maplit::btreemap! {PathBuf::from("/foo") => foo.clone(),PathBuf::from("/bar") => bar.clone()};
    assert_eq!(s3.get(PathBuf::from("/foo")).await, Ok(foo));
    assert_eq!(s3.get(PathBuf::from("/bar")).await, Ok(bar));
}

#[tokio::test]
async fn test_delete() {
    let s3 = S3Memory::new();
    let foo = File {
        last_modified: chrono::Utc::now(),
        etag: "foo".to_owned(),
        data: b"foo".to_vec(),
    };
    let bar = File {
        last_modified: chrono::Utc::now(),
        etag: "bar".to_owned(),
        data: b"bar".to_vec(),
    };
    *s3.files.lock().unwrap() = maplit::btreemap! {PathBuf::from("/foo") => foo.clone(),PathBuf::from("/bar") => bar.clone()};
    s3.delete(PathBuf::from("/bar")).await.unwrap();
    assert_eq!(s3.get(PathBuf::from("/foo")).await, Ok(foo));
    assert_eq!(s3.get(PathBuf::from("/bar")).await, Err(S3Error::NotFound));
}

#[tokio::test]
async fn test_list() {
    let s3 = S3Memory::new();
    let foo = File {
        last_modified: chrono::Utc::now(),
        etag: "foo".to_owned(),
        data: b"foo".to_vec(),
    };
    let bar = File {
        last_modified: chrono::Utc::now(),
        etag: "bar".to_owned(),
        data: b"bar".to_vec(),
    };
    let baz = File {
        last_modified: chrono::Utc::now(),
        etag: "baz".to_owned(),
        data: b"baz".to_vec(),
    };
    *s3.files.lock().unwrap() = maplit::btreemap! {PathBuf::from("/foo") => foo.clone(),PathBuf::from("/bar") => bar.clone(),PathBuf::from("/dir/baz") => baz.clone()};
    assert_eq!(
        s3.list().await,
        Ok(List(vec![
            ListItem {
                key: PathBuf::from("/bar"),
                last_modified: bar.last_modified,
                size: 3,
                etag: bar.etag
            },
            ListItem {
                key: PathBuf::from("/dir/"),
                last_modified: baz.last_modified,
                size: 0,
                etag: String::new()
            },
            ListItem {
                key: PathBuf::from("/dir/baz"),
                last_modified: baz.last_modified,
                size: 3,
                etag: baz.etag
            },
            ListItem {
                key: PathBuf::from("/foo"),
                last_modified: foo.last_modified,
                size: 3,
                etag: foo.etag
            },
        ]))
    );
}
