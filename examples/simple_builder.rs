extern crate resp;
use resp::*;
fn main() {
    let js = include_str!("simple.js");
    let mut builder = Builder::new();
    let mut p = builder
                .module(false)
                .tolerant(false)
                .comments(false)
                .js(js)
                .build().unwrap();
    let script = p.parse().unwrap();
    println!("{:#?}", script);
}