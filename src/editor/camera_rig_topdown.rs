use bevy::prelude::*;

use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::window::CursorGrabMode;
use bevy::window::CursorOptions;
use bevy::window::PrimaryWindow;

use crate::editor::components::ViewportRenderTarget;

use super::selection_actions::SelectionActionState;

pub struct CameraRigTopdown;

impl Plugin for CameraRigTopdown {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, CameraRigTopdown::update);
    }
}

impl<'a> CameraRigTopdown {
    #[allow(dead_code)]
    pub fn spawn(commands: &'a mut Commands) -> EntityCommands<'a> {
        Self::spawn_with_name(commands, "CameraRigTopdown")
    }

    pub fn spawn_with_name(
        commands: &'a mut Commands,
        name: impl Into<std::borrow::Cow<'static, str>>,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Camera2d::default(),
            Transform::default(),
            GlobalTransform::default(),
            Name::new(name),
        ))
    }
}

impl CameraRigTopdown {
    #[allow(clippy::too_many_arguments)]
    fn update(
        mouse_b: Res<ButtonInput<MouseButton>>,
        mut evr_mouse: MessageReader<CursorMoved>,
        mut evr_scroll: MessageReader<MouseWheel>,
        mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
        mut cursor_options: Single<&mut CursorOptions, With<Window>>,
        mut q_camera: Query<(&mut Transform, &ViewportRenderTarget)>,
        op: Res<SelectionActionState>,
    ) {
        let Ok(window) = q_windows.single_mut() else {
            return;
        };

        for (mut xform, render_target) in q_camera.iter_mut() {
            if !render_target.contains_cursor(&window) {
                continue;
            };

            if *op != SelectionActionState::None {
                continue;
            }

            if mouse_b.pressed(MouseButton::Middle) {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                for ev in evr_mouse.read() {
                    if let Some(delta) = ev.delta {
                        xform.translation.x -= delta.x * xform.scale.x;
                        xform.translation.y += delta.y * xform.scale.y;
                    }
                }
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
                    xform.scale *= mult;
                }
            }
        }
    }
}
