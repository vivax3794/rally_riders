use battlefield::BattleField;
use bevy::color::palettes::tailwind::BLUE_300;
use card::{spawn_card, CardInfo, Deck};
use deck::GlobalCards;
use hand::{DrawCard, Hand, InHand, PlayCard};

use crate::data::PlayerInfo;
use crate::position::{AxisAnchor, Relative, RelativeAxis};
use crate::prelude::*;

mod ai;
mod battlefield;
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
struct Crowd(u8);

#[derive(Component)]
struct CrowdText(Entity);

#[derive(Component)]
struct AllowedToPlay;

#[derive(Component, PartialEq, Eq, Clone, Copy, Default, Hash, Debug)]
enum PlayerReference {
    #[default]
    Player,
    Ai,
}

#[derive(SubStates, Default, Clone, Hash, Eq, PartialEq, Debug)]
#[source(MainState = MainState::Combat)]
pub enum TurnState {
    #[default]
    DrawCard,
    PlayCreature,
    EndOfTurn,
}

#[derive(SubStates, Default, Clone, Hash, Eq, PartialEq, Debug)]
#[source(MainState = MainState::Combat)]
pub struct WhosTurnIsIt(PlayerReference);

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            healthbar::HealthBarPlugin,
            card::CardPlugin,
            hand::HandPlugin,
            hovering::HoveringPlugin,
            battlefield::BattlePlugin,
            ai::AiPlugin,
        ));

        app.add_sub_state::<TurnState>();
        app.add_sub_state::<WhosTurnIsIt>();

        app.add_systems(OnExit(MainState::TestingSetup), create_test_combat);
        app.add_systems(OnEnter(MainState::Combat), setup_combat);
        app.add_systems(Update, (set_allowed_cards, update_crowd_text));

        app.add_systems(OnEnter(TurnState::DrawCard), do_draw_card);
        app.add_systems(
            Update,
            progress_turn_after_play.run_if(in_state(TurnState::PlayCreature)),
        );
        app.add_systems(OnEnter(TurnState::EndOfTurn), do_end_of_turn);
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
            Crowd(0),
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
            Crowd(0),
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

    commands.spawn((
        BattleField(Vec::new()),
        Name::new("Battle Player"),
        PlayerReference::Player,
    ));
    commands.spawn((
        BattleField(Vec::new()),
        Name::new("Battle Ai"),
        PlayerReference::Ai,
    ));

    spawn_crowd_text(&mut commands, player, &fonts, false);
    spawn_crowd_text(&mut commands, ai, &fonts, true);
}

fn spawn_crowd_text(
    commands: &mut Commands,
    player: Entity,
    fonts: &Res<assets::Fonts>,
    top: bool,
) {
    let mut style = Style {
        position_type: PositionType::Absolute,
        right: Val::Px(20.0),
        ..default()
    };
    if top {
        style.top = Val::Px(20.0);
    } else {
        style.bottom = Val::Px(20.0);
    }
    commands.spawn((
        CrowdText(player),
        TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: fonts.pixel.clone_weak(),
                    font_size: 200.0,
                    color: BLUE_300.into(),
                },
            ),
            style,
            ..default()
        },
    ));
}

fn set_allowed_cards(
    mut commands: Commands,
    query: Query<(Entity, &Costs, &PlayerReference), With<InHand>>,
    players: Query<(&Crowd, &PlayerReference)>,
) {
    for (card, costs, player) in &query {
        let Some((crowd, _)) = players.iter().find(|(_, p)| *p == player) else {
            continue;
        };

        if costs.cast <= crowd.0 {
            commands.entity(card).insert(AllowedToPlay);
        } else {
            commands.entity(card).remove::<AllowedToPlay>();
        }
    }
}

fn update_crowd_text(mut texts: Query<(&CrowdText, &mut Text)>, query: Query<&Crowd>) {
    for (reference, mut text) in &mut texts {
        let Ok(crowd) = query.get(reference.0) else {
            continue;
        };
        text.sections[0].value = crowd.0.to_string();
    }
}

fn do_draw_card(
    mut draw_event: EventWriter<DrawCard>,
    player: Res<State<WhosTurnIsIt>>,
    mut turn: ResMut<NextState<TurnState>>,
) {
    draw_event.send(DrawCard {
        player: player.0,
        amount: 1,
    });
    turn.set(TurnState::PlayCreature);
}

fn progress_turn_after_play(
    mut play: EventReader<PlayCard>,
    mut turn: ResMut<NextState<TurnState>>,
) {
    if !play.is_empty() {
        turn.set(TurnState::EndOfTurn);
        play.clear();
    }
}

fn do_end_of_turn(
    mut turn: ResMut<NextState<TurnState>>,
    mut new_player: ResMut<NextState<WhosTurnIsIt>>,
    current_player: Res<State<WhosTurnIsIt>>,
) {
    turn.set(TurnState::DrawCard);
    new_player.set(WhosTurnIsIt(
        if current_player.0 == PlayerReference::Player {
            PlayerReference::Ai
        } else {
            PlayerReference::Player
        },
    ));
}
