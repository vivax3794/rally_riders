use bevy_asset_loader::prelude::*;

use crate::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(crate::MainState::Loading)
                .continue_to_state(crate::MainState::TestingSetup)
                .load_collection::<Fonts>()
                .load_collection::<Cards>()
                .load_collection::<HealthBar>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "Fonts/pixel.ttf")]
    pub pixel: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct HealthBar {
    #[asset(path = "Healthbar/filled.png")]
    pub filled: Handle<Image>,
    #[asset(path = "Healthbar/empty.png")]
    pub empty: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Cards {
    #[asset(path = "Cards/base.png")]
    pub base: Handle<Image>,
    #[asset(path = "Cards/back.png")]
    pub back: Handle<Image>,
    #[asset(path = "Cards/placeholder.png")]
    pub placeholder: Handle<Image>,
    #[asset(path = "Cards/ghost.png")]
    pub ghost: Handle<Image>,
}
