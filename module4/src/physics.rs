use crate::components::*;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;

pub fn handle_for_edge_collisions(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut balls_query: Query<(&Transform, &mut Velocity, &mut Gravity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let window = window_query.get_single().expect("Window should exist.");

    let window_left = -window.width() / 2.0;
    let window_right = window.width() / 2.0;
    let window_top = window.height() / 2.0;
    let window_bottom = -window.height() / 2.0;

    for (transform, mut velocity, mut gravity, _) in &mut balls_query {
        let new_edge_x = next_frame_edge(
            transform.translation.x,
            velocity.x,
            transform.scale.x,
            &time_step,
        );

        let new_edge_y = next_frame_edge(
            transform.translation.y,
            velocity.y,
            transform.scale.y,
            &time_step,
        );

        let current_y_edge = calculate_edge(transform.translation.y, transform.scale.y);

        if new_edge_x > window_right || new_edge_x < window_left {
            velocity.x *= -1.0;
        }

        if new_edge_y > window_top {
            velocity.y *= -1.0;

            let max_movable_distance = window_top - current_y_edge;
            if max_movable_distance < gravity.max_movable_distance {
                gravity.max_movable_distance = max_movable_distance;
            }
        }

        if new_edge_y < window_bottom {
            velocity.y *= -1.0;

            let max_movable_distance = current_y_edge - window_bottom;
            if max_movable_distance < gravity.max_movable_distance {
                gravity.max_movable_distance = max_movable_distance;
            }
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    static mut NR_RUNS: u32 = 0;
    static mut TOTAL_SPEED_SO_FAR: f32 = 0.0;
    static mut PAST_AVERAGE: f32 = 0.0;

    let mut speed_sum = 0.0;
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
        speed_sum += velocity.length();
    }

    unsafe {
        TOTAL_SPEED_SO_FAR += speed_sum;
        NR_RUNS += 1;

        if NR_RUNS > 60 * 10 {
            let average_speed = TOTAL_SPEED_SO_FAR / NR_RUNS as f32;
            info!("Average speed: {}", average_speed);

            if PAST_AVERAGE != 0.0 && (average_speed - PAST_AVERAGE).abs() > 100.0 {
                info!("SPEED HAS CHANGED SIGNIFICANTLY SINCE LAST AVERAGE");
                info!("{} vs {}", average_speed, PAST_AVERAGE);
            }
            PAST_AVERAGE = average_speed;

            NR_RUNS = 0;
            TOTAL_SPEED_SO_FAR = 0.0;
        }
    }
}

pub fn apply_gravity(
    mut query: Query<(&mut Velocity, &mut Gravity)>,
    gravity_const: Res<GravityConstant>,
    time_step: Res<FixedTime>,
) {
    for (mut velocity, mut gravity) in &mut query {
        let gravity_factor = (gravity.max_movable_distance
            / (gravity_const.0 * time_step.period.as_secs_f32().powf(2.0)))
        .clamp(0.0, 1.0);

        velocity.y -= gravity_const.0 * time_step.period.as_secs_f32() * gravity_factor;

        gravity.max_movable_distance = f32::INFINITY;
    }
}

pub fn handle_inter_ball_collision(
    mut balls_query: Query<(&Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let mut ball_combinations = balls_query.iter_combinations_mut();

    while let Some([mut ball_a, mut ball_b]) = ball_combinations.fetch_next() {
        if is_colliding(&time_step, &ball_a, &ball_b) {
            bounce(&mut ball_a, &mut ball_b);
        }
    }
}

fn is_colliding(
    time_step: &Res<FixedTime>,
    ball_a: &(&Transform, Mut<Velocity>, &Ball),
    ball_b: &(&Transform, Mut<Velocity>, &Ball),
) -> bool {
    let (transform_a, velocity_a, _) = ball_a;
    let (transform_b, velocity_b, _) = ball_b;

    let next_position_a = next_frame_coords(
        &transform_a.translation.truncate(),
        &velocity_a.0,
        time_step,
    );

    let next_position_b = next_frame_coords(
        &transform_b.translation.truncate(),
        &velocity_b.0,
        time_step,
    );

    let next_distance = next_position_a.distance(next_position_b);

    let combined_size = (transform_a.scale.x + transform_b.scale.x) / 2.0;
    let next_edge_distance = next_distance - combined_size;

    next_edge_distance <= 0.0
}

fn bounce(
    ball_a: &mut (&Transform, Mut<Velocity>, &Ball),
    ball_b: &mut (&Transform, Mut<Velocity>, &Ball),
) {
    let (transform_a, velocity_a, _) = ball_a;
    let (transform_b, velocity_b, _) = ball_b;

    let vec_between =
        (transform_b.translation.truncate() - transform_a.translation.truncate()).normalize();

    velocity_a.0 = velocity_a.0.length() * -vec_between;
    velocity_b.0 = velocity_b.0.length() * vec_between;
}

fn next_frame_edge(center: f32, velocity: f32, diameter: f32, time_step: &Res<FixedTime>) -> f32 {
    let change = center + velocity * time_step.period.as_secs_f32();

    calculate_edge(change, diameter)
}

fn calculate_edge(center: f32, diameter: f32) -> f32 {
    if center > 0.0 {
        center + diameter / 2.0
    } else {
        center - diameter / 2.0
    }
}

fn next_frame_coords(cords: &Vec2, velocity: &Vec2, time_step: &Res<FixedTime>) -> Vec2 {
    (*cords) + ((*velocity) * time_step.period.as_secs_f32())
}
