use super::hand::{Hand, PlayCard};
use super::{AllowedToPlay, PlayerReference, TurnState, WhosTurnIsIt};
use crate::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TurnState::DrawCard),
            do_very_smart_ai_thing.run_if(in_state(WhosTurnIsIt(PlayerReference::Ai))),
        );
    }
}

fn do_very_smart_ai_thing(
    hands: Query<(&Hand, &PlayerReference)>,
    cards: Query<(), With<AllowedToPlay>>,
    mut play: EventWriter<PlayCard>,
) {
    let Some((hand, _)) = hands
        .iter()
        .find(|(_, player)| **player == PlayerReference::Ai)
    else {
        return;
    };

    for (index, card) in hand.0.iter().enumerate() {
        // We are allowed to play the card
        if cards.contains(*card) {
            play.send(PlayCard {
                player: PlayerReference::Ai,
                card: *card,
                hand_index: index,
            });
            return;
        }
    }
}
