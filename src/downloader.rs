use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use hyper::Client;
use hyper::header::Connection;
use hyper::status::StatusCode;

fn build_filename(directory: &String, version_str: &String) -> String {
    let filename = format!("v{}.tar.gz", version_str.clone());
    Path::new(directory).join(filename).to_str()
        .unwrap()
        .into()
}

pub fn download_source(version: String, destination_path: &String) -> Option<String> {
    let client = Client::new();

    let url = format!("https://nodejs.org/dist/v{version}/node-v{version}-darwin-x64.tar.gz", version=version);
    let mut res = client.get(&*url)
        .header(Connection::close())
        .send().unwrap();

    println!("Headers:\n{}", res.headers);
    println!("Status Code:{}", res.status);

    if res.status == StatusCode::NotFound {
        return None
    }

    let filename = build_filename(&destination_path, &version);
    let mut file_handle = match File::create(filename.clone()) {
        Ok(handle)  => handle,
        Err(err)    => return None
    };

    let mut archive = Vec::new();
    res.read_to_end(&mut archive);

    match file_handle.write_all(&archive[..]) {
        Ok(_)       => Some(filename),
        Err(err)    => None
    }
}
