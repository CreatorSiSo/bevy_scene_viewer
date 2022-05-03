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
      height: 720.0,
      width: 1200.0,
      ..default()
    })
    .insert_resource(AssetsLoading(Vec::default()))
    .add_startup_system(config)
    .add_startup_system(setup_graphics)
    .add_plugins(DefaultPlugins)
    .add_event::<LoadFileEvent>()
    .add_system(files_drag_drop)
    .add_system(load_files)
    .add_system(check_assets_loading)
    .run();
}

fn config(asset_server: Res<AssetServer>) {
  #[cfg(feature = "dev")]
  asset_server.watch_for_changes().unwrap();
}

fn setup_graphics(mut commands: Commands) {
  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

struct LoadFileEvent(PathBuf);

#[derive(Debug)]
struct AssetsLoading(Vec<HandleUntyped>);

fn files_drag_drop(
  mut events: EventReader<FileDragAndDrop>,
  mut load_scene_event: EventWriter<LoadFileEvent>,
) {
  for event in events.iter() {
    if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
      load_scene_event.send(LoadFileEvent(path_buf.to_owned()));

      debug!("Dropped: {:?}", path_buf);
    }
  }
}

fn load_files(
  mut commands: Commands,
  mut load_file_events: EventReader<LoadFileEvent>,
  mut assets_loading: ResMut<AssetsLoading>,
  asset_server: Res<AssetServer>,
) {
  for load_file_event in load_file_events.iter() {
    let LoadFileEvent(path_buf) = load_file_event;

    if let Some(extension) = path_buf.extension() {
      let mut handle = Handle::default();

      match extension.to_str() {
        Some("gltf" | "glb") => {
          // TODO: Move this into some general asset loading structure, choose the correct sub-scene (here 0)
          // TODO: and get rid of this string manipulation mess
          let mut path = path_buf.to_str().unwrap_or_default().to_string();
          path.extend(["#Scene0"]);
          handle = asset_server.load(&path);
          commands.spawn_scene(handle.to_owned());
        }
        Some("png" | "jpg") => info!("Insert Image"),
        _ => (),
      }

      assets_loading.0.push(handle.clone_untyped());
    }
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
