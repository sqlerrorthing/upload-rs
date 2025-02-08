use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;
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
                &vec!["exe", "scr", "cpl", "doc", "docx", "jar"]
            )
        }
    }

    fn instance() -> &'static Mutex<Self> {
        static INSTANCE: Lazy<Mutex<Catbox>> = Lazy::new(|| Mutex::new(Self::new()));
        &INSTANCE
    }
}

impl Uploader for Catbox {
    fn get_config(&self) -> &UploaderConfig {
        &self.config
    }

    fn do_upload_internal(&self, file: &Path) -> Result<String, UploadError> {

    }
}