use reqwest;
use scraper::{Html, Selector};
use structopt::StructOpt;
use termion::{color, style};
use std::path::PathBuf;

/*
Plans:
Grabs the articles
Provides links to the specific news report with date and time

Summarize articles with OpenAI API
Pretty terminal GUI or integrate with teams?

Watermarking
https://www.nytimes.com/interactive/2023/02/17/business/ai-text-detection.html
*/

#[derive(StructOpt)]
struct Cli {}

fn main() {
    let url = "https://krebsonsecurity.com/";

    println!("{}{}Fetching from: {}{}", style::Bold, color::Fg(color::Yellow), url, style::Reset);
    // Grab main page content
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    // Grabbing information on the main page that contains links and titles
    let entry_selector = scraper::Selector::parse("header.entry-header").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    for header in document.select(&entry_selector) {
        for link in header.select(&link_selector) {
            if let Some(href) = link.value().attr("href") {
                // Ignore links that contain comments (due to redundancy)
                if !link.value().classes().any(|class| class == "link-comments") {
                    let title = link.inner_html();
                    let cleaned_title = strip_html_tags(&title);
                    println!("{}{}Link: {} | Title: {}{}{}", style::Bold, color::Fg(color::Green), href, color::Fg(color::Cyan), cleaned_title, style::Reset);

                    // Grab the new link related to the threatwatch news
                    let res1 = reqwest::blocking::get(href).unwrap().text().unwrap();
                    let doc1 = scraper::Html::parse_document(&res1);
                    // Grab the information stored within the link that is important
                    let general_selector = scraper::Selector::parse("div.entry-content").unwrap();
                    let p_selector = Selector::parse("p").unwrap();

                    for entry_content in doc1.select(&general_selector) {
                        for node in entry_content.select(&p_selector) {
                            let content = strip_html_tags(&node.inner_html());
                            println!("{}", content);
                        }
                    }
                    println!("{}", "-".repeat(80));
                }
            }
        }
    }
}

fn strip_html_tags(html: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}
