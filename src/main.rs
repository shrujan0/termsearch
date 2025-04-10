use std::env;
use reqwest;
use tokio;
use std::process::exit;
use termsearch;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No query was provided\n");
        exit(1);
    }
    let query = args.iter()
        .skip(1)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    println!("{}\n\n\n\n", query);
        let client = reqwest::Client::new();
        let param = [("q", &query)];
        let res = client.post("https://lite.duckduckgo.com/lite/")
            .form(&param)
            .send()
            .await;
        let html = res.unwrap().text().await.unwrap();
        termsearch::parse(html);
}

