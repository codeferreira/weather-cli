use std;

fn main() {
    let api_token =
        std::env::var("API_TOKEN").expect("expected there to be an API_TOKEN environment variable");
    dbg!(api_token);
}
