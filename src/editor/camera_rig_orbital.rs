use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use super::transform_ops::TransformOp;

#[derive(Component)]
pub struct CurrentCamera;

#[derive(Component)]
struct CameraRigOrbitalData {
    /// Position the camera orbits around
    pub origin: Vec3,
    /// Cam distance from origin
    pub distance: f32,
    pub angle_x: f32,
    pub angle_y: f32,
}

impl Default for CameraRigOrbitalData {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            distance: 10.0,
            angle_x: -45.0_f32.to_radians(),
            angle_y: 0.0,
        }
    }
}

pub struct CameraRigOrbital;

impl Plugin for CameraRigOrbital {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, ready).chain());
        app.add_systems(Update, update_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::default(),
        GlobalTransform::default(),
        CameraRigOrbitalData::default(),
        Name::new("CameraRigOrbital"),
        CurrentCamera,
    ));
}

fn update_camera(
    mouse_b: Res<ButtonInput<MouseButton>>,
    mut evr_mouse: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    keyb: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<(&mut CameraRigOrbitalData, &mut Transform), With<Camera3d>>,
    op: Res<TransformOp>,
) {
    let (mut data, mut xform) = query.single_mut();
    if *op != TransformOp::None {
        refresh_camera_xform(&mut data, &mut xform);
        return;
    }

    let mut window = q_windows.single_mut();

    if mouse_b.pressed(MouseButton::Middle) {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        let mut input_vec = Vec2::ZERO;

        for ev in evr_mouse.read() {
            input_vec.x += ev.delta.x;
            input_vec.y += ev.delta.y;
        }

        if keyb.pressed(KeyCode::ShiftLeft) {
            const MAX_X: f32 = std::f32::consts::PI / 2.0;
            const MIN_X: f32 = -MAX_X;
            data.angle_x -= input_vec.y * 0.01;
            data.angle_x = data.angle_x.clamp(MIN_X, MAX_X);
            data.angle_y -= input_vec.x * 0.01;
        } else {
            let dist_mult = 0.00085 * data.distance;
            data.origin -= xform.local_x() * input_vec.x * dist_mult;
            data.origin += xform.local_y() * input_vec.y * dist_mult;
        }

        refresh_camera_xform(&mut data, &mut xform);
    } else {
        window.cursor_options.grab_mode = CursorGrabMode::None;

        for ev in evr_scroll.read() {
            let mut mult = match ev.unit {
                MouseScrollUnit::Line => 0.8 * ev.y.abs(),
                // Untested
                MouseScrollUnit::Pixel => 0.99 * ev.y.abs(),
            };
            if ev.y < 0. {
                mult = 1. / mult;
            }
            data.distance *= mult;
        }

        refresh_camera_xform(&mut data, &mut xform);
    }
}

fn ready(mut query: Query<(&mut CameraRigOrbitalData, &mut Transform), With<Camera3d>>) {
    let (mut data, mut xform) = query.single_mut();
    refresh_camera_xform(&mut data, &mut xform);
}

fn refresh_camera_xform(data: &mut CameraRigOrbitalData, xform: &mut Transform) {
    xform.rotation = Quat::from_rotation_x(data.angle_x);
    xform.rotate(Quat::from_rotation_y(data.angle_y));
    let offset = xform.local_z() * data.distance;
    xform.translation = data.origin + offset;
}
