use crate::utils::{
    ChampionNames, LanguageInfo, clean_champion_name, clean_text, identify_language,
};
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Champion {
    pub name: String,
    pub first_image_url: String,
    pub second_image_url: String,
}

const BASE_URL: &'static str = "https://www.leagueoflegends.com";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AbilityChange {
    name: String,
    changes: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatchCard {
    pub champion_name: String,
    pub champion_image: ChampionNames,
    pub summary: String,
    pub context: String,
    pub abilities: Vec<AbilityChange>,
    pub patch_version: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchNote {
    pub version: String,
    pub release_date: String,
}

pub async fn get_all_champions(
    client: &Client,
) -> Result<Vec<Champion>, Box<dyn std::error::Error>> {
    let url = "https://www.leagueoflegends.com/en-us/champions/";
    let response = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&response);

    let card_selector = Selector::parse(r#"a[role="button"][aria-label]"#).unwrap();

    let mut champions = Vec::new();

    for element in document.select(&card_selector) {
        if let Some(champion_name) = element.value().attr("aria-label") {
            let image_url = generate_champion_image_url(champion_name);

            champions.push(Champion {
                name: champion_name.to_string(),
                first_image_url: image_url.lower,
                second_image_url: image_url.upper,
            });
        }
    }

    Ok(champions)
}

fn clean_champion_name_for_image(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

pub fn generate_champion_image_url(champion_name: &str) -> ChampionNames {
    let mut clean_name = clean_champion_name(champion_name);
    clean_name.lower = format!(
        "https://am-a.akamaihd.net/image?f=https://ddragon.leagueoflegends.com/cdn/15.18.1/img/champion/{}.png",
        clean_name.lower
    );
    clean_name.upper = format!(
        "https://am-a.akamaihd.net/image?f=https://ddragon.leagueoflegends.com/cdn/15.18.1/img/champion/{}.png",
        clean_name.upper
    );
    clean_name
}

pub async fn get_champion_notes_for_patch(
    client: &Client,
    desired_character: &str,
    selected_language: &mut LanguageInfo,
    version: &str,
) -> Result<PatchCard, StatusCode> {
    let url = format!(
        "{}/{}/news/tags/patch-notes/",
        BASE_URL, &selected_language.lang
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut patch_notes_label = format!("Patch {} Notes", version);

    if vec!["de-de", "pt-br"].contains(&selected_language.lang.as_str()) {
        selected_language.patch_notes = format!(
            "{} {}",
            identify_language(&selected_language.lang).patch_notes,
            version
        );
        patch_notes_label = selected_language.patch_notes.clone();
    }

    info!("Patch notes label: {}", patch_notes_label);

    let links: Vec<String> = {
        let document = Html::parse_document(&response);
        let selector = Selector::parse(&format!(r#"a[aria-label*="{}"]"#, patch_notes_label))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        info!(
            "Selector: {}",
            format!(r#"a[aria-label*="{}"]"#, patch_notes_label)
        );

        document
            .select(&selector)
            .filter_map(|el| {
                el.value()
                    .attr("href")
                    .map(|href| format!("{}{}", BASE_URL, href))
            })
            .collect()
    };

    info!("Links: {}", links.len());

    for full_url in links {
        let patch_html = client
            .get(&full_url)
            .send()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .text()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let document = Html::parse_document(&patch_html);
        let card_selector = Selector::parse("div.patch-change-block").unwrap();
        let champion_name_selector = Selector::parse("h3.change-title a").unwrap();
        let summary_selector = Selector::parse("p.summary").unwrap();
        let context_selector = Selector::parse("blockquote.blockquote.context").unwrap();
        let h4_or_ul_selector = Selector::parse("h4.change-detail-title, ul").unwrap();
        let li_selector = Selector::parse("li").unwrap();

        let mut collected_abilities = Vec::new();
        let mut summaries = Vec::new();
        let mut contexts = Vec::new();
        let mut image_url = String::new();
        let mut champion_name = String::new();

        for card in document.select(&card_selector) {
            let this_champion = card
                .select(&champion_name_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            if clean_champion_name(&this_champion).lower
                != clean_champion_name(desired_character).lower
            {
                continue;
            }

            champion_name = this_champion.clone();

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

            let mut current_ability = selected_language.base_stats.clone();
            let mut temp_changes = Vec::new();

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

            let mut champion_names = ChampionNames::default();
            if image_url.is_empty() {
                image_url =
                    generate_champion_image_url(&clean_champion_name(&champion_name).lower).lower;
                champion_names = generate_champion_image_url(&champion_name);
            }

            return Ok(PatchCard {
                champion_name: this_champion,
                champion_image: champion_names,
                summary: summaries.join(" / "),
                context: contexts.join(" / "),
                abilities: collected_abilities,
                patch_version: version.to_string(),
            });
        }
    }

    Ok(PatchCard::default())
}

pub const PATCH_NOTES_LIST: &[(&str, &str)] = &[
    ("25.19", "2025-09-24"),
    ("25.18", "2025-09-10"),
    ("25.17", "2025-08-27"),
    ("25.16", "2025-08-13"),
    ("25.15", "2025-07-29"),
    ("25.14", "2025-07-15"),
    ("25.13", "2025-06-24"),
    ("25.12", "2025-06-10"),
    ("25.11", "2025-05-27"),
    ("25.10", "2025-05-13"),
    ("25.09", "2025-04-29"),
    ("25.08", "2025-04-15"),
    ("25.07", "2025-04-01"),
];

pub async fn get_all_champion_patch_cards(
    client: &Client,
    desired_character: &str,
    mut selected_language: LanguageInfo,
) -> Result<Vec<PatchCard>, StatusCode> {
    let mut results: Vec<PatchCard> = Vec::new();

    for (ver, _date) in PATCH_NOTES_LIST.iter() {
        match get_champion_notes_for_patch(client, desired_character, &mut selected_language, ver)
            .await
        {
            Ok(card) => {
                if !card.champion_name.is_empty() {
                    info!("Card found");
                    results.push(card);
                }
            }
            Err(_) => {
                continue;
            }
        }
    }

    Ok(results)
}

pub async fn get_champion_notes_and_select_page(
    client: &Client,
    desired_character: &str,
    selected_language: &mut LanguageInfo,
    version: &str,
) -> Result<PatchCard, StatusCode> {
    let url = format!(
        "{}/{}/news/tags/patch-notes/",
        BASE_URL, &selected_language.lang
    );

    info!("URL found: {}", url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if vec!["de-de", "pt-br"].contains(&selected_language.lang.as_str()) {
        selected_language.patch_notes = format!(
            "{} {}",
            identify_language(&selected_language.lang).patch_notes,
            version
        );
    }

    let patch_notes_label = selected_language.patch_notes.clone();

    info!("Patch note label found");

    let links: Vec<String> = {
        let document = Html::parse_document(&response);
        let selector = Selector::parse(&format!(r#"a[aria-label*="{}"]"#, patch_notes_label))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        document
            .select(&selector)
            .filter_map(|el| {
                el.value()
                    .attr("href")
                    .map(|href| format!("{}{}", BASE_URL, href))
            })
            .collect()
    };

    for full_url in links {
        let patch_html = client
            .get(&full_url)
            .send()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .text()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let document = Html::parse_document(&patch_html);
        info!("Document found");

        let card_selector = Selector::parse("div.patch-change-block").unwrap();
        let champion_name_selector = Selector::parse("h3.change-title a").unwrap();
        let summary_selector = Selector::parse("p.summary").unwrap();
        let context_selector = Selector::parse("blockquote.blockquote.context").unwrap();
        let h4_or_ul_selector = Selector::parse("h4.change-detail-title, ul").unwrap();
        let li_selector = Selector::parse("li").unwrap();

        info!("Selectors found");

        let mut collected_abilities = vec![];
        let mut summaries = vec![];
        let mut contexts = vec![];
        let mut image_url = String::new();
        let mut champion_name = String::new();

        for card in document.select(&card_selector) {
            let this_champion = card
                .select(&champion_name_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            if clean_champion_name(&this_champion).lower
                != clean_champion_name(desired_character).lower
            {
                continue;
            }

            champion_name = this_champion;

            info!("Champion: {}", champion_name);

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

            let mut current_ability = selected_language.base_stats.clone();
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

            let mut champion_names = ChampionNames::default();
            if image_url.is_empty() {
                image_url =
                    generate_champion_image_url(&clean_champion_name(&champion_name).lower).lower;
                champion_names = generate_champion_image_url(&champion_name);
            }
            if image_url.is_empty() {
                image_url = format!(
                    "https://am-a.akamaihd.net/image?f=https://ddragon.leagueoflegends.com/cdn/15.17.1/img/champion/{}.png",
                    clean_champion_name(&champion_name).lower
                );
            }

            return Ok(PatchCard {
                champion_name,
                champion_image: champion_names,
                summary: summaries.join(" / "),
                context: contexts.join(" / "),
                abilities: collected_abilities,
                patch_version: selected_language.patch_notes.clone(),
            });
        }
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn get_champion_by_name(
    client: &Client,
    champion_name: &str,
) -> Result<Option<Champion>, Box<dyn std::error::Error>> {
    let champions = get_all_champions(client).await?;

    let champion = champions
        .into_iter()
        .find(|c| c.name.trim().to_lowercase() == champion_name.trim().to_lowercase());

    Ok(champion)
}
