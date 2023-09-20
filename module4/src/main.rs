mod components;
mod physics;
mod inputs;

use bevy::{prelude::*};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::common_conditions::*;
use bevy_prototype_debug_lines::*;
use components::*;
use physics::*;
use inputs::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);

const BALLS_Z_INDEX: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Balls".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(DebugLinesPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 10000.0))
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        .add_systems(
            FixedUpdate,
            (
                handle_left_click
                    .run_if(input_just_pressed(MouseButton::Left)),
                handle_inter_ball_collision.before(handle_for_edge_collisions),
                handle_for_edge_collisions.before(apply_velocity),
                apply_gravity.before(apply_velocity),
                apply_velocity,
            ),
        )
        .insert_resource(Gravity(1000.0))
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
) {
    // Camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        MainCamera,
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}
