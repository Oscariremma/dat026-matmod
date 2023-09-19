mod components;
mod physics;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;
use components::*;
use physics::*;

struct StartingParameter {
    velocity: Vec2,
    position: Vec2,
    color: Color,
    size: f32,
}

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const BALLS_Z_INDEX: f32 = 1.0;

fn get_initial_balls() -> Vec<StartingParameter> {
    vec![
        StartingParameter {
            velocity: Vec2::new(0.0, -200.0),
            position: Vec2::new(50.0, 100.0),
            color: Color::DARK_GREEN,
            size: 100.0,
        },
        StartingParameter {
            velocity: Vec2::new(-500.0, 100.0),
            position: Vec2::new(-50.0, -200.0),
            color: Color::TURQUOISE,
            size: 100.0,
        },
        StartingParameter {
            velocity: Vec2::new(500.0, 100.0),
            position: Vec2::new(-300.0, 300.0),
            color: Color::BLUE,
            size: 100.0,
        },
        StartingParameter {
            velocity: Vec2::new(-500.0, 600.0),
            position: Vec2::new(-90.0, -100.0),
            color: Color::BLACK,
            size: 100.0,
        },
    ]
}

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
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        .add_systems(
            FixedUpdate,
            (
                handle_inter_ball_collision.before(handle_for_edge_collisions),
                handle_for_edge_collisions,
                apply_velocity.after(handle_for_edge_collisions),
            ),
        )
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let initial_balls = get_initial_balls();

    for ball in initial_balls {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(ball.color)),
                transform: Transform::from_translation(Vec3 {
                    x: ball.position.x,
                    y: ball.position.y,
                    z: BALLS_Z_INDEX,
                })
                .with_scale(Vec3::splat(ball.size)),
                ..default()
            },
            Ball,
            Velocity(ball.velocity),
        ));
    }
}
