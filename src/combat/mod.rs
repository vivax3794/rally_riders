use card::{spawn_card, CardInfo, Deck};
use deck::GlobalCards;
use hand::{DrawCard, Hand, InHand};

use crate::data::PlayerInfo;
use crate::position::{AxisAnchor, Relative, RelativeAxis};
use crate::prelude::*;

pub mod card;
mod deck;
mod hand;
mod healthbar;
mod hovering;

pub struct CombatPlugin;

#[derive(Resource)]
pub struct OpponentInfo {
    pub hp: u8,
    pub deck: Vec<CardInfo>,
}

#[derive(Component)]
struct Controller;

#[derive(Component)]
struct Hp {
    max_hp: u8,
    current_hp: u8,
}

#[derive(Component)]
struct Costs {
    cast: u8,
    minimum: u8,
}

#[derive(Component)]
struct Power(u8);

#[derive(Component)]
struct AllowedToPlay;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
enum PlayerReference {
    Player,
    Ai,
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            healthbar::HealthBarPlugin,
            card::CardPlugin,
            hand::HandPlugin,
            hovering::HoveringPlugin,
        ));

        app.add_systems(OnExit(MainState::TestingSetup), create_test_combat);
        app.add_systems(OnEnter(MainState::Combat), setup_combat);
        app.add_systems(Update, set_allowed_cards);
    }
}

fn create_test_combat(
    mut commands: Commands,
    mut player: ResMut<PlayerInfo>,
    cards: Res<GlobalCards>,
) {
    commands.insert_resource(OpponentInfo {
        hp: 20,
        deck: cards.0.clone(),
    });
    player.deck.clone_from(&cards.0);
}

fn spawn_deck(
    commands: &mut Commands,
    card_assets: &assets::Cards,
    font_assets: &assets::Fonts,
    deck: &[CardInfo],
    y_level: AxisAnchor,
) -> Deck {
    let mut entities = Vec::with_capacity(deck.len());
    for card in deck {
        let card = spawn_card(
            commands,
            card_assets,
            font_assets,
            card,
            Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
        );
        commands.entity(card).insert(Relative {
            x: Some(RelativeAxis {
                anchor: AxisAnchor::Neg,
                amount: 100.0,
            }),
            y: Some(RelativeAxis {
                anchor: y_level,
                amount: 150.0,
            }),
        });
        entities.push(card);
    }
    Deck(entities)
}

fn setup_combat(
    mut commands: Commands,
    opponent_info: Res<OpponentInfo>,
    player_info: Res<PlayerInfo>,
    fonts: Res<assets::Fonts>,
    card_assets: Res<assets::Cards>,
    mut draw_event: EventWriter<DrawCard>,
) {
    let player = commands
        .spawn((
            Name::new("Player"),
            Controller,
            PlayerReference::Player,
            Hp {
                max_hp: player_info.max_hp,
                current_hp: player_info.current_hp / 2,
            },
        ))
        .id();
    let ai = commands
        .spawn((
            Name::new("Ai"),
            Controller,
            PlayerReference::Ai,
            Hp {
                max_hp: opponent_info.hp,
                current_hp: opponent_info.hp / 5,
            },
        ))
        .id();

    commands.spawn(healthbar::HealthBarBundle::new(player, false, &fonts));
    commands.spawn(healthbar::HealthBarBundle::new(ai, true, &fonts));

    let player_deck = spawn_deck(
        &mut commands,
        &card_assets,
        &fonts,
        &player_info.deck,
        AxisAnchor::Neg,
    );
    commands.spawn((
        player_deck,
        Name::new("Player Deck"),
        PlayerReference::Player,
    ));

    let ai_deck = spawn_deck(
        &mut commands,
        &card_assets,
        &fonts,
        &opponent_info.deck,
        AxisAnchor::Pos,
    );
    commands.spawn((ai_deck, Name::new("Ai Deck"), PlayerReference::Ai));

    commands.spawn((
        Name::new("Player hand"),
        Hand(Vec::new()),
        PlayerReference::Player,
    ));
    commands.spawn((Name::new("Ai hand"), Hand(Vec::new()), PlayerReference::Ai));
    draw_event.send(DrawCard {
        player: PlayerReference::Player,
        amount: 7,
    });
    draw_event.send(DrawCard {
        player: PlayerReference::Ai,
        amount: 7,
    });
}

fn set_allowed_cards(mut commands: Commands, query: Query<(Entity, &Costs), With<InHand>>) {
    for (card, costs) in &query {
        if costs.cast < 1 {
            commands.entity(card).insert(AllowedToPlay);
        } else {
            commands.entity(card).remove::<AllowedToPlay>();
        }
    }
}
