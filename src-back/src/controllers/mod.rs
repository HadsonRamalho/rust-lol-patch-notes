use axum::{Json, extract::Query};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    champions::{
        self, Champion, PatchCard, generate_champion_image_url, get_all_champion_patch_cards,
    },
    utils::identify_language,
};

#[derive(Deserialize, Serialize)]
pub struct PatchNotesInput {
    pub champion_name: String,
    pub language: String,
}

#[derive(Deserialize)]
pub struct NameInput {
    name: String,
}

pub async fn list_champions() -> Result<Json<Vec<Champion>>, StatusCode> {
    let client = Client::new();
    let champions = champions::get_all_champions(&client).await.expect("erro");
    Ok(Json(champions))
}

pub async fn champion_info(
    Query(name): Query<NameInput>,
) -> Result<Json<Option<Champion>>, StatusCode> {
    let client = Client::new();

    let champion = champions::get_champion_by_name(&client, &name.name)
        .await
        .expect("erro");
    Ok(Json(champion))
}

#[axum::debug_handler]
pub async fn champion_notes(
    Query(request): Query<PatchNotesInput>,
) -> Result<Json<Vec<PatchCard>>, StatusCode> {
    let client = Client::new();
    let language = identify_language(&request.language);
    let mut notes = get_all_champion_patch_cards(&client, &request.champion_name, language)
        .await
        .expect("erro");
    info!("Patch notes found: {}", notes.len());
    if notes.len() > 0 {
        notes[0].champion_image = generate_champion_image_url(&request.champion_name);
        Ok(Json(notes))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
