use crate::object::{store_object, GitObject};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn hash_object_write<P: AsRef<Path>>(path: P) -> Result<String> {
    let source_file = File::open(path)?;
    let mut reader = BufReader::new(source_file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    store_object(GitObject {
        object_type: "blob".to_string(),
        content: buffer,
    })
}
