use bytes::Bytes;
use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct FileEntity {
    pub format: String,
    pub content: Bytes,
}