use std::ffi::OsStr;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use enum_dispatch::enum_dispatch;
use crate::uploader::uploader::UploadError::{MaxFilesizeLimitsReached, NotAllowedExtension};
use reqwest::Error as RError;

pub struct UploaderConfig {
    pub name: &'static str,
    max_file_size: &'static u64,
    blocked_extensions: &'static [&'static str],
}

impl UploaderConfig {
    pub(crate) fn new(name: &'static str, max_file_size: &'static u64, blocked_extensions: &'static [&'static str]) -> Self {
        Self {
            name,
            max_file_size,
            blocked_extensions
        }
    }
}

pub enum UploadError {
    MaxFilesizeLimitsReached,
    NoFilename,
    NotAllowedExtension,
    IOError(Error),
    ReqwestError(RError),
}

impl From<Error> for UploadError {
    fn from(error: Error) -> Self {
        UploadError::IOError(error)
    }
}

impl From<RError> for UploadError {
    fn from(error: RError) -> Self {
        UploadError::ReqwestError(error)
    }
}

#[enum_dispatch]
pub trait Uploader {
    async fn upload(&self, file: &Path) -> Result<String, UploadError> {
        if file.metadata()?.len() > *self.get_config().max_file_size {
            return Err(MaxFilesizeLimitsReached)
        }

        let is_extension_forbidden = file.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.get_config().blocked_extensions.contains(&ext))
            .unwrap_or(false);

        if is_extension_forbidden {
            return Err(NotAllowedExtension)
        }

        let file_name = match file.file_name() {
            None => return Err(UploadError::NoFilename),
            Some(name) => name.to_string_lossy().into_owned(),
        };

        let mut file_content = Vec::new();
        let mut file_reader = File::open(file)?;
        file_reader.read_to_end(&mut file_content)?;

        let mime_type = mime_guess::from_path(&file)
            .first()
            .map(|mime| mime.to_string())
            .unwrap_or("application/octet-stream".into());

        self.do_upload_internal(file, file_name, file_content, mime_type).await
    }

    fn get_config(&self) -> &UploaderConfig;

    async fn do_upload_internal(
        &self, 
        path: &Path, 
        file_name: String, 
        file_content: Vec<u8>, 
        mime_type: String
    ) -> Result<String, UploadError>;
}