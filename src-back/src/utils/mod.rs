use chrono::{NaiveDate, Utc};

use crate::PatchNote;

#[derive(Debug)]
pub struct LanguageInfo {
    pub lang: String,
    pub base_stats: String,
    pub patch_notes: String,
}

pub fn select_language() -> LanguageInfo {
    use std::io::stdin;
    let mut option = String::new();
    stdin()
        .read_line(&mut option)
        .expect("Error while reading language option");
    identify_language(&option)
}

pub fn identify_language(option: &str) -> LanguageInfo {
    match option.trim() {
        "pt-br" => LanguageInfo {
            patch_notes: "Notas da Atualização".to_string(),
            lang: option.trim().to_string(),
            base_stats: "Atributos base".to_string(),
        },
        "en-us" => LanguageInfo {
            patch_notes: "Patch Notes".to_string(),
            lang: option.trim().to_string(),
            base_stats: "Base Stats".to_string(),
        },
        "de-de" => LanguageInfo {
            lang: option.trim().to_string(),
            base_stats: "Grundwerte".to_string(),
            patch_notes: "Patchnotizen".to_string(),
        },
        _ => LanguageInfo {
            patch_notes: "Notas da Atualização".to_string(),
            lang: option.trim().to_string(),
            base_stats: "Atributos base".to_string(),
        },
    }
}

pub fn clean_text(text: &str) -> String {
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

pub fn find_closest_patch(patches: &[PatchNote]) -> Option<&PatchNote> {
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

pub fn clean_champion_name(name: &str) -> String {
    let mut result = String::new();
    let mut lowercase_next = false;

    for c in name.chars() {
        if c.is_alphanumeric() {
            if lowercase_next {
                result.push(c.to_ascii_lowercase());
                lowercase_next = false;
            } else {
                result.push(c);
            }
        } else {
            lowercase_next = true;
        }
    }

    result
}

pub fn read_character_name() -> String {
    use std::io::stdin;
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Error while reading character name");
    name
}
