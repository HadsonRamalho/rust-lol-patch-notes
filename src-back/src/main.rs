use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use tokio;

use crate::utils::{
    LanguageInfo, clean_champion_name, clean_text, find_closest_patch, identify_language,
    read_character_name, select_language,
};

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
    selected_language: &mut LanguageInfo,
) -> Result<SelectedPage, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/{}/news/tags/patch-notes/",
        BASE_URL, &selected_language.lang
    );

    let response = client.get(&url).send().await?.text().await?;

    let document = Html::parse_document(&response);
    if vec!["de-de", "pt-br"].contains(&selected_language.lang.as_str()) {
        selected_language.patch_notes = format!(
            "{} {}",
            identify_language(&selected_language.lang).patch_notes,
            &version
        );
    }

    let selector = Selector::parse(&format!(
        r#"a[aria-label*="{}"]"#,
        selected_language.patch_notes
    ))
    .unwrap();

    Ok(SelectedPage { document, selector })
}

async fn get_champion_info(
    client: &Client,
    page: &SelectedPage,
    desired_character: &str,
    selected_language: &LanguageInfo,
) -> Result<PatchCard, Box<dyn std::error::Error>> {
    let mut collected_abilities = vec![];
    let mut summaries = vec![];
    let mut contexts = vec![];
    let mut image_url = String::new();
    let mut champion_name = String::new();

    if let Some(element) = page.document.select(&page.selector).next() {
        if let Some(href) = element.value().attr("href") {
            let full_url = format!("{}{}", BASE_URL, href);
            let patch_page = client.get(&full_url).send().await?.text().await?;
            let document = Html::parse_document(&patch_page);

            let card_selector = Selector::parse("div.patch-change-block").unwrap();
            let champion_name_selector = Selector::parse("h3.change-title a").unwrap();
            let summary_selector = Selector::parse("p.summary").unwrap();
            let context_selector = Selector::parse("blockquote.blockquote.context").unwrap();

            for card in document.select(&card_selector) {
                let this_champion = card
                    .select(&champion_name_selector)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                if clean_champion_name(&this_champion) != clean_champion_name(desired_character) {
                    continue;
                }

                champion_name = this_champion;

                if let Some(summary_elem) = card.select(&summary_selector).next() {
                    let summary = summary_elem.text().collect::<String>();
                    if !summary.is_empty() {
                        summaries.push(summary);
                    }
                }

                if let Some(context_elem) = card.select(&context_selector).next() {
                    let context = clean_text(&context_elem.text().collect::<String>());
                    if !context.is_empty() {
                        contexts.push(context);
                    }
                }

                let h4_or_ul_selector = Selector::parse("h4.change-detail-title, ul").unwrap();
                let li_selector = Selector::parse("li").unwrap();
                let mut current_ability = selected_language.base_stats.to_string();
                let mut temp_changes = vec![];

                for element in card.select(&h4_or_ul_selector) {
                    if element.value().name() == "h4" {
                        if !temp_changes.is_empty() {
                            collected_abilities.push(AbilityChange {
                                name: current_ability.clone(),
                                changes: temp_changes.clone(),
                            });
                            temp_changes.clear();
                        }
                        current_ability = element.text().collect::<String>().trim().to_string();
                    } else if element.value().name() == "ul" {
                        let changes = element
                            .select(&li_selector)
                            .map(|li| li.text().collect::<String>().trim().to_string())
                            .collect::<Vec<_>>();
                        temp_changes.extend(changes);
                    }
                }

                if !temp_changes.is_empty() {
                    collected_abilities.push(AbilityChange {
                        name: current_ability.clone(),
                        changes: temp_changes.clone(),
                    });
                }

                if image_url.is_empty() {
                    image_url = format!(
                        "https://am-a.akamaihd.net/image?f=https://ddragon.leagueoflegends.com/cdn/15.17.1/img/champion/{}.png",
                        clean_champion_name(&champion_name)
                    );
                }
            }
        }
    }

    if champion_name.is_empty() {
        return Ok(PatchCard::default());
    }

    Ok(PatchCard {
        champion_name,
        champion_image: image_url,
        summary: summaries.join(" / "),
        context: contexts.join(" / "),
        abilities: collected_abilities,
    })
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

    println!("Language: (pt-br, en-us or de-de)");
    let mut selected_language = select_language();

    println!("Selected: {:?}", selected_language);

    let mut infos = vec![];
    for version in versions.iter() {
        if selected_language.lang.eq("en-us") {
            selected_language.patch_notes = format!("Patch {} Notes", &version);
        }

        let page = select_page(&client, &version, &mut selected_language).await?;

        let info =
            get_champion_info(&client, &page, &desired_character, &selected_language).await?;

        if !info.champion_name.is_empty() {
            infos.push(info);
        }
    }

    infos.iter().for_each(|i| println!("\n\n{:#?}", i));
    println!(
        "\n\n\nUpdated {} times in the last {} patches",
        infos.len(),
        versions.len()
    );

    Ok(())
}
