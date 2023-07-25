mod loading;
mod game;
mod mouse;
mod input;
mod camera;
mod animation;
mod animator;
mod field_of_view;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
