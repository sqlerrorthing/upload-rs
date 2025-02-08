use crate::uploader::{Catbox, UploadError, Uploader};

mod uploader;

#[tokio::main]
async fn main() {
    let file_path = std::path::Path::new("txt.txt");
    let catbox = Catbox::instance().lock().unwrap();
    let result = catbox.upload(file_path);

    match result.await {
        Ok(link) => {println!("{:?}", link);}
        Err(error) => {
            match error {
                UploadError::MaxFilesizeLimitsReached => println!("MaxFilesizeLimitsReached"),
                UploadError::NoFilename => println!("NoFilename"),
                UploadError::NotAllowedExtension => println!("NotAllowedExtension"),
                UploadError::IOError(e) => println!("IOError, {e}"),
                UploadError::ReqwestError(e) => println!("ReqwestError, {e}"),
            }
        }
    }

}
