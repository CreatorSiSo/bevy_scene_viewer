use bevy::{log::LogSettings, prelude::*};

#[cfg(feature = "debug")]
use bevy::log::Level;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LogSettings {
        #[cfg(feature = "debug")]
        level: Level::INFO,
        ..default()
      })
      .insert_resource(WindowDescriptor {
        title: String::from("Bevy Scene Viewer"),
        height: 720.0,
        width: 1200.0,
        ..default()
      })
      .add_startup_system(|asset_server: Res<AssetServer>| {
        asset_server.watch_for_changes().unwrap();
      });
  }
}
