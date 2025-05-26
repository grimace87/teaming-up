mod camera;
mod chunks;
mod lighting;
mod loading;
mod markers;
mod splash;
mod state;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins((
            DefaultPlugins,
            state::StatePlugin,
            camera::GameCameraPlugin,
            chunks::ChunksPlugin,
            lighting::LightingPlugin,
            loading::LoadingPlugin,
            splash::SplashPlugin
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            ..default()
        })
        .run();
}


