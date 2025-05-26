
use bevy::prelude::*;
use crate::state::AppState;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_lights);
    }
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(0.0, 20.0, 0.0)
    ));
}

