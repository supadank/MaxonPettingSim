use assets::AppAssets;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_asset_loader::prelude::*;

mod assets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Maxon Petting Simulator".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // App states
        .add_state::<AppState>()
        // Loading state
        .add_loading_state(LoadingState::new(AppState::Boot).continue_to_state(AppState::Menu))
        .add_collection_to_loading_state::<_, AppAssets>(AppState::Boot)
        // Diagnostics
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    Boot,
    Menu,
    Game,
    Pause,
}