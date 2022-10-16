use clap::{arg, Command};
use itertools::Itertools;
use reqwest;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Command::new("words")
        .arg(
            arg!(-w --word <WORD> "Enter the word")
                .required(false)
                .default_value("eerf"),
        )
        .get_matches();
    let word = args
        .get_one::<String>("word")
        .expect("Cannot be None because of default val");

    let mut handles = Vec::new();
    for len in 1..word.chars().count() {
        for perm in word.chars().permutations(len).unique() {
            let perm = String::from_iter(perm);
            let handle = tokio::spawn(is_valid(perm.clone()));
            handles.push(handle);
        }
    }

    let mut op = Vec::with_capacity(handles.len());
    for handle in handles {
        if let Some(word) = handle.await.unwrap() {
            op.push(word);
        }
    }
    println!("Valid words are: \n{op:?}");
}

async fn is_valid(word: String) -> Option<String> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let resp = reqwest::get(url).await.unwrap();
    if resp
        .text()
        .await
        .unwrap()
        .contains("Sorry pal, we couldn't find definitions for the word you were looking for.")
    {
        None
    } else {
        Some(word)
    }
}
