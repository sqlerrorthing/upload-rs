use std::path::Path;
use enum_dispatch::enum_dispatch;
use crate::uploader::{Catbox, FileIo, UploadError, Uploader, UploaderConfig};

mod uploader;

#[derive(Clone)]
#[enum_dispatch(Uploader)]
enum UploaderEnum {
    FileIo(&'static FileIo),
    Catbox(&'static Catbox)
}

impl UploaderEnum {
    pub(crate) async fn upload(&self, file: &Path) -> Result<String, UploadError> {
        match self {
            UploaderEnum::FileIo(uploader) => uploader.upload(file).await,
            UploaderEnum::Catbox(uploader) => uploader.upload(file).await,
        }
    }
}

fn instances() -> Vec<UploaderEnum> {
    vec![
        UploaderEnum::Catbox(Catbox::instance()),
        UploaderEnum::FileIo(FileIo::instance()),
    ]
}

fn get_uploader_by_name(name: &str) -> Result<UploaderEnum, Vec<&'static str>> {
    let instances = instances();

    let res = instances.iter()
        .find(|curr| name.eq_ignore_ascii_case(curr.get_config().name))
        .cloned();

    res.ok_or(instances.iter()
        .map(|el| el.get_config().name)
        .collect()
    )
}

#[tokio::main]
async fn main() {
    let file_path = Path::new("txt.txt");
    
    let uploader = match get_uploader_by_name("fileio") {
        Ok(uploader) => uploader,
        Err(err) => panic!("{:?}", err),
    };

    let result = uploader.upload(file_path);

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
