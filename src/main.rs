use bevy::prelude::*;

mod assets;
mod config;

fn main() {
  App::new()
    .add_plugin(config::ConfigPlugin)
    .add_plugins(DefaultPlugins)
    .add_plugin(assets::AssetsPlugin)
    .add_startup_system(setup_graphics)
    .run();
}

fn setup_graphics(mut commands: Commands) {
  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}
