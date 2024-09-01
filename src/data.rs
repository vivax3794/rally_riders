use crate::combat::card::CardInfo;
use crate::prelude::*;

#[derive(Resource)]
pub struct PlayerInfo {
    pub max_hp: u8,
    pub current_hp: u8,
    pub deck: Vec<CardInfo>,
}
