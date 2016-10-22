extern crate hyper;

use hyper::Client;
use std::env;
use std::io::Read;

fn main() {
    let client = Client::new();

    let url = &env::args().nth(1).expect("No url supplied");  // in rust 1.1 env::args() returns an Iterator

    let mut res = client.get(url)
                    .send()
                    .expect("Request failed");

    let mut body = String::new(); // read_to_string needs &str to store res in

    res.read_to_string(&mut body).expect("Read failed");

    println!("{:?}", body);

}
