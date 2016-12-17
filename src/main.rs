#![feature(plugin)]
#![plugin(clippy)]

extern crate calamine;
extern crate hyper;

use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str;

use hyper::client::Client;

mod excel;

fn main() {
    fs::create_dir_all("data").unwrap();
    let csv = Path::new("data/foo.csv");
    let url = "https://www.spdrs.com/site-content/xls/SPY_All_Holdings.\
          xls?fund=SPY&docname=All+Holdings&onyx_code1=1286&onyx_code2=1700";
    let worksheet = "SPY_All_Holdings";
    let xls = Path::new("data/foo.xls");
    download(url, xls);
    excel::excel_to_csv(csv, xls, worksheet);
}

fn download(url: &str, xls: &Path) {
    let mut response = Client::new().get(url).send().unwrap();
    let mut buf = Vec::new();
    response.read_to_end(&mut buf).unwrap();
    let mut file = File::create(&xls).unwrap();
    file.write_all(&buf).unwrap();
}
