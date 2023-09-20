use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Ball {
    pub mass: f32,
}

#[derive(Resource)]
pub struct Gravity(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
