use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Game)
                .load_collection::<MainAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct MainAssets {
    #[asset(path = "ColorGrid.png")]
    #[asset(image(sampler(filter = nearest, wrap = repeat)))]
    pub color_grid: Handle<Image>,
}
