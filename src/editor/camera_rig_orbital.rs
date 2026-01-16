use bevy::prelude::*;

use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::window::CursorGrabMode;
use bevy::window::CursorOptions;
use bevy::window::PrimaryWindow;

use crate::editor::components::ViewportRenderTarget;

use super::selection_actions::SelectionActionState;

#[derive(Component)]
pub struct CurrentCamera;

#[derive(Component)]
struct OrbitXform {
    pub origin: Vec3,
    pub distance: f32,
    pub angle_x: f32,
    pub angle_y: f32,
}

impl Default for OrbitXform {
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
        app.add_systems(Update, CameraRigOrbital::update);
    }
}

impl<'a> CameraRigOrbital {
    #[allow(dead_code)]
    pub fn spawn(commands: &'a mut Commands) -> EntityCommands<'a> {
        Self::spawn_with_name(commands, "CameraRigOrbital")
    }

    pub fn spawn_with_name(
        commands: &'a mut Commands,
        name: impl Into<std::borrow::Cow<'static, str>>,
    ) -> EntityCommands<'a> {
        let mut xform = Transform::default();
        let mut data = OrbitXform::default();

        Self::refresh_xform(&mut data, &mut xform);

        commands.spawn((
            Camera3d::default(),
            xform,
            GlobalTransform::default(),
            data,
            Name::new(name),
            CurrentCamera,
        ))
    }
}

impl CameraRigOrbital {
    #[allow(clippy::too_many_arguments)]
    fn update(
        mouse_b: Res<ButtonInput<MouseButton>>,
        mut evr_mouse: MessageReader<CursorMoved>,
        mut evr_scroll: MessageReader<MouseWheel>,
        keyb: Res<ButtonInput<KeyCode>>,
        mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
        mut cursor_options: Single<&mut CursorOptions, With<Window>>,
        mut q_camera: Query<(&mut OrbitXform, &mut Transform, &ViewportRenderTarget)>,
        op: Res<SelectionActionState>,
    ) {
        let Ok(window) = q_windows.single_mut() else {
            return;
        };

        for (mut data, mut xform, render_target) in q_camera.iter_mut() {
            if !render_target.contains_cursor(&window) {
                continue;
            };

            if *op != SelectionActionState::None {
                Self::refresh_xform(&mut data, &mut xform);
                continue;
            }

            if mouse_b.pressed(MouseButton::Middle) {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                for ev in evr_mouse.read() {
                    if let Some(delta) = ev.delta {
                        if keyb.pressed(KeyCode::ShiftLeft) {
                            const MAX_X: f32 = std::f32::consts::PI / 2.0;
                            const MIN_X: f32 = -MAX_X;
                            data.angle_x -= delta.y * 0.01;
                            data.angle_x = data.angle_x.clamp(MIN_X, MAX_X);
                            data.angle_y -= delta.x * 0.01;
                        } else {
                            let dist_mult = 0.00085 * data.distance;
                            data.origin -= xform.local_x() * delta.x * dist_mult;
                            data.origin += xform.local_y() * delta.y * dist_mult;
                        }
                    }
                }
                Self::refresh_xform(&mut data, &mut xform);
            } else {
                cursor_options.grab_mode = CursorGrabMode::None;
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
                Self::refresh_xform(&mut data, &mut xform);
            }
        }
    }

    fn refresh_xform(data: &mut OrbitXform, xform: &mut Transform) {
        xform.rotation = Quat::from_rotation_x(data.angle_x);
        xform.rotate(Quat::from_rotation_y(data.angle_y));
        let offset = xform.local_z() * data.distance;
        xform.translation = data.origin + offset;
    }
}
