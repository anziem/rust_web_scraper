extern crate hyper;
extern crate select;

use hyper::Client;
use std::env;
use std::io::Read;
use select::document::Document;
use select::predicate::{Class, Attr, Name};

fn main() {
    let client = Client::new();

    let url = &env::args().nth(1).expect("No url supplied");  // in rust 1.1 env::args() returns an Iterator

    let mut res = client.get(url)
                    .send()
                    .expect("Request failed");

    let mut body = String::new(); // read_to_string needs &str to store res in

    res.read_to_string(&mut body).expect("Read failed");
    // println!("{:?}", body);

    let document = Document::from(body.as_str()); // put &str of body in Document

    // here's where you should customize - create a function for the particular info you want to scrape
    print_amazon_wishlist_items(&document);

}

fn print_amazon_wishlist_items (doc : &Document) -> () {
    // only works for public wishlist
    let wrapper = doc.find(Class("g-items-section"));
    let rows = wrapper.find(Class("a-fixed-right-grid"));
    for row in rows.iter() {
        let title_node = row.find(Name("h5")).first();
        let price_node = row.find(Class("a-color-price")).first();
        match (title_node, price_node) {
            (Some(title), Some(price)) => {
                println!("* \"{}\", with price {}",
                    title.text().trim(),
                    price.text().trim()
                );
            }
            (_, _) => (), // if don't have both title and price, don't include in list
        }
    }
}
