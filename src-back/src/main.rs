use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;
use tokio;

use crate::utils::{clean_champion_name, clean_text, find_closest_patch, read_character_name};

mod utils;

const BASE_URL: &'static str = "https://www.leagueoflegends.com";

#[derive(Debug, Default)]
struct AbilityChange {
    name: String,
    changes: Vec<String>,
}

#[derive(Debug, Default)]
struct PatchCard {
    champion_name: String,
    champion_image: String,
    summary: String,
    context: String,
    abilities: Vec<AbilityChange>,
}

#[derive(Debug, Deserialize)]
struct PatchNote {
    version: String,
    release_date: String,
}

struct SelectedPage {
    document: Html,
    selector: Selector,
}

async fn select_page(
    client: &Client,
    version: &str,
) -> Result<SelectedPage, Box<dyn std::error::Error>> {
    let url = format!("{}/pt-br/news/tags/patch-notes/", BASE_URL);

    let response = client.get(&url).send().await?.text().await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse(&format!(
        r#"a[aria-label*="Notas da Atualização {}"]"#,
        version
    ))
    .unwrap();

    Ok(SelectedPage { document, selector })
}

async fn get_champion_info(
    client: &Client,
    page: &SelectedPage,
    desired_character: &str,
) -> Result<PatchCard, Box<dyn std::error::Error>> {
    if let Some(element) = page.document.select(&page.selector).next() {
        if let Some(href) = element.value().attr("href") {
            let full_url = format!("{}{}", BASE_URL, href);

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

                let champion_image = format!(
                    "https://am-a.akamaihd.net/image?f=https://ddragon.leagueoflegends.com/cdn/15.17.1/img/champion/{}.png",
                    clean_champion_name(&champion_name)
                );

                let is_desired_character = if (&desired_character.eq(&champion_name)).to_owned() {
                    true
                } else {
                    false
                };

                let patch_card = PatchCard {
                    champion_image,
                    champion_name,
                    summary,
                    context,
                    abilities,
                };

                if is_desired_character {
                    return Ok(patch_card);
                }
            }
        }
        return Ok(PatchCard::default());
    } else {
        //println!("Link to 'Update Notes {}' not found.", version);
        Ok(PatchCard::default())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let json_data = std::fs::read_to_string("patch_notes.json")?;

    let patches: Vec<PatchNote> = serde_json::from_str(&json_data).unwrap();

    let versions: Vec<String> = patches.iter().map(|p| p.version.clone()).collect();

    let _version = if let Some(closest) = find_closest_patch(&patches) {
        println!(
            "Closest version: {} - Date: {}",
            closest.version, closest.release_date
        );
        closest.version.to_string()
    } else {
        eprintln!("No version found.");
        return Ok(());
    };

    let desired_character = read_character_name().trim().to_owned();

    let mut infos = vec![];
    for version in versions.iter() {
        let page = select_page(&client, &version).await?;

        let info = get_champion_info(&client, &page, &desired_character).await?;

        if !info.champion_name.is_empty() {
            infos.push(info);
        }
    }

    infos.iter().for_each(|i| println!("\n\n{:?}", i));
    println!(
        "\n\n\nUpdated {} times in the last {} patches",
        infos.len(),
        versions.len()
    );

    Ok(())
}
