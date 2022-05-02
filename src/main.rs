use std::path::PathBuf;

use bevy::{
  asset::LoadState,
  log::{Level, LogSettings},
  prelude::*,
};

fn main() {
  App::new()
    .insert_resource(LogSettings {
      #[cfg(feature = "dev")]
      level: Level::DEBUG,
      ..default()
    })
    .insert_resource(WindowDescriptor {
      title: String::from("Bevy Scene Viewer"),
      height: 800.0,
      width: 1000.0,
      ..default()
    })
    .add_startup_system(config)
    .add_plugins(DefaultPlugins)
    .add_event::<LoadFileEvent>()
    .insert_resource(AssetsLoading(Vec::default()))
    .add_system(file_drop)
    .add_system(load_scene)
    .add_system(check_assets_loading)
    .run();
}

fn config(asset_server: Res<AssetServer>) {
  #[cfg(feature = "dev")]
  asset_server.watch_for_changes().unwrap();
}

struct LoadFileEvent(PathBuf);

#[derive(Debug)]
struct AssetsLoading(Vec<HandleUntyped>);

fn file_drop(
  mut events: EventReader<FileDragAndDrop>,
  mut load_scene_event: EventWriter<LoadFileEvent>,
) {
  for event in events.iter().last() {
    if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
      load_scene_event.send(LoadFileEvent(path_buf.to_owned()));

      debug!("Dropped: {:?}", path_buf);
    }
  }
}

fn load_scene(
  mut events: EventReader<LoadFileEvent>,
  mut assets_loading: ResMut<AssetsLoading>,
  asset_server: Res<AssetServer>,
) {
  for event in events.iter().last() {
    let LoadFileEvent(path_buf) = event;

    let scene_handle: Handle<Scene> = asset_server.load(path_buf.to_owned());

    assets_loading.0.push(scene_handle.clone_untyped());
  }
}

fn check_assets_loading(mut assets_loading: ResMut<AssetsLoading>, asset_server: Res<AssetServer>) {
  for (index, handle) in assets_loading.0.clone().iter().enumerate() {
    match AssetServer::get_load_state(&asset_server, handle) {
      // TODO: Make the logging more generic (warn + info logging together)
      LoadState::Loaded => {
        info!(
          "{:?}: {:?}",
          LoadState::Loaded,
          AssetServer::get_handle_path(&asset_server, handle)
            .unwrap_or("".into())
            .path()
        );
        assets_loading.0.remove(index);
      }
      LoadState::Failed => {
        warn!(
          "{:?}: {:?}",
          LoadState::Failed,
          AssetServer::get_handle_path(&asset_server, handle)
            .unwrap_or("".into())
            .path()
        );
        assets_loading.0.remove(index);
      }
      _ => (),
      // Other
      // LoadState::Loading => (),
      // LoadState::Unloaded => (),
      // LoadState::NotLoaded => (),
    }
  }
}
