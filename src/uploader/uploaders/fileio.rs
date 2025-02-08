use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use reqwest::Client;
use crate::uploader::{Catbox, UploadError, Uploader, UploaderConfig};

pub struct FileIo {
    config: UploaderConfig
}

impl FileIo {
    pub fn new() -> Self {
        Self {
            config: UploaderConfig::new(
                "FileIo",
                &(4 * 1024 * 1024 * 1024),
                &[]
            )
        }
    }

    pub fn instance() -> &'static FileIo {
        static INSTANCE: Lazy<FileIo> = Lazy::new(|| FileIo::new());
        &INSTANCE
    }
}

impl Uploader for FileIo {
    fn get_config(&self) -> &UploaderConfig {
        &self.config
    }

    async fn do_upload_internal(&self, path: &Path, file_name: String, file_content: Vec<u8>, mime_type: String) -> Result<String, UploadError> {
        let client = Client::new();
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(file_content)
                .file_name(file_name)
                .mime_str(mime_type.as_str())?);

        let response = client
            .post("https://file.io")
            .multipart(form)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        Ok(json["link"].as_str().unwrap().into())
    }
}