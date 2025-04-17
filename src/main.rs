use reqwest::blocking::{Client, Response};
use scraper;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let current_year = (since_the_epoch.as_secs() / 31536000) + 1970;

    let mut mon: String = String::new();
    let mut year: String = String::new();
    let mut web_link = String::new();

    loop {
        println!("Type the Month that you want to check (in number): ");
        std::io::stdin().read_line(&mut mon).expect("A DAM MONTH");

        println!("Type a year that you want to check: ");
        std::io::stdin().read_line(&mut year).expect("something");

        if let Ok(month) = mon.trim().parse::<i8>() {
            if month >= 1 && month < 13 {
                if let Ok(years) = year.trim().parse::<u64>() {
                    if years <= current_year {
                        web_link = format!(
                            "https://anidb.net/anime/season/{}/{}/?do=calendar&h=1&view=smallgrid",
                            year.trim(),
                            mon.trim()
                        );
                        break;
                    } else {
                        println!("\nYears to big\n")
                    }
                } else {
                    println!("\nIncorect input months\n")
                }
            } else {
                println!("\nMonths too big\n")
            }
        } else {
            println!("\nIncorect input months\n")
        }
    }

    println!("The date you choosed is: {mon}/{year}");

    let client: Client = Client::new();
    let res: Response = client
        .get(web_link)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36")
        .send()
        .unwrap();
    let html: String = res.text().unwrap();

    let doc = scraper::Html::parse_document(&html);

    let anime_selector = &scraper::Selector::parse("a.name-colored").unwrap();
    let mut anime_links: Vec<String> = Vec::new();
    let anime_titles: Vec<String> = doc
        .select(&anime_selector)
        .map(|anime| anime.text().collect::<String>())
        .collect();

    for link in doc.select(&anime_selector) {
        anime_links.push(String::from("https://anidb.net") + link.value().attr("href").unwrap());
    }

    for (title, link) in anime_titles.iter().zip(anime_links.iter()) {
        println!("Title: {}\nLinks: {}\n", title, link)
    }
}
