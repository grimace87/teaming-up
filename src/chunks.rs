use bevy::prelude::*;
use crate::state::AppState;

pub const GRID_WIDTH: usize = 4;
pub const GRID_DEPTH: usize = 4;
pub const CHUNK_SIZE: f32 = 2.0;
pub const SURFACE_Y: f32 = 0.0;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_scene);
    }
}

fn spawn_scene(
    mut commands: Commands,
    ass: Res<AssetServer>
) {
    let grid_start_x = -0.5 * CHUNK_SIZE * GRID_WIDTH as f32;
    let grid_start_z = -0.5 * CHUNK_SIZE * GRID_DEPTH as f32;
    let material_handle = ass.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.3),
        ..default()
    });
    let mesh_handle = ass.add(Cuboid::from_length(CHUNK_SIZE).into());
    for i in 0..GRID_WIDTH {
        for j in 0..GRID_DEPTH {
            let x = grid_start_x + (i as f32 + 0.5) * CHUNK_SIZE;
            let y = SURFACE_Y - 0.5 * CHUNK_SIZE;
            let z = grid_start_z + (j as f32 + 0.5) * CHUNK_SIZE;
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(x, y, z)
            ));
        }
    }
}

