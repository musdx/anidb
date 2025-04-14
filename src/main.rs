use reqwest::{
    self, Error,
    blocking::{Client, Response},
};
use scraper;

fn main() {
    let client: Client = reqwest::blocking::Client::new();
    let res: Response = client
        .get("https://anidb.net/anime/season/2025/spring/?do=calendar&h=1&view=smallgrid")
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36")
        .send()
        .unwrap();
    let html: String = res.text().unwrap();
}
