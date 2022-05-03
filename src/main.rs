use serde::Deserialize;
use reqwest::{Error};
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
}

#[tokio::main]
async fn get_repositories() -> Result<Vec<Repository>, Error> {
    let client = reqwest::Client::new();

    let request_url = format!("https://api.github.com/users/{user}/repos",
                              user = "anteroserrano99");

    let response =client
        .get(request_url)
        .header(USER_AGENT, "Mirepoix")
        .send()
        .await?;


    let repositories: Vec<Repository> = response.json().await?;

    Ok(repositories)
}

//TODO add error handling
fn main(){

    let repositories = get_repositories().unwrap();

    for repository in repositories.iter() {
        println!("The name of the repository is: {}",repository.name);
        println!("The repository url is: {}",repository.html_url);
    }


}