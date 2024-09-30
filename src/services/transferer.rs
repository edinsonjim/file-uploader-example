use web_sys::FormData;

use super::types::{BucketDetail, FailureReply};

const API_URL: &str = "./api/v1/files";

pub async fn transfer_file(files: &Vec<web_sys::File>) -> Result<BucketDetail, FailureReply> {
    let form_data = FormData::new().unwrap();
    for file in files.iter() {
        form_data
            .append_with_blob_and_filename("file", file, file.name().as_str())
            .unwrap();
    }

    match reqwasm::http::Request::post(&API_URL)
        .body(form_data)
        .send()
        .await
    {
        Ok(res) => res
            .json::<BucketDetail>()
            .await
            .map_err(|err| FailureReply {
                message: err.to_string(),
            }),
        Err(err) => Err(FailureReply {
            message: err.to_string(),
        })?,
    }
}
