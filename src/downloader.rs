use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use hyper::Client;
use hyper::client::response::Response;
use hyper::header::Connection;
use hyper::status::StatusCode;
use os_type;
use language::{Language, Url};

pub struct Downloader {
    urls: Vec<Url>
}


impl Downloader {

    pub fn new(lang: Language) -> Downloader {

        Downloader { urls: lang.download_urls }
    }

    fn build_filename(&self, directory: &String, version_str: &String) -> String {
        let filename = format!("v{}.tar.gz", version_str.clone());
        Path::new(directory).join(filename).to_str()
            .unwrap()
            .into()
    }

    fn write_to_file(&self, destination_path: &String, version: &String, response: &mut Response) -> Result<String, String> {
        let filename = self.build_filename(&destination_path, &version);
        let mut file_handle = match File::create(filename.clone()) {
            Ok(handle)  => handle,
            Err(_)    => return Err("Failed to create file".to_string())
        };
        let mut archive = Vec::<u8>::new();
        for byte in response.bytes() {
            archive.push(byte.unwrap());
        }
        match file_handle.write_all(&archive[..]) {
            Ok(_)       => {
                Ok(filename)
            },
            Err(_)    => Err("Failed to write file".to_string())
        }
    }

    fn url_for_os_type(&self, version: &String) -> Option<String> {
        let platform = os_type::current_platform();

        match self.urls.iter().find(|&u| u.platform == platform) {
            Some(url) => {
                Some(url.url.replace("{VERSION}", version))
            },
            None => None
        }
    }

    pub fn download_source(&self, version: &String, destination_path: &String) -> Result<String, String> {
        let client = Client::new();
        let url = self.url_for_os_type(&version);
        if url == None {
            return Err(format!("Unsupported OS Type {:?}", os_type::current_platform()));
        }
        let mut response = client.get(&*url.unwrap())
            .header(Connection::close())
            .send().unwrap();

        if response.status == StatusCode::Ok {
            self.write_to_file(destination_path, &version, &mut response)
        } else {
            return Err(format!("Download failed with HTTP Status {}", response.status));
        }
    }
}
