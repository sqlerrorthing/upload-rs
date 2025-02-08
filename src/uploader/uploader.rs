use std::io::Error;
use std::path::Path;
use crate::uploader::uploader::UploadError::{MaxFilesizeLimitsReached, NotAllowedExtension};

pub struct UploaderConfig {
    name: &'static str,
    max_file_size: &'static u64,
    not_allowed: &'static Vec<&'static str>,
}

impl UploaderConfig {
    pub(crate) fn new(name: &'static str, max_file_size: &'static u64, not_allowed: &'static Vec<&'static str>) -> Self {
        Self {
            name,
            max_file_size,
            not_allowed,
        }
    }
}

pub enum UploadError {
    MaxFilesizeLimitsReached,
    NotAllowedExtension,
    IOError(Error)
}

impl From<Error> for UploadError {
    fn from(error: Error) -> Self {
        UploadError::IOError(error)
    }
}

pub trait Uploader {
    fn upload(&self, file: &Path) -> Result<String, UploadError> {
        if file.metadata()?.len() > *self.get_config().max_file_size {
            return Err(MaxFilesizeLimitsReached)
        }

        let is_extension_forbidden = file.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.get_config().not_allowed.contains(&ext))
            .unwrap_or(false);

        if is_extension_forbidden {
            return Err(NotAllowedExtension)
        }

        self.do_upload_internal(file)
    }

    fn get_config(&self) -> &UploaderConfig;

    fn do_upload_internal(&self, file: &Path) -> Result<String, UploadError>;
}