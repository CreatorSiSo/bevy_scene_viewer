use std::path::PathBuf;

use bevy::{asset::LoadState, prelude::*};

fn main() {
  App::new()
    .add_event::<LoadSceneEvent>()
    .insert_resource(WindowDescriptor {
      title: String::from("Bevy Scene Viewer"),
      ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_system(file_drop)
    .add_system(load_scene)
    .add_system(update_scene_handle)
    .run();
}

struct SceneHandle {
  handle: Handle<Scene>,
  load_state: LoadState,
}

impl Default for SceneHandle {
  fn default() -> Self {
    Self {
      handle: Default::default(),
      load_state: LoadState::NotLoaded,
    }
  }
}

struct LoadSceneEvent(PathBuf);

fn file_drop(
  mut events: EventReader<FileDragAndDrop>,
  mut load_scene_event: EventWriter<LoadSceneEvent>,
) {
  for event in events.iter().last() {
    if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
      load_scene_event.send(LoadSceneEvent(path_buf.to_owned()));

      info!("Dropped: {:?}", path_buf);
    }
  }
}

fn load_scene(
  mut commands: Commands,
  mut events: EventReader<LoadSceneEvent>,
  asset_server: Res<AssetServer>,
) {
  for event in events.iter().last() {
    let LoadSceneEvent(path_buf) = event;

    commands.insert_resource(SceneHandle {
      handle: asset_server.load(path_buf.to_owned()),
      ..default()
    });
  }
}

fn update_scene_handle(
  maybe_scene_handle: Option<ResMut<SceneHandle>>,
  asset_server: Res<AssetServer>,
) {
  if let Some(mut scene_handle) = maybe_scene_handle {
    let load_state = AssetServer::get_load_state(&asset_server, scene_handle.handle.to_owned());

    if load_state != scene_handle.load_state {
      info!("{:?}", load_state);
      scene_handle.load_state = load_state;
    }
  }
}
