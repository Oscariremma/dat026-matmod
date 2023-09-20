use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::window::{PrimaryWindow, Window};
use rand::{Rng, RngCore};
use crate::BALLS_Z_INDEX;
use crate::components::*;

pub fn handle_left_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {

    let (camera, camera_transform) = camera_q.single();

    let mut pos = Vec2::new(0.0, 0.0);
    if let Some(world_position) = q_windows.single().cursor_position()
    	        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    	        .map(|ray| ray.origin.truncate())
    {
        pos = world_position;
    }


    let mut rng = rand::thread_rng();

    let color = Color::rgb(
        rng.next_u32() as f32 / u32::MAX as f32,
        rng.next_u32() as f32 / u32::MAX as f32,
        rng.next_u32() as f32 / u32::MAX as f32,
    );

    let size = rng.gen_range(10.0..200.0);
    let mass = size / 10.0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(pos.extend(BALLS_Z_INDEX))
                .with_scale(Vec3::splat(size)),
            ..default()
        },
        Ball { mass },
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

pub fn handle_drag() {

}

