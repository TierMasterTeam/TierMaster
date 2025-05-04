use bytes::Bytes;

#[derive(Clone, Debug)]
pub struct ImageEntity {
    pub content_type: String,
    pub image_name: String,
    pub data: Bytes,
}