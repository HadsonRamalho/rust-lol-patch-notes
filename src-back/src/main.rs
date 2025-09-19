use chrono::{NaiveDate, Utc};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;
use tokio;

#[derive(Debug)]
struct AbilityChange {
    name: String,
    changes: Vec<String>,
}

#[derive(Debug)]
struct PatchCard {
    champion_name: String,
    summary: String,
    context: String,
    abilities: Vec<AbilityChange>,
}

#[derive(Debug, Deserialize)]
struct PatchNote {
    version: String,
    release_date: String,
}

fn clean_text(text: &str) -> String {
    let replaced = text.replace('\n', " ").replace('\t', " ");
    let mut result = String::new();
    let mut last_was_space = false;
    for c in replaced.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
        } else {
            result.push(c);
            last_was_space = false;
        }
    }
    result.trim().to_string()
}

fn find_closest_patch(patches: &[PatchNote]) -> Option<&PatchNote> {
    let today = Utc::now().naive_utc().date();

    patches
        .iter()
        .filter_map(|patch| {
            NaiveDate::parse_from_str(&patch.release_date, "%Y-%m-%d")
                .ok()
                .filter(|&d| d <= today)
                .map(|date| (patch, date))
        })
        .min_by_key(|(_, date)| (today - *date).num_days())
        .map(|(patch, _)| patch)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = std::fs::read_to_string("patch_notes.json")?;

    let patches: Vec<PatchNote> = serde_json::from_str(&json_data).unwrap();

    let version = if let Some(closest) = find_closest_patch(&patches) {
        println!(
            "Closest version: {} - Date: {}",
            closest.version, closest.release_date
        );
        closest.version.to_string()
    } else {
        eprintln!("No version found.");
        return Ok(());
    };

    let base_url = "https://www.leagueoflegends.com";
    let url = format!("{}/pt-br/news/tags/patch-notes/", base_url);

    let client = Client::new();
    let response = client.get(&url).send().await?.text().await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse(&format!(
        r#"a[aria-label*="Notas da Atualização {}"]"#,
        version
    ))
    .unwrap();

    if let Some(element) = document.select(&selector).next() {
        if let Some(href) = element.value().attr("href") {
            let full_url = format!("{}{}", base_url, href);

            let patch_page = client.get(&full_url).send().await?.text().await?;

            let document = Html::parse_document(&patch_page);

            let card_selector = Selector::parse("div.patch-change-block").unwrap();
            let champion_name_selector = Selector::parse("h3.change-title a").unwrap();
            let summary_selector = Selector::parse("p.summary").unwrap();
            let context_selector = Selector::parse("blockquote.blockquote.context").unwrap();
            let ability_title_selector =
                Selector::parse("h4.change-detail-title.ability-title").unwrap();
            let change_list_selector = Selector::parse("ul").unwrap();

            for card in document.select(&card_selector) {
                let champion_name = card
                    .select(&champion_name_selector)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                if champion_name.is_empty() {
                    continue;
                }

                let summary = card
                    .select(&summary_selector)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let context = card
                    .select(&context_selector)
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()))
                    .unwrap_or_default();

                let mut abilities = Vec::new();

                let mut ability_iter = card.select(&ability_title_selector).peekable();
                let mut ul_iter = card.select(&change_list_selector);

                while let Some(ability_title) = ability_iter.next() {
                    let ability_name = ability_title.text().collect::<String>();

                    let changes = if let Some(changes_ul) = ul_iter.next() {
                        changes_ul
                            .select(&Selector::parse("li").unwrap())
                            .map(|li| li.text().collect::<String>())
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };

                    abilities.push(AbilityChange {
                        name: ability_name,
                        changes,
                    });
                }

                if abilities.is_empty() {
                    continue;
                }

                let patch_card = PatchCard {
                    champion_name,
                    summary,
                    context,
                    abilities,
                };

                println!("{:#?}", patch_card);
            }
        }
    } else {
        println!("Link to 'Update Notes {}' not found.", version);
    }

    Ok(())
}
