use super::card::{Card, ShowFront};
use super::hand::PlayCard;
use super::{Crowd, PlayerReference};
use crate::position::{AxisAnchor, Relative, RelativeAxis};
use crate::prelude::*;

#[derive(Component)]
pub struct BattleField(pub Vec<Entity>);

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_card_to_battlefield,
                position_cards_in_battle,
                update_crowd_value,
            )
                .run_if(in_state(MainState::Combat)),
        );
    }
}

fn move_card_to_battlefield(
    mut battlefields: Query<(&mut BattleField, &PlayerReference)>,
    mut cards: Query<&mut ShowFront>,
    mut play: EventReader<PlayCard>,
) {
    for event in play.read() {
        let Ok(mut card) = cards.get_mut(event.card) else {
            return;
        };
        card.0 = true;

        let Some((mut battlefield, _)) = battlefields
            .iter_mut()
            .find(|(_, player)| **player == event.player)
        else {
            return;
        };

        battlefield.0.push(event.card);
    }
}

#[allow(clippy::cast_precision_loss)] // The hand should never be very large
fn position_cards_in_battle(
    battlefields: Query<(&BattleField, &PlayerReference), Changed<BattleField>>,
    mut cards: Query<(&mut Relative, &mut Transform), With<Card>>,
) {
    for (hand, player) in &battlefields {
        let card_spacing = 150.0;

        let y_level = if *player == PlayerReference::Player {
            470.0
        } else {
            300.0
        };
        let anchor = if *player == PlayerReference::Player {
            AxisAnchor::Neg
        } else {
            AxisAnchor::Pos
        };

        let offset = hand.0.len() as f32 / 2.0 * card_spacing;
        for (index, card) in hand.0.iter().enumerate().rev() {
            let Ok((mut rel, mut trans)) = cards.get_mut(*card) else {
                continue;
            };

            let position = index as f32 * card_spacing - offset;
            trans.scale = Vec3::new(0.5, 0.5, 1.0);
            trans.translation.z = index as f32 * 10.0;
            *rel = Relative {
                x: Some(RelativeAxis {
                    anchor: AxisAnchor::Center,
                    amount: position,
                }),
                y: Some(RelativeAxis {
                    anchor,
                    amount: y_level,
                }),
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn update_crowd_value(
    mut players: Query<(&mut Crowd, &PlayerReference)>,
    battlefields: Query<(&BattleField, &PlayerReference), Changed<BattleField>>,
) {
    for (battle, player) in &battlefields {
        let Some((mut crowd, _)) = players.iter_mut().find(|(_, p)| *p == player) else {
            continue;
        };

        crowd.0 = battle.0.len() as u8;
    }
}
