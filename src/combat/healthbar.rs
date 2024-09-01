use crate::prelude::*;

pub struct HealthBarPlugin;

#[derive(Component)]
struct HealthBar(Entity);

#[derive(Component)]
struct BarChild(Entity);

#[derive(Bundle)]
pub struct HealthBarBundle {
    health_bar: HealthBar,
    text: TextBundle,
    name: Name,
}

#[derive(Component)]
struct BarMarker;

impl HealthBarBundle {
    pub fn new(entity: Entity, on_top: bool, fonts: &assets::Fonts) -> Self {
        let mut style = Style {
            position_type: PositionType::Absolute,
            width: Val::Vw(100.0),
            ..default()
        };
        if !on_top {
            style.bottom = Val::Px(0.0);
        }
        Self {
            name: Name::new("Healthbar"),
            health_bar: HealthBar(entity),
            text: TextBundle {
                text: Text::from_section(
                    "HELLO WORLD",
                    TextStyle {
                        font_size: 30.0,
                        font: fonts.pixel.clone_weak(),
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                style,
                ..default()
            },
        }
    }
}

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_healthbar,
                spawn_healthbar_children.run_if(not(in_state(MainState::Loading))),
            ),
        );
    }
}

fn update_healthbar(
    mut health_bar: Query<(&HealthBar, &BarChild, &mut Text)>,
    targets: Query<&super::Hp>,
    mut bars: Query<&mut Style, With<BarMarker>>,
) {
    for (target, bar, mut text) in &mut health_bar {
        let Ok(hp) = targets.get(target.0) else {
            continue;
        };
        let new_text = format!("{} / {}", hp.current_hp, hp.max_hp);
        text.sections[0].value = new_text;

        let Ok(mut bar_style) = bars.get_mut(bar.0) else {
            continue;
        };

        let frac = f32::from(hp.current_hp) / f32::from(hp.max_hp);
        bar_style.width = Val::Percent(frac * 100.0);
    }
}

fn spawn_healthbar_children(
    mut commands: Commands,
    new_bars: Query<Entity, Added<HealthBar>>,
    assets: Res<assets::HealthBar>,
) {
    for entity in &new_bars {
        let filled_bar = commands
            .spawn((
                ImageBundle {
                    image: UiImage::new(assets.filled.clone_weak()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        ..default()
                    },
                    ..default()
                },
                ImageScaleMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0,
                },
                BarMarker,
            ))
            .id();
        let empty_bar = commands
            .spawn((
                ImageBundle {
                    image: UiImage::new(assets.empty.clone_weak()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(-1),
                    ..default()
                },
                ImageScaleMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0,
                },
            ))
            .add_child(filled_bar)
            .id();
        commands
            .entity(entity)
            .add_child(empty_bar)
            .insert(BarChild(filled_bar));
    }
}
