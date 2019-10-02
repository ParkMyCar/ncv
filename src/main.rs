use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;

use chrono::{DateTime};
use clap::{App, Arg};
use colored::*;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_derive::Deserialize;
use url::Url;

fn main() {
    // Command Line Args
    let args = App::new("ncv")
        .version("0.1")
        .author("Parker Timmerman")
        .arg(
            Arg::with_name("crate")
                .index(1)
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let crate_name = args.value_of("crate").expect("Error parsing the args!");

    // Requeset Headers
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_str("reqwests/0.9.20").unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        HeaderValue::from_str("gzip, deflate").unwrap(),
    );

    // Request URL
    let mut url_params: HashMap<&str, &str> = HashMap::new();
    url_params.insert("page", "1");
    url_params.insert("per_page", "10");
    url_params.insert("q", crate_name);

    let url = Url::parse_with_params("https://crates.io/api/v1/crates", url_params).unwrap();

    // Create the client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    // Make the request and parse the response
    let json_resp: SearchResponse = client.get(url.as_str()).send().unwrap().json().unwrap();

    match json_resp.crates.iter().find(|&c| c.exact_match == true) {
        // If there is a crate that is an exact match, just print that one
        Some(c) => {
            println!("{}", c);
        }
        // otherwise print all of them!
        None => {
            json_resp.crates.iter().for_each(|c| println!("{}\n", c));
        }
    };
}

#[derive(Clone, Debug, Deserialize)]
struct Crate {
    id: String,
    name: String,
    updated_at: String,
    max_version: String,
    exact_match: bool,
}
impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crate_name = &self.name;
        let max_version = &self.max_version;
        let crate_and_version = format!("{} ⟶ {}", crate_name, max_version.green());

        match DateTime::parse_from_rfc3339(&self.updated_at) {
            Ok(date) => {
                // Append the date in a format of DD MM YYYY
                write!(f, "{}\nLast Updated: {}", crate_and_version, date.format("%d %B %Y"))
            }
            Err(_) => {
                write!(f, "{}", crate_and_version)
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct SearchResponse {
    crates: Vec<Crate>,
}
