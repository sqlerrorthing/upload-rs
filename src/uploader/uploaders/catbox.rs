use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use reqwest::{multipart, Client};
use crate::uploader::uploader::{UploadError, Uploader};
use crate::uploader::UploaderConfig;

pub struct Catbox {
    config: UploaderConfig
}

impl Catbox {
    fn new() -> Self {
        Self {
            config: UploaderConfig::new (
                "catbox",
                &(200 * 1024 * 1024),
                &["exe", "scr", "cpl", "doc", "docx", "jar"]
            )
        }
    }

    pub fn instance() -> &'static Catbox {
        static INSTANCE: Lazy<Catbox> = Lazy::new(|| Catbox::new());
        &INSTANCE
    }
}

impl Uploader for Catbox {
    fn get_config(&self) -> &UploaderConfig {
        &self.config
    }

    async fn do_upload_internal(&self, _: &Path, file_name: String, file_content: Vec<u8>, mime_type: String) -> Result<String, UploadError> {
        let form = multipart::Form::new()
            .text("reqtype", "fileupload")
            .text("userhash", "")
            .part("fileToUpload", multipart::Part::bytes(file_content)
                .file_name(file_name)
                .mime_str(&*mime_type)?);

        let client = Client::new();
        let response = client.post("https://catbox.moe/user/api.php")
            .multipart(form)
            .send()
            .await?;

        let response_text = response.text().await?;

        Ok(response_text)
    }
}