use std::collections::HashMap;

use bevy::prelude::{Commands, Resource};
use bevy_persistent::{Persistent, StorageFormat};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{APP_DEVELOPER, APP_NAME},
    game::building::Building,
};

#[derive(Resource, Serialize, Deserialize)]
pub struct Settings {
    pub is_fullscreen: bool,
    pub music: bool,
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            is_fullscreen: true,
            music: true,
            language: "english".to_string(),
        }
    }
}

pub fn init_settings(mut commands: Commands) {
    let path = dirs::preference_dir()
        .unwrap()
        .join(APP_DEVELOPER)
        .join(APP_NAME);

    commands.insert_resource(
        Persistent::<Settings>::builder()
            .name("Settings")
            .format(StorageFormat::Json)
            .path(path.join("settings.maxon"))
            .default(Settings::default())
            .build()
            .expect("Failed to initialize settings"),
    );
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct Savegame {
    pub points: usize,
    pub slaves: HashMap<Building, usize>,
}

pub fn init_savegame(mut commands: Commands) {
    let path = dirs::data_dir().unwrap().join(APP_DEVELOPER).join(APP_NAME);

    commands.insert_resource(
        Persistent::<Savegame>::builder()
            .name("Savegame")
            .format(StorageFormat::Json)
            .path(path.join("savegame.maxon"))
            .default(Savegame::default())
            .build()
            .expect("Failed to initialize savegame"),
    );
}