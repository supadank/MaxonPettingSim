use std::collections::HashMap;

use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, On, Pointer};
use bevy_persistent::{Persistent, StorageFormat};
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};
use serde::{Deserialize, Serialize};

use crate::{
    assets::AppAssets,
    constants::{APP_DEVELOPER, APP_NAME},
    startup_systems::CameraType,
};

#[derive(Resource, Serialize, Deserialize)]
pub struct PlayerData {
    pub money: f64,
    pub multiplier: f64,
    pub purchased_items: HashMap<String, i32>,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            money: 0.0,
            multiplier: 0.0,
            purchased_items: HashMap::new(),
        }
    }
}

pub fn init_player_data(mut commands: Commands) {
    let path = dirs::data_dir().unwrap().join(APP_DEVELOPER).join(APP_NAME);

    commands.insert_resource(
        Persistent::<PlayerData>::builder()
            .name("PlayerData")
            .format(StorageFormat::Json)
            .path(path.join("savegame.maxon"))
            .default(PlayerData::default())
            .build()
            .expect("Failed to build player data"),
    );
}

#[derive(Component)]
pub struct PlayerComponent;

pub fn generate_player(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    mut sprite_params: Sprite3dParams,
    camera_query: Query<(&mut Transform, &CameraType), With<CameraType>>,
) {
    if let Some((t, _)) = camera_query.iter().find(|x| x.1.eq(&CameraType::ThreeD)) {
        commands.spawn((
            AtlasSprite3d {
                atlas: app_assets.cat_maxon.clone(),
                index: 0,
                unlit: true,
                transform: Transform::from_xyz(-4.0, 2.2, -4.0)
                    .with_scale(Vec3::new(3.0, 3.0, 3.0))
                    .looking_at(t.translation, Vec3::Y),
                ..default()
            }
            .bundle(&mut sprite_params),
            PlayerComponent,
            On::<Pointer<Click>>::run(click_on_player),
        ));
    }
}

pub fn click_on_player(
    mut player_sprite: Query<&mut AtlasSprite3dComponent, With<PlayerComponent>>,
    mut player_data: ResMut<Persistent<PlayerData>>,
    app_assets: Res<AppAssets>,
    assets: Res<Assets<TextureAtlas>>,
) {
    if let Ok(mut s) = player_sprite.get_single_mut() {
        let sprite = assets.get(&app_assets.cat_maxon).unwrap();

        s.index += 1;

        if s.index >= sprite.textures.len() {
            s.index = 0;
        }

        player_data
            .update(|data| {
                data.money += 1.0;
            })
            .expect("Failed to update player data");
    }
}

#[derive(Resource)]
pub struct MultiplierTimer(Timer);

pub fn generate_multiplier_timer(mut commands: Commands) {
    commands.insert_resource(MultiplierTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
}

pub fn tick_multiplier_timer(
    delta_time: Res<Time>,
    mut player_data: ResMut<Persistent<PlayerData>>,
    mut timer: ResMut<MultiplierTimer>,
) {
    timer.0.tick(delta_time.delta());

    if timer.0.just_finished() {
        player_data.money += player_data.multiplier / 10.0;
    }
}