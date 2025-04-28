mod editor;
mod spline;

use bevy::{math::vec3, prelude::*, window::WindowResolution};

use editor::{EditorPlugin, Selectable};
use spline::{Spline, SplinePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "WorldEdit II".into(),
                resolution: WindowResolution::new(1600., 900.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(HelloPlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(SplinePlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let points = [[
        vec3(-6., 0., 0.),
        vec3(4., 0., 0.),
        vec3(-4., 4., 0.),
        vec3(6., 0., 0.),
    ]];

    let bezier = CubicBezier::new(points).to_curve().unwrap();

    commands.spawn((
        Spline { curve: bezier },
        Name::new("bezier"),
        Selectable,
        Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0)),
    ));
}
