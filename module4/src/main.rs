use bevy::window::{PrimaryWindow, WindowResized};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

struct StartingParameter {
    velocity: Vec2,
    position: Vec2,
    color: Color,
    size: f32,
}

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const BALLS_Z_INDEX: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        .add_systems(
            FixedUpdate,
            (
                check_for_collisions,
                apply_velocity.before(check_for_collisions),
            ),
        )
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let initial_balls = vec![
        StartingParameter {
            velocity: Vec2::new(40.0, 40.0),
            position: Vec2::new(20.0, -100.0),
            color: Color::DARK_GREEN,
            size: 100.0,
        },
        StartingParameter {
            velocity: Vec2::new(0.0, 8.0),
            position: Vec2::new(0.0, 0.0),
            color: Color::TURQUOISE,
            size: 70.0,
        },
    ];

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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut balls_query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let window = window_query.get_single().expect("Window should exist.");

    let window_left = -window.width() / 2.0;
    let window_right = window.width() / 2.0;
    let window_top = window.height() / 2.0;
    let window_bottom = -window.height() / 2.0;

    for (mut transform, mut velocity, _) in &mut balls_query {
        let x_change = transform.translation.x + velocity.x * time_step.period.as_secs_f32();
        let new_x = if x_change > 0.0 {
            x_change + transform.scale.x / 2.0
        } else {
            x_change - transform.scale.x / 2.0
        };

        let y_change = transform.translation.y + velocity.y * time_step.period.as_secs_f32();
        let new_y = if y_change > 0.0 {
            y_change + transform.scale.y / 2.0
        } else {
            y_change - transform.scale.y / 2.0
        };

        if new_x > window_right || new_x < window_left {
            velocity.x *= -1.0;
        }

        if new_y > window_top || new_y < window_bottom {
            velocity.y *= -1.0;
        }
    }
}

fn window_resized_event(
    mut events: EventReader<WindowResized>,
    mut cammera_query: Query<&mut Transform, &MainCamera>,
) {
    for event in events.iter() {
        // resize window
        cammera_query.single_mut().translation =
            Vec3::new(event.width / 2.0, event.height / 2.0, 0.0);
    }
}
