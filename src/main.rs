use reqwest::blocking::{Client, Response};
use scraper;

fn main() {
    let client: Client = reqwest::blocking::Client::new();
    let res: Response = client
        .get("https://anidb.net/anime/season/2025/spring/?do=calendar&h=1&view=smallgrid")
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36")
        .send()
        .unwrap();
    let html: String = res.text().unwrap();

    let doc = scraper::Html::parse_document(&html);

    let anime_selector = &scraper::Selector::parse("a.name-colored").unwrap();

    let anime_titles: Vec<String> = doc
        .select(&anime_selector)
        .map(|anime| anime.text().collect::<String>())
        .collect();

    let mut anime_links: Vec<String> = Vec::new();

    for link in doc.select(&anime_selector) {
        anime_links.push(String::from("https://anidb.net") + link.value().attr("href").unwrap());
    }

    for (title, link) in anime_titles.iter().zip(anime_links.iter()) {
        println!("Title: {}\nLinks {}\n", title, link)
    }
}
