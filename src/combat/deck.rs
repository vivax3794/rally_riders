use super::card::{CardGameplayInfo, CardInfo};
use crate::prelude::*;

#[derive(Resource)]
pub struct GlobalCards(pub Vec<CardInfo>);

impl GlobalCards {
    pub fn new(assets: &assets::Cards) -> Self {
        let base = vec![
            CardInfo {
                name: "Test Unit",
                img: assets.placeholder.clone_weak(),
                flavor_text: Some("Beep Boop, debugging is fun"),
                gameplay: CardGameplayInfo {
                    cast_crowd: 0,
                    minimum_crowd: 0,
                    hp: 2,
                    power: 2,
                },
            },
            CardInfo {
                name: "Ghost",
                img: assets.ghost.clone_weak(),
                flavor_text: Some("I am very scary :P"),
                gameplay: CardGameplayInfo {
                    cast_crowd: 1,
                    minimum_crowd: 0,
                    hp: 1,
                    power: 1,
                },
            },
        ];

        // FOR TESTING
        let mut deck = Vec::new();
        for _ in 0..10 {
            deck.extend_from_slice(&base);
        }
        Self(deck)
    }
}
