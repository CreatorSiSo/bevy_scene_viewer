use bevy::{asset::LoadState, prelude::*};

use std::path::PathBuf;

pub(crate) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(AssetsLoading(Vec::default()))
      .add_event::<LoadFileEvent>()
      .add_system(files_drag_drop)
      .add_system(load_files)
      .add_system(check_assets_loading);
  }
}

pub(crate) struct LoadFileEvent(PathBuf);

#[derive(Debug)]
pub(crate) struct AssetsLoading(pub Vec<HandleUntyped>);

pub(crate) fn files_drag_drop(
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

pub(crate) fn load_files(
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

pub(crate) fn check_assets_loading(
  mut assets_loading: ResMut<AssetsLoading>,
  asset_server: Res<AssetServer>,
) {
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

        // TODO: Find out why this crashes when loading a lot of heavy gltf files with:
        // panicked at 'removal index (is 4) should be < len (is 3)',
        // or
        // panicked at 'removal index (is 4) should be < len (is 4)',
        if let Some(_) = assets_loading.0.get(index) {
          assets_loading.0.remove(index);
        }
      }
      LoadState::Failed => {
        warn!(
          "{:?}: {:?}",
          LoadState::Failed,
          AssetServer::get_handle_path(&asset_server, handle)
            .unwrap_or("".into())
            .path()
        );

        // TODO: See the end of the LoadState::Loaded match branch
        if let Some(_) = assets_loading.0.get(index) {
          assets_loading.0.remove(index);
        }
      }
      _ => (),
      // Other
      // LoadState::Loading => (),
      // LoadState::Unloaded => (),
      // LoadState::NotLoaded => (),
    }
  }
}
