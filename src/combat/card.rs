use bevy::color::palettes::css::{BLACK, GRAY, WHITE};
use bevy::color::palettes::tailwind::{GRAY_400, GRAY_700, GRAY_950};
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;

use super::deck::GlobalCards;
use super::hovering::Hoverable;
use super::{Costs, Hp, Power};
use crate::prelude::*;

#[derive(Clone)]
pub struct CardGameplayInfo {
    pub cast_crowd: u8,
    pub minimum_crowd: u8,
    pub hp: u8,
    pub power: u8,
}

#[derive(Clone)]
pub struct CardInfo {
    pub gameplay: CardGameplayInfo,
    pub name: &'static str,
    pub img: Handle<Image>,
    pub flavor_text: Option<&'static str>,
}

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct ShowFront(pub bool);

#[derive(Component)]
struct Front;

#[derive(Component)]
struct Back;

#[derive(Component)]
struct CardStatsText;

#[derive(Component)]
struct CardCostsText;

#[derive(Component)]
struct CardArt;

#[derive(Component)]
pub struct CardGray;

#[derive(Component)]
pub struct Deck(pub Vec<Entity>);

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(MainState::Loading), create_global_cards);

        app.add_systems(
            Update,
            (
                update_card_crowd,
                update_card_stats,
                update_shown_side,
                set_grayscale,
            ),
        );
    }
}

fn create_global_cards(mut commands: Commands, assets: Res<assets::Cards>) {
    commands.insert_resource(GlobalCards::new(&assets));
}

#[allow(clippy::too_many_lines)] // Theres a lot of stuff to spawn
pub fn spawn_card(
    commands: &mut Commands,
    card_assets: &assets::Cards,
    font_assets: &assets::Fonts,
    card: &CardInfo,
    trans: Transform,
) -> Entity {
    commands
        .spawn((
            Name::from(card.name),
            Card,
            ShowFront(false),
            VisibilityBundle::default(),
            TransformBundle::from_transform(trans),
            Hp {
                current_hp: card.gameplay.hp,
                max_hp: card.gameplay.hp,
            },
            Power(card.gameplay.power),
            Costs {
                minimum: card.gameplay.minimum_crowd,
                cast: card.gameplay.cast_crowd,
            },
            Hoverable {
                size: Vec2::new(52.0 * 5.0, 84.0 * 5.0),
            },
        ))
        .with_children(|commands| {
            commands.spawn((
                Name::new("Back"),
                Back,
                SpriteBundle {
                    texture: card_assets.back.clone_weak(),
                    transform: Transform::from_scale(Vec3::new(5.0, 5.0, 1.0)),
                    ..default()
                },
            ));
            commands
                .spawn((
                    Name::new("Front"),
                    Front,
                    SpriteBundle {
                        texture: card_assets.base.clone_weak(),
                        transform: Transform::from_scale(Vec3::new(5.0, 5.0, 1.0)),
                        ..default()
                    },
                ))
                .with_children(|commands| {
                    commands.spawn((
                        Name::new("Art"),
                        CardArt,
                        SpriteBundle {
                            texture: card.img.clone_weak(),
                            transform: Transform::from_xyz(0.0, 25.0, 0.0),
                            ..default()
                        },
                    ));
                    commands.spawn((
                        Name::new("Name"),
                        Text2dBundle {
                            text: Text::from_section(
                                card.name,
                                TextStyle {
                                    font_size: 100.0,
                                    font: font_assets.pixel.clone_weak(),
                                    color: BLACK.into(),
                                },
                            ),
                            transform: Transform::from_xyz(0.0, 3.0, 1.0)
                                .with_scale(Vec3::new(0.1, 0.1, 1.0)),
                            ..default()
                        },
                    ));
                    commands.spawn((
                        Name::new("Stats"),
                        CardStatsText,
                        Text2dBundle {
                            text: Text::from_section(
                                "0/0",
                                TextStyle {
                                    font: font_assets.pixel.clone_weak(),
                                    font_size: 90.0,
                                    color: WHITE.into(),
                                },
                            ),
                            transform: Transform::from_xyz(19.0, -36.0, 1.0)
                                .with_scale(Vec3::new(0.1, 0.1, 1.0)),
                            ..default()
                        },
                    ));
                    commands.spawn((
                        Name::new("Crowd"),
                        CardCostsText,
                        Text2dBundle {
                            text: Text::from_section(
                                "0/0",
                                TextStyle {
                                    font: font_assets.pixel.clone_weak(),
                                    font_size: 90.0,
                                    color: WHITE.into(),
                                },
                            ),
                            transform: Transform::from_xyz(-19.0, -36.0, 1.0)
                                .with_scale(Vec3::new(0.1, 0.1, 1.0)),
                            ..default()
                        },
                    ));
                    if let Some(flavor) = card.flavor_text {
                        commands.spawn((
                            Name::new("Flavor"),
                            Text2dBundle {
                                text: Text::from_section(
                                    flavor,
                                    TextStyle {
                                        font_size: 50.0,
                                        font: font_assets.pixel.clone_weak(),
                                        color: GRAY_400.into(),
                                    },
                                )
                                .with_justify(JustifyText::Center),
                                text_anchor: Anchor::TopLeft,
                                text_2d_bounds: Text2dBounds {
                                    size: Vec2::new(560.0, 300.0),
                                },
                                transform: Transform::from_xyz(-28.0, -2.0, 1.0)
                                    .with_scale(Vec3::new(0.1, 0.1, 1.0)),
                                ..default()
                            },
                        ));
                    }
                });
        })
        .id()
}

fn update_card_stats(
    cards: Query<(&Hp, &Power, &Children), With<Card>>,
    fronts: Query<&Children, With<Front>>,
    mut text: Query<&mut Text, With<CardStatsText>>,
) {
    for (hp, power, children) in &cards {
        'search: for child in children {
            if let Ok(children) = fronts.get(*child) {
                for child in children {
                    if let Ok(mut text) = text.get_mut(*child) {
                        text.sections[0].value = format!("{}/{}", power.0, hp.current_hp);
                        break 'search;
                    }
                }
            }
        }
    }
}

fn update_card_crowd(
    cards: Query<(&Costs, &Children), With<Card>>,
    fronts: Query<&Children, With<Front>>,
    mut text: Query<&mut Text, With<CardCostsText>>,
) {
    for (costs, children) in &cards {
        'search: for child in children {
            if let Ok(children) = fronts.get(*child) {
                for child in children {
                    if let Ok(mut text) = text.get_mut(*child) {
                        text.sections[0].value = format!("{}/{}", costs.minimum, costs.cast);
                        break 'search;
                    }
                }
            }
        }
    }
}

fn update_shown_side(
    cards: Query<(&ShowFront, &Children), With<Card>>,
    // The negative queries are to make bevys conflict detector happy,
    // which is fair it is possible we have a entity with both front and back in theory
    // (but ofc that should never happen in practice)
    mut fronts: Query<&mut Visibility, (With<Front>, Without<Back>)>,
    mut backs: Query<&mut Visibility, (With<Back>, Without<Front>)>,
) {
    for (show_front, children) in &cards {
        for child in children {
            if let Ok(mut front) = fronts.get_mut(*child) {
                *front = if show_front.0 {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            if let Ok(mut back) = backs.get_mut(*child) {
                *back = if show_front.0 {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
            }
        }
    }
}

fn set_grayscale(
    cards: Query<(&Children, Option<&CardGray>), With<Card>>,
    fronts: Query<&Children, With<Front>>,
    arts: Query<(), With<CardArt>>,
    mut sprites: Query<&mut Sprite>,
) {
    for (children, gray) in &cards {
        let color = if gray.is_some() { GRAY_700 } else { WHITE };

        for front in children {
            if let Ok(children) = fronts.get(*front) {
                if let Ok(mut sprite) = sprites.get_mut(*front) {
                    sprite.color = color.into();
                }

                for art in children {
                    if arts.get(*art).is_ok() {
                        if let Ok(mut sprite) = sprites.get_mut(*art) {
                            sprite.color = color.into();
                        }
                    }
                }
            }
        }
    }
}
