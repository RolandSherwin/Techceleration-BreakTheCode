use clap::{arg, Command};
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    nickname: String,
    organisations: Vec<Organization>,
    pull_requests: Vec<PullRequests>,
}

#[derive(Table)]
struct UserRow {
    #[table(title = "Username", justify = "Justify::Left")]
    name: String,
    #[table(title = "No of Organizations", justify = "Justify::Center")]
    organisations: isize,
    #[table(title = "No of Pull Requests", justify = "Justify::Center")]
    pull_requests: isize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Organization {}

#[derive(Debug, Serialize, Deserialize)]
struct PullRequests {}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Command::new("poi")
        .arg(
            arg!(-p --pages <MAX_PAGES> "Number of pages to go through")
                .required(false)
                .default_value("1")
                .value_parser(clap::value_parser!(isize)),
        )
        .get_matches();
    let max_pages = *args
        .get_one::<isize>("pages")
        .expect("Cannot be None because of default val");

    // fetch each api concurrently
    let handles = (0..max_pages)
        .map(|page| {
            let url = format!("https://24pullrequests.com/users.json?page={page}");
            tokio::spawn(fetch_users(url.clone()))
        })
        .collect::<Vec<_>>();

    let mut users = Vec::with_capacity(handles.len());
    for handle in handles {
        users.push(handle.await.unwrap())
    }
    let users = users
        .into_iter()
        .flatten()
        .map(|user| UserRow {
            name: user.nickname,
            organisations: user.organisations.len() as isize,
            pull_requests: user.pull_requests.len() as isize,
        })
        .collect::<Vec<_>>();
    if let Err(_) = print_stdout(users.with_title()) {
        println!("Username | Orgs | PRs");
        for user in &users {
            println!(
                "{}  {}  {}",
                user.name, user.organisations, user.pull_requests
            );
        }
    }
    println!("User count: {}", users.len());
}

async fn fetch_users(url: String) -> Vec<User> {
    let resp = reqwest::get(url).await.expect("Failed to get response");
    resp.json::<Vec<User>>()
        .await
        .expect("Failed to parse json")
}
