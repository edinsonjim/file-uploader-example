use serde::Deserialize;

#[derive(Debug, Default, Clone)]
pub struct FailureReply {
    pub message: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct BucketDetail {
    pub bucket_id: String,
}
