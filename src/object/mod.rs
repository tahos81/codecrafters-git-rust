mod load;
mod store;

pub use load::load_object;
pub use store::store_object;

pub struct GitObject {
    pub object_type: String,
    pub content: Vec<u8>,
}

impl GitObject {
    pub fn new(object_type: String, content: Vec<u8>) -> Self {
        Self {
            object_type,
            content,
        }
    }
}
