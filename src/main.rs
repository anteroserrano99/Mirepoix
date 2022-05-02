use serde::Deserialize;
use reqwest::{StatusCode,Error, header, Client};
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);

    let response =client
        .get(request_url)
        .header(USER_AGENT, "anteroserrano99")
        .send()
        .await?;


    match response.status() {
        StatusCode::OK => println!("Works"),
        _ => println!("Not Works")
    }

    println!("{}",response.status());


    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}