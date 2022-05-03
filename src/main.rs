use std::fs;
use std::io::Write;
use std::env;
use serde::Deserialize;
use reqwest::{Error};
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
}
const directory_name:&str = "anteroserrano99";

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


    let os = env::consts::OS;
    println!("{}", env::consts::OS);


    create_directory(os);

    for repository in repositories.iter() {
        println!("The name of the repository is: {}",repository.name);
        println!("The repository url is: {}",repository.html_url);
    }


}




fn create_directory(os:&str) -> std::io::Result<()> {

    match os {
        "windows" => fs::create_dir_all(format!("{}{}","C:\\",directory_name))?,
        _ => fs::create_dir_all(format!("{}{}","/",directory_name))?
    }

    Ok(())
}