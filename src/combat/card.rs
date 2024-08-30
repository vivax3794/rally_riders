use crate::prelude::*;

#[derive(Component)]
pub struct CardGameplayInfo {
    pub cast_crowd: u8,
    pub minimum_crowd: u8,
    pub hp: u8,
    pub power: u8,
}

pub struct CardInfo {
    pub gameplay: CardGameplayInfo,
    pub name: String,
    pub img: Handle<Image>,
    pub flavor_text: Option<String>,
}

#[derive(Resource)]
pub struct GlobalCards(Vec<CardInfo>);

#[derive(Component)]
pub struct Deck(Vec<Entity>);

impl Default for GlobalCards {
    fn default() -> Self {
        Self(vec![])
    }
}
