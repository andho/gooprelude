mod loading;
mod game;
mod mouse;
mod input;
mod camera;
mod animation;
mod animator;
mod field_of_view;
mod scene;

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, window::{PresentMode, WindowTheme}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "GoO prelude".into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            GamePlugin,
        ))
        .add_plugins((
            WorldInspectorPlugin::new(),
            //FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .run();
}
