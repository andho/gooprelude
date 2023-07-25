use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::ProgressPlugin;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "character/character-sheet.png")]
    pub player_spritesheet: Handle<Image>,
    #[asset(path = "background/campsite-improved.png")]
    pub background_texture: Handle<Image>,
    #[asset(path = "textures/stationary/tent.png")]
    pub tent_texture: Handle<Image>,
    #[asset(path = "textures/stationary/sitting-log.png")]
    pub sitting_log_texture: Handle<Image>,
}

#[derive(Default)]
pub struct LoadingPlugin<State> {
    loading: State,
    next: State,
}

impl<State> LoadingPlugin<State> {

    pub fn new(loading: State, next: State) -> LoadingPlugin<State> {
        LoadingPlugin {
            loading,
            next,
        }
    }

}

impl<State: Sync + Send + States> Plugin for LoadingPlugin<State> {
    fn build(&self, app: &mut App) {
        app
            .init_collection::<GameAssets>()
            .insert_resource(ProgressTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_loading_state(LoadingState::new(self.loading.clone()))
            .add_collection_to_loading_state::<_, GameAssets>(self.loading.clone())
            .add_plugins(ProgressPlugin::new(self.loading.clone()).continue_to(self.next.clone()))
            .add_systems(OnEnter(self.loading.clone()), loading_screen_setup)
            .add_systems(OnExit(self.loading.clone()), cleanup_loading)
            .add_systems(Update, print_progress.run_if(in_state(self.loading.clone())));
    }
}

#[derive(Resource)]
struct ProgressTimer(Timer);

#[derive(Default)]
struct ProgressState {
    tick: usize,
}

fn print_progress(
    time: Res<Time>,
    mut timer: ResMut<ProgressTimer>,
    mut query: Query<&mut Text>,
    mut state: Local<ProgressState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let length = state.tick % 3;
        let dots = ".".repeat(length + 1);
        let mut text = query.single_mut();
        text.sections[1].value = dots;

        state.tick += 1;
    }
}
fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<Node>>, cam_query: Query<Entity, With<Camera2d>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &cam_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn loading_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Loading".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/Ubuntu.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "...".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/Ubuntu.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });
        });
}
