use serde::Deserialize;
use reqwest::{StatusCode,Error, header, Client};
use reqwest::header::USER_AGENT;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);

    let response =client
        .get(request_url)
        .header(USER_AGENT, "My rust app")
        .send()
        .await?;


    match response.status() {
        StatusCode::OK => println!("Works"),
        _ => println!("Not Works")
    }

    let json = response.text().await?;

    println!("{}", json);

    Ok(())
}