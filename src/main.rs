use std::env;
use std::fs;
use std::io::{stdin, Write};
use std::process::Command;

use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repository {
    html_url: String,
}


fn main() {
    let user: String = get_user();
    let mut username = user;
    println!("Introduced user is: {}", username);

    let repositories = get_repositories(&username).unwrap();

    create_directory(&username);

    for repository in repositories.iter() {
        println!("The repository url is: {}", repository.html_url);
        git_clone(repository.html_url.as_str(), &username);
    }
}


#[tokio::main]
async fn get_repositories(username: &str) -> Result<Vec<Repository>, Error> {
    let client = reqwest::Client::new();

    let request_url = format!("https://api.github.com/users/{user}/repos",
                              user = username);

    let response = client
        .get(request_url)
        .header(USER_AGENT, "Mirepoix")
        .send()
        .await?;

    let repositories: Vec<Repository> = response.json().await?;

    Ok(repositories)
}

fn get_user() -> String {
    println!("Insert your gitHub username: ");

    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    str::replace(input_string.as_str(), "\n", "")
}

fn create_directory(username: &str) -> std::io::Result<()> {
    let os = env::consts::OS;

    match os {
        "windows" => fs::create_dir_all(format!("{}{}", "C:\\", username))?,
        _ => fs::create_dir_all(format!("{}{}", "/", username))?
    }

    Ok(())
}

fn git_clone(repository: &str, username: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", format!("git -C C:\\{} clone {}.git", username, repository).as_str()])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("git -C /{} clone {}.git", username, repository).as_str())
            .output()
            .expect("failed to execute process")
    };

    let exit_code = output.status;
    println!("{}", exit_code)
}