use reqwest::blocking::{Client, Response};
use scraper;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// Color thing
const _BLACK: &str = "\x1b[30m";
const _RED: &str = "\x1b[31m";
const _GREEN: &str = "\x1b[32m";
const _YELLOW: &str = "\x1b[33m";
const _BLUE: &str = "\x1b[34m";
const _MAGENTA: &str = "\x1b[35m";
const _CYAN: &str = "\x1b[36m";
const _WHITE: &str = "\x1b[37m";
const _BRIGHT_BLACK: &str = "\x1b[30;1m";
const _BRIGHT_RED: &str = "\x1b[31;1m";
const _BRIGHT_GREEN: &str = "\x1b[32;1m";
const _BRIGHT_YELLOW: &str = "\x1b[33;1m";
const _BRIGHT_BLUE: &str = "\x1b[34;1m";
const _BRIGHT_MAGENTA: &str = "\x1b[35;1m";
const _BRIGHT_CYAN: &str = "\x1b[36;1m";
const _BRIGHT_WHITE: &str = "\x1b[37;1m";
// Effect thing
const _BOLD: &str = "\x1b[1m";
const _RESET: &str = "\x1b[0m";

fn display_image(image_path: &str) -> std::io::Result<()> {
    let status = Command::new("kitty").arg("icat").arg(image_path).status()?;

    if !status.success() {
        eprintln!("Error: Command exited with status: {:?}", status.code());
    }
    Ok(())
}

fn main() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let current_year = (since_the_epoch.as_secs() / 31536000) + 1970;

    let mut mon: String = String::new();
    let mut year: String = String::new();
    let mut _web_link = String::new();

    loop {
        println!("Type the Month that you want to check (in number): ");
        std::io::stdin().read_line(&mut mon).expect("A DAM MONTH");

        println!("Type a year that you want to check: ");
        std::io::stdin().read_line(&mut year).expect("something");

        if let Ok(month) = mon.trim().parse::<i8>() {
            if month >= 1 && month < 13 {
                if let Ok(years) = year.trim().parse::<u64>() {
                    if years <= current_year {
                        _web_link = format!(
                            "https://anidb.net/anime/season/{}/{}/?do=calendar&h=1&view=smallgrid",
                            year.trim(),
                            mon.trim()
                        );
                        break;
                    } else {
                        println!("{}\nYears to big\n{}", _RED, _RESET);
                        mon = String::new();
                        year = String::new();
                    }
                } else {
                    println!("{}\nIncorect input months\n{}", _RED, _RESET);
                    mon = String::new();
                    year = String::new();
                }
            } else {
                println!("{}\nMonths too big\n{}", _RED, _RESET);
                mon = String::new();
                year = String::new();
            }
        } else {
            println!("{}\nIncorect input months\n{}", _RED, _RESET);
            mon = String::new();
            year = String::new();
        }
    }

    println!(
        "{_GREEN}\nThe date you choosed is: {} / {}\n{_RESET}",
        mon.trim(),
        year.trim()
    );

    let client: Client = Client::new();
    let res: Response = client
        .get(_web_link)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36")
        .send()
        .unwrap();
    let html: String = res.text().unwrap();

    let doc = scraper::Html::parse_document(&html);

    let anime_selector = scraper::Selector::parse("a.name-colored").unwrap();
    let rating_selector = scraper::Selector::parse("div.votes.rating").unwrap();
    let div_tag = scraper::Selector::parse("div.tags").unwrap();
    let tags = scraper::Selector::parse("span.tagname").unwrap();
    let data_div = scraper::Selector::parse("div.data").unwrap();
    let img_selector = scraper::Selector::parse("img.g_image.g_bubble.small").unwrap();

    let mut anime_image: Vec<&str> = Vec::new();
    let mut anime_links: Vec<String> = Vec::new();
    let mut anime_tags: Vec<Vec<String>> = Vec::new();
    let mut anime_rating: Vec<String> = Vec::new();

    for anime in doc.select(&data_div) {
        let mut in_tags: Vec<String> = vec![String::from("None")];
        for div in anime.select(&div_tag) {
            in_tags = div
                .select(&tags)
                .map(|x| x.text().collect::<String>())
                .collect();
        }
        anime_tags.push(in_tags);
        let ratings: Vec<String> = anime
            .select(&rating_selector)
            .map(|r| r.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if ratings.is_empty() {
            anime_rating.push("Rating: None".to_string());
        } else {
            anime_rating.push(ratings[0].clone());
        }
    }

    let anime_titles: Vec<String> = doc
        .select(&anime_selector)
        .map(|anime| anime.text().collect::<String>())
        .collect();

    for link in doc.select(&anime_selector) {
        anime_links.push(String::from("https://anidb.net") + link.value().attr("href").unwrap());
    }

    for image in doc.select(&img_selector) {
        anime_image.push(image.value().attr("src").unwrap());
    }

    let anime_count = anime_titles.len();

    for i in 0..anime_count {
        if let Err(e) = display_image(anime_image[i]) {
            eprintln!("Failed to display image: {}", e);
        }
        println!(
            "{_BOLD}Title:{_RESET} {_CYAN}{}{_RESET}\n{_BOLD}Links:{_RESET} {_BRIGHT_YELLOW}{}{_RESET}\n{}\n{_BOLD}Genre:{_RESET} {:?}\n",
            anime_titles[i].trim(),
            anime_links[i].trim(),
            anime_rating[i].trim(),
            anime_tags[i]
        )
    }
}
