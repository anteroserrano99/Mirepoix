use serde::Deserialize;
use reqwest::{StatusCode,Error, header, Client};
use reqwest::header::USER_AGENT;
use serde_json;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let request_url = format!("https://api.github.com/users/{user}/repos",
                              user = "anteroserrano99");
    println!("{}", request_url);

    let response =client
        .get(request_url)
        .header(USER_AGENT, "Mirepoix")
        .send()
        .await?;


    let repositories: Vec<Repository> = response.json().await?;
    println!("{:?}", repositories);

    Ok(())
}