"use client"; // n precisa de use client aqui
// é só pra páginas q a gente importa alguma coisa q o usuário vai interagir (e que por isso não podem estar do lado do servidor)
// tipo useState e tal

export interface Champion {
  name: string;
  image_url: string;
}
export interface AbilityChanges {
  name: string;
  changes: Array<string>;
}
export interface PatchCard {
  champion_name: Champion[];
  champion_image: Champion[];
  summary: string;
  context: string;
  abilities: AbilityChanges[];
  patch_version: string;
}
