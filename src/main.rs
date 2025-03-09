mod editor;
mod spline;

use bevy::{math::vec3, prelude::*};

use editor::{ui::*, *};
use spline::{Spline, SplinePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(EditorGuiPlugin)
        .add_plugins(SplinePlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraRigOrbital);
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),
    ]];

    let bezier = CubicBezier::new(points).to_curve().unwrap();

    commands.spawn(Spline(bezier));
}
