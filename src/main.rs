use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins((
            DefaultPlugins
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            ..default()
        })
        .run();
}
