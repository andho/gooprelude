mod loading;
mod game;
mod mouse;
mod input;
mod camera;
mod animation;
mod animator;
mod vision_cone;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .run();
}
