use crate::RequiredAssets;
use game_lib::{
    bevy::{
        asset::{Asset, AssetPath, LoadState},
        ecs::{self as bevy_ecs, schedule::ShouldRun, system::SystemParam},
        prelude::*,
    },
    tracing::{self, instrument},
};

#[derive(SystemParam)]
pub struct RequiredAssetLoader<'a> {
    pub asset_server: Res<'a, AssetServer>,
    pub required_assets: ResMut<'a, RequiredAssets>,
}

impl<'a> RequiredAssetLoader<'a> {
    pub fn load_required<'b, T, P>(&mut self, path: P) -> Handle<T>
    where
        T: Asset,
        P: Into<AssetPath<'a>>,
    {
        let handle = self.asset_server.load(path);
        self.required_assets
            .loading_assets
            .push(handle.clone_untyped());
        handle
    }
}

#[instrument(skip(commands))]
pub fn setup_main_loading(mut commands: Commands) {
    commands.insert_resource(RequiredAssets::default());
}

#[instrument(skip(commands))]
pub fn cleanup_main_loading(mut commands: Commands) {
    commands.remove_resource::<RequiredAssets>();
}

#[instrument(skip(required_assets, asset_server))]
pub fn if_required_assets_loaded(
    required_assets: Res<RequiredAssets>,
    asset_server: Res<AssetServer>,
) -> ShouldRun {
    let load_state = asset_server.get_group_load_state(
        required_assets
            .loading_assets
            .iter()
            .map(|handle| handle.id),
    );

    match load_state {
        LoadState::Loading => {
            info!("loading assets...");
            ShouldRun::No
        }
        LoadState::Loaded => ShouldRun::Yes,
        LoadState::Failed => {
            // TODO: properly show an error screen
            panic!("assets failed to load");
        }
        LoadState::NotLoaded => {
            // TODO: properly show an error screen
            panic!("some assets aren't loaded");
        }
    }
}
