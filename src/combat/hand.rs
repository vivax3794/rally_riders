use super::card::{Card, CardGray, Deck, ShowFront};
use super::hovering::Hovered;
use super::{AllowedToPlay, PlayerReference};
use crate::position::{AxisAnchor, Relative, RelativeAxis};
use crate::prelude::*;

pub struct HandPlugin;

#[derive(Component)]
pub struct InHand;

#[derive(Component)]
pub struct Focused;

#[derive(Component)]
pub struct Hand(pub Vec<Entity>);

#[derive(Event)]
pub struct DrawCard {
    pub player: PlayerReference,
    pub amount: usize,
}

#[derive(Event)]
pub struct PlayCard {
    pub player: PlayerReference,
    pub card: Entity,
    pub hand_index: usize,
}

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawCard>();
        app.add_event::<PlayCard>();
        app.add_systems(
            Update,
            (
                draw_cards,
                setup_cards_in_hand,
                position_cards_in_hand,
                show_allowed_cards,
                handle_play_input,
                remove_played_card_from_hand,
            )
                .run_if(in_state(MainState::Combat)),
        );
    }
}

fn draw_cards(
    mut commands: Commands,
    mut decks: Query<(&mut Deck, &PlayerReference)>,
    mut events: EventReader<DrawCard>,
    mut hands: Query<(&mut Hand, &PlayerReference)>,
) {
    for event in events.read() {
        let Some((mut deck, _)) = decks
            .iter_mut()
            .find(|(_, player)| event.player == **player)
        else {
            continue;
        };
        let Some((mut hand, _)) = hands
            .iter_mut()
            .find(|(_, player)| event.player == **player)
        else {
            continue;
        };

        for _ in 0..event.amount {
            let Some(card) = deck.0.pop() else {
                continue;
            };
            hand.0.push(card);

            let mut card = commands.entity(card);
            card.insert((InHand, event.player));
        }
    }
}

fn setup_cards_in_hand(
    mut query: Query<(&mut ShowFront, &mut Transform, &PlayerReference), Added<InHand>>,
) {
    for (mut front, mut trans, player) in &mut query {
        if *player == PlayerReference::Player {
            trans.scale = Vec3::new(0.7, 0.7, 1.0);
            front.0 = true;
        } else {
            trans.scale = Vec3::new(0.5, 0.5, 1.0);
        }
    }
}

#[allow(clippy::cast_precision_loss)] // The hand should never be very large
fn position_cards_in_hand(
    mut commands: Commands,
    hands: Query<(&Hand, &PlayerReference)>,
    mut cards: Query<(&mut Relative, &mut Transform, Option<&Hovered>), With<Card>>,
) {
    for (hand, player) in &hands {
        let hand_spacing;
        let y_level;
        let anchor;

        if *player == PlayerReference::Player {
            hand_spacing = 100.0;
            y_level = 200.0;
            anchor = AxisAnchor::Neg;
        } else {
            hand_spacing = 40.0;
            y_level = 40.0;
            anchor = AxisAnchor::Pos;
        }

        let offset = hand.0.len() as f32 / 2.0 * hand_spacing;
        let mut already_hovered = false;
        for (index, card) in hand.0.iter().enumerate().rev() {
            let Ok((mut rel, mut trans, hovered)) = cards.get_mut(*card) else {
                continue;
            };

            let position = index as f32 * hand_spacing - offset;
            trans.translation.z = index as f32 * 10.0;
            let mut is_hovered = false;
            if hovered.is_some() && !already_hovered {
                already_hovered = true;
                is_hovered = true;
                trans.translation.z += 100.0;

                commands.entity(*card).insert(Focused);
            } else {
                commands.entity(*card).remove::<Focused>();
            }
            *rel = Relative {
                x: Some(RelativeAxis {
                    anchor: AxisAnchor::Center,
                    amount: position,
                }),
                y: Some(RelativeAxis {
                    anchor,
                    amount: y_level + is_hovered.then_some(40.0).unwrap_or_default(),
                }),
            }
        }
    }
}

fn show_allowed_cards(
    mut commands: Commands,
    query: Query<(Entity, Option<&AllowedToPlay>), With<InHand>>,
) {
    for (card, allowed) in &query {
        if allowed.is_some() {
            commands.entity(card).remove::<CardGray>();
        } else {
            commands.entity(card).insert(CardGray);
        }
    }
}

fn handle_play_input(
    cards: Query<Entity, (With<Focused>, With<AllowedToPlay>)>,
    hands: Query<(&Hand, &PlayerReference)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut play: EventWriter<PlayCard>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(card) = cards.get_single() else {
            return;
        };

        let Some((player_hand, _)) = hands
            .iter()
            .find(|(_, player)| **player == PlayerReference::Player)
        else {
            return;
        };

        let Some(index) = player_hand.0.iter().position(|e| *e == card) else {
            return;
        };

        play.send(PlayCard {
            player: PlayerReference::Player,
            card,
            hand_index: index,
        });
    }
}

fn remove_played_card_from_hand(
    mut commands: Commands,
    mut hands: Query<(&mut Hand, &PlayerReference)>,
    mut play: EventReader<PlayCard>,
) {
    for event in play.read() {
        let Some((mut hand, _)) = hands
            .iter_mut()
            .find(|(_, player)| **player == event.player)
        else {
            continue;
        };

        hand.0.remove(event.hand_index);
        commands
            .entity(event.card)
            .remove::<(InHand, Focused, AllowedToPlay)>();
    }
}
