use crate::object::{load_object, GitObject};
use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Cursor, Seek};

pub fn ls_tree_name_only(blob_sha: &str) -> Result<()> {
    if blob_sha.len() != 40 {
        return Err(anyhow!(
            "object id should be 40 characters but was {}",
            blob_sha.len()
        ));
    }

    let GitObject {
        object_type,
        content,
    } = load_object(blob_sha)?;

    if object_type != "tree" {
        return Err(anyhow!(
            "Expected object type 'tree' but was {}",
            object_type
        ));
    }

    print_names(content)?;

    Ok(())
}

fn print_names(content: Vec<u8>) -> Result<()> {
    let content_size = content.len();
    let mut reader = BufReader::new(Cursor::new(content));
    while reader.stream_position()? < content_size as u64 {
        let mut buffer = Vec::new();

        reader.read_until(' ' as u8, &mut buffer)?;
        buffer.clear();

        reader.read_until(0, &mut buffer)?;
        buffer.pop();

        let file_name = String::from_utf8(buffer)?;

        reader.seek_relative(20)?;
        println!("{file_name}");
    }

    Ok(())
}
