extern crate html2text;
extern crate hyper;
extern crate hyper_native_tls;

use std::io;
use std::io::Read;
use std::io::BufRead;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let url = line.as_str();
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut res = client.get(url).send().unwrap();
    let mut result = String::new();
    res.read_to_string(&mut result);

    println!("{}", html2text::from_read(&mut result.as_bytes(), 80));
}
