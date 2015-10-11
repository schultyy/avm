use std::io;
use hyper::Client;
use hyper::header::Connection;

pub fn download_source(version: String) -> String {
    let client = Client::new();

    let url = format!("https://nodejs.org/dist/v{version}/node-v{version}-darwin-x64.tar.gz", version=version);
    let mut res = client.get(&*url)
        .header(Connection::close())
        .send().unwrap();

    println!("Headers:\n{}", res.headers);
    // io::copy(&mut res, &mut io::stdout()).unwrap();
    String::new()
}
