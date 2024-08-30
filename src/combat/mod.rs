use crate::data::PlayerInfo;
use crate::prelude::*;

mod card;
mod healthbar;

pub struct CombatPlugin;

#[derive(Resource)]
pub struct OpponentInfo {
    pub hp: u8,
}

#[derive(Component)]
struct Controller;

#[derive(Component)]
struct Hp {
    max_hp: u8,
    current_hp: u8,
}

#[derive(Component)]
enum PlayerReference {
    Player,
    Ai,
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // TESTING OPPONENT
        app.insert_resource(OpponentInfo { hp: 20 });

        app.add_plugins(healthbar::HealthBarPlugin);

        app.add_systems(OnEnter(MainState::Combat), setup_combat);
    }
}

fn setup_combat(
    mut commands: Commands,
    opponent_info: Res<OpponentInfo>,
    player_info: Res<PlayerInfo>,
    fonts: Res<assets::Fonts>,
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
}
