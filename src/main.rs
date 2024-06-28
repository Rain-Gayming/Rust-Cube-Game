use bevy::prelude::*;
use bevy::window::ExitCondition;
//use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Voxel Engine".into(),
                    name: Some("Engine.app".into()),
                    resolution: (500., 300.).into(),
                    ..default()
                }),
                close_when_requested: true,
                exit_condition: ExitCondition::DontExit,
            })
        )
        .add_systems(Startup, setup)
        //.add_plugins(FpsCounterPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}