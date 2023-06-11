use crate::object::{store_object, GitObject};
use anyhow::Result;
use std::time;

pub fn commit_tree(tree_sha: &str, parent_sha: &str, message: &str) -> Result<String> {
    let mut content: Vec<u8> = Vec::new();

    content.extend_from_slice("tree ".as_bytes());
    content.extend_from_slice(tree_sha.as_bytes());
    content.extend_from_slice("\n".as_bytes());

    content.extend_from_slice("parent ".as_bytes());
    content.extend_from_slice(parent_sha.as_bytes());
    content.extend_from_slice("\n".as_bytes());

    let now = format!("{:?}", time::SystemTime::now());

    content.extend_from_slice("author ".as_bytes());
    content.extend_from_slice("Scott Chacon <schacon@gmail.com> ".as_bytes());
    content.extend_from_slice(now.as_bytes());
    content.extend_from_slice(" -0700".as_bytes());
    content.extend_from_slice("\n".as_bytes());

    content.extend_from_slice("committer ".as_bytes());
    content.extend_from_slice("Scott Chacon <schacon@gmail.com> ".as_bytes());
    content.extend_from_slice(now.as_bytes());
    content.extend_from_slice(" -0700".as_bytes());
    content.extend_from_slice("\n".as_bytes());

    content.extend_from_slice("\n".as_bytes());
    content.extend_from_slice(message.as_bytes());
    content.extend_from_slice("\n".as_bytes());

    store_object(GitObject {
        object_type: "commit".to_string(),
        content,
    })
}
