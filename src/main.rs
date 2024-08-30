#![warn(
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::if_then_some_else_none,
    clippy::missing_const_for_fn,
    clippy::mixed_read_write_in_expression,
    clippy::panic,
    clippy::partial_pub_fields,
    clippy::same_name_method,
    clippy::str_to_string,
    clippy::suspicious_xor_used_as_pow,
    clippy::try_err,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::expect_used
)]
#![deny(
    clippy::unwrap_used,
    clippy::unreachable,
    clippy::unimplemented,
    clippy::todo,
    clippy::dbg_macro,
    clippy::error_impl_error,
    clippy::exit,
    clippy::panic_in_result_fn,
    clippy::tests_outside_test_module
)]
#![allow(
    clippy::type_complexity,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

mod assets;
mod combat;
mod data;

#[allow(unused_imports)]
mod prelude {
    pub(crate) use bevy::prelude::*;

    pub(crate) use super::{assets, MainState};
}
use prelude::*;

#[derive(States, Default, Clone, Hash, Eq, PartialEq, Debug)]
pub enum MainState {
    #[default]
    Loading,
    Combat,
}

fn main() {
    let mut app = App::new();
    #[cfg(feature = "release")]
    app.add_plugins(bevy_embedded_assets::EmbeddedAssetPlugin {
        mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
    });

    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "dev")]
    {
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }

    app.init_state::<MainState>();

    app.insert_resource(data::PlayerInfo {
        max_hp: 20,
        current_hp: 20,
    });
    app.add_plugins((assets::AssetPlugin, combat::CombatPlugin));

    // app.add_systems(Update, ());
    app.add_systems(Startup, (setup_camera,));

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
