mod loading;
mod game;
mod mouse;
mod input;
mod camera;
mod animation;
mod animator;
mod vision_cone;
mod post_processing;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
