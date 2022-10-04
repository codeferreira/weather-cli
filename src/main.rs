use reqwest;
use std;

fn main() {
    let api_token =
        std::env::var("API_TOKEN").expect("expected there to be an API_TOKEN environment variable");

    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    let client = reqwest::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");

    dbg!(response);
}
