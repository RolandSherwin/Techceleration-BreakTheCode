use cli_table::{print_stdout, Cell, Style, Table};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, env};

#[derive(Debug, Serialize, Deserialize)]
struct CurrentsResponse {
    news: Vec<News>,
}

#[derive(Debug, Serialize, Deserialize)]
struct News {
    id: String,
    title: String,
    description: String,
    author: String,
    language: String,
    category: Vec<String>,
}

#[tokio::main]
async fn main() {
    let mut authors = BTreeSet::new();
    let mut languages = BTreeSet::new();
    let mut categories = BTreeSet::new();
    let news = fetch_news().await.news;
    news.iter().for_each(|new| {
        new.author.split(",").for_each(|auth| {
            let auth = auth.trim().to_string();
            if !auth.is_empty() {
                authors.insert(auth);
            }
        });
        languages.insert(new.language.clone());
        new.category.iter().cloned().for_each(|cat| {
            let cat = cat.trim().to_string();
            if !cat.is_empty() {
                categories.insert(cat);
            }
        })
    });
    println!("Use <Space> to select an option and <Enter> to confirm the options");
    let filtered_authors = filtered_from_cli(authors.into_iter().collect(), "Authors");
    let filtered_langs = filtered_from_cli(languages.into_iter().collect(), "Languages");
    let filtered_categories = filtered_from_cli(categories.into_iter().collect(), "Categories");
    let news = news
        .into_iter()
        .filter_map(|new| {
            if filtered_authors.contains(&new.author)
                || filtered_langs.contains(&new.language)
                || new
                    .category
                    .iter()
                    .any(|cat| filtered_categories.contains(cat))
            {
                Some(new.title)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let table = news
        .iter()
        .cloned()
        .map(|new| vec![new])
        .table()
        .title(vec!["News".cell().bold(true)])
        .bold(true);

    if let Err(_) = print_stdout(table) {
        println!("Here are the news for you!");
        news.into_iter().for_each(|new| println!("- {new}\n"));
    }
}

async fn fetch_news() -> CurrentsResponse {
    let api_key = env::var("CURRENTS_API_KEY").expect("Set CURRENTS_API_KEY");
    let url =
        format!("https://api.currentsapi.services/v1/latest-news?language=en&apiKey={api_key}");
    let resp = reqwest::get(url).await.unwrap();
    resp.json::<CurrentsResponse>().await.unwrap()
}

fn filtered_from_cli(list: Vec<String>, prompt: &str) -> BTreeSet<String> {
    let keep_indices = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&list)
        .defaults(&[true])
        .interact()
        .unwrap_or(Vec::new())
        .into_iter()
        .collect::<BTreeSet<_>>();
    list.into_iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            if keep_indices.contains(&idx) {
                Some(item)
            } else {
                None
            }
        })
        .collect()
}
