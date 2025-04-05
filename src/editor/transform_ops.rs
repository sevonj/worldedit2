//! Transform operations for selection - Move, rotate, scale

use bevy::{
    color::palettes::tailwind::{CYAN_100, RED_100},
    prelude::*,
    window::PrimaryWindow,
};
use derive_more::Display;

use super::{camera_rig_orbital::CurrentCamera, selection::Selected, Selectable};

#[derive(Resource, Debug, Default, PartialEq, Clone, Copy)]
pub enum TransformOp {
    #[default]
    None,
    Move {
        axis_lock: AxisLock,
        original_pos: Vec3,
    },
    Rotate {
        axis_lock: AxisLock,
        original_rot: Quat,
        center_pos: Vec3,
        original_cursor_pos: Vec2,
    },
    #[allow(dead_code)]
    Scale(AxisLock),
}

impl std::fmt::Display for TransformOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformOp::None => write!(f, "No operation"),
            TransformOp::Move { .. } => write!(f, "Move"),
            TransformOp::Rotate { .. } => write!(f, "Rotate"),
            TransformOp::Scale(..) => write!(f, "Scale"),
        }
    }
}

#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq)]
pub enum AxisLock {
    #[default]
    Free,
    X,
    Y,
    Z,
    PlaneX,
    PlaneY,
    PlaneZ,
}

pub struct TransformOpsPlugin;

impl Plugin for TransformOpsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TransformOp::default());
        app.add_systems(Update, update);
    }
}

#[allow(clippy::type_complexity)]
fn update(
    mut op: ResMut<TransformOp>,
    q_selection: Query<&mut Transform, (With<Selectable>, With<Selected>)>,
    q_camera: Query<
        (&Camera, &Transform, &GlobalTransform),
        (With<CurrentCamera>, Without<Selectable>),
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    kb: Res<ButtonInput<KeyCode>>,
    mb: Res<ButtonInput<MouseButton>>,
    gizmos: Gizmos,
) {
    if q_selection.is_empty() {
        *op = TransformOp::None;
        return;
    }

    if kb.just_pressed(KeyCode::Escape) {
        cancel_op(q_selection, &mut op);
        return;
    } else if kb.just_pressed(KeyCode::Enter) || mb.just_pressed(MouseButton::Left) {
        commit_op(q_selection, &mut op);
        return;
    }

    let (camera, camera_xform, camera_global) = q_camera.single();
    let window = q_windows.single();

    if kb.just_pressed(KeyCode::KeyG) {
        *op = TransformOp::Move {
            axis_lock: AxisLock::default(),
            original_pos: selection_center(&q_selection),
        };
    } else if kb.just_pressed(KeyCode::KeyR) {
        let Some(original_cursor_pos) = window.cursor_position() else {
            return;
        };

        *op = TransformOp::Rotate {
            axis_lock: AxisLock::default(),
            original_rot: Quat::default(),
            center_pos: selection_center(&q_selection),
            original_cursor_pos,
        };
    }

    update_axis_lock(&mut op, &kb);

    match op.as_ref() {
        TransformOp::None => (),
        TransformOp::Move {
            axis_lock,
            original_pos,
        } => op_move(
            q_selection,
            camera,
            camera_xform,
            camera_global,
            window,
            original_pos,
            axis_lock,
            gizmos,
        ),
        TransformOp::Rotate {
            axis_lock,
            original_rot,
            center_pos,
            original_cursor_pos,
        } => op_rotate(
            q_selection,
            camera,
            camera_xform,
            camera_global,
            window,
            original_rot,
            center_pos,
            *original_cursor_pos,
            axis_lock,
            gizmos,
        ),
        TransformOp::Scale(_axis_lock) => todo!(),
    }
}

fn update_axis_lock(op: &mut ResMut<TransformOp>, kb: &Res<ButtonInput<KeyCode>>) {
    match op.as_mut() {
        TransformOp::None => (),
        TransformOp::Move { axis_lock, .. }
        | TransformOp::Rotate { axis_lock, .. }
        | TransformOp::Scale(axis_lock) => {
            if kb.pressed(KeyCode::ShiftLeft) {
                if kb.just_pressed(KeyCode::KeyX) {
                    *axis_lock = AxisLock::PlaneX
                } else if kb.just_pressed(KeyCode::KeyY) {
                    *axis_lock = AxisLock::PlaneY
                } else if kb.just_pressed(KeyCode::KeyZ) {
                    *axis_lock = AxisLock::PlaneZ
                }
            } else if kb.just_pressed(KeyCode::KeyX) {
                *axis_lock = AxisLock::X
            } else if kb.just_pressed(KeyCode::KeyY) {
                *axis_lock = AxisLock::Y
            } else if kb.just_pressed(KeyCode::KeyZ) {
                *axis_lock = AxisLock::Z
            }
        }
    }
}

fn selection_center(
    query: &Query<'_, '_, &mut Transform, (With<Selectable>, With<Selected>)>,
) -> Vec3 {
    let mut min_translation = Vec3::INFINITY;
    let mut max_translation = -Vec3::INFINITY;
    for xform in query.iter() {
        min_translation = min_translation.min(xform.translation);
        max_translation = max_translation.max(xform.translation);
    }
    max_translation - min_translation / 2.0
}

fn cancel_op(
    mut query: Query<&mut Transform, (With<Selectable>, With<Selected>)>,
    op: &mut TransformOp,
) {
    match op {
        TransformOp::None => return,
        TransformOp::Move { original_pos, .. } => {
            for mut xform in query.iter_mut() {
                xform.translation = *original_pos;
            }
        }
        TransformOp::Rotate { .. } => todo!(),
        TransformOp::Scale(_axis_lock) => todo!(),
    }
    *op = TransformOp::None;
}

fn commit_op(
    mut _query: Query<&mut Transform, (With<Selectable>, With<Selected>)>,
    op: &mut TransformOp,
) {
    *op = TransformOp::None;
}

#[allow(clippy::too_many_arguments)]
fn op_move(
    mut q_selection: Query<&mut Transform, (With<Selectable>, With<Selected>)>,
    camera: &Camera,
    camera_xform: &Transform,
    camera_global: &GlobalTransform,
    window: &Window,
    original_pos: &Vec3,
    axis_lock: &AxisLock,
    mut gizmos: Gizmos,
) {
    gizmos.circle(Isometry3d::from_translation(*original_pos), 0.5, CYAN_100);

    let look = camera_xform.forward().as_vec3();
    const X: Vec3 = Vec3::AXES[0];
    const Y: Vec3 = Vec3::AXES[1];
    const Z: Vec3 = Vec3::AXES[2];
    let move_axis = match axis_lock {
        AxisLock::Free => look,
        AxisLock::X => {
            let right = X.cross(look);
            X.cross(right)
        }
        AxisLock::Y => {
            let right = Y.cross(look);
            Y.cross(right)
        }
        AxisLock::Z => {
            let right = Z.cross(look);
            Z.cross(right)
        }
        AxisLock::PlaneX => Dir3::X.as_vec3(),
        AxisLock::PlaneY => Dir3::Y.as_vec3(),
        AxisLock::PlaneZ => Dir3::Z.as_vec3(),
    };
    let plane = InfinitePlane3d::new(move_axis);
    let Some(viewport_position) = window.cursor_position() else {
        return;
    };
    let Some(pos) = plane_line_intersect(
        viewport_position,
        camera,
        camera_global,
        &plane,
        original_pos,
    ) else {
        return;
    };
    match axis_lock {
        AxisLock::X => {
            for mut xform in q_selection.iter_mut() {
                xform.translation.x = pos.x;
            }
        }
        AxisLock::Y => {
            for mut xform in q_selection.iter_mut() {
                xform.translation.y = pos.y;
            }
        }
        AxisLock::Z => {
            for mut xform in q_selection.iter_mut() {
                xform.translation.z = pos.z;
            }
        }
        AxisLock::Free | AxisLock::PlaneX | AxisLock::PlaneY | AxisLock::PlaneZ => {
            for mut xform in q_selection.iter_mut() {
                xform.translation = pos;
            }
        }
    }

    gizmos.circle(Isometry3d::from_translation(pos), 0.5, RED_100);
}

#[allow(clippy::too_many_arguments)]
fn op_rotate(
    mut q_selection: Query<&mut Transform, (With<Selectable>, With<Selected>)>,
    camera: &Camera,
    camera_xform: &Transform,
    camera_global: &GlobalTransform,
    window: &Window,
    original_rot: &Quat,
    op_origin: &Vec3,
    original_cursor_pos: Vec2,
    axis_lock: &AxisLock,
    mut gizmos: Gizmos,
) {
    gizmos.circle(Isometry3d::from_translation(*op_origin), 0.5, CYAN_100);

    let look = camera_xform.forward().as_vec3();

    let axis = match axis_lock {
        AxisLock::Free => look,
        AxisLock::X | AxisLock::PlaneX => Dir3::X.as_vec3(),
        AxisLock::Y | AxisLock::PlaneY => Dir3::Y.as_vec3(),
        AxisLock::Z | AxisLock::PlaneZ => Dir3::Z.as_vec3(),
    };

    let Some(cursor_position) = window.cursor_position() else {
        println!("bailed: no cursor pos");
        return;
    };
    let plane = InfinitePlane3d::new(axis);
    let Some(new_world_pos) =
        plane_line_intersect(cursor_position, camera, camera_global, &plane, op_origin)
    else {
        return;
    };
    let Some(old_world_pos) = plane_line_intersect(
        original_cursor_pos,
        camera,
        camera_global,
        &plane,
        op_origin,
    ) else {
        return;
    };
    let new_vector = new_world_pos - op_origin;
    let old_vector = old_world_pos - op_origin;
    let angle = angle_between_signed(old_vector, new_vector, axis);

    for mut xform in q_selection.iter_mut() {
        xform.rotation = *original_rot * Quat::from_axis_angle(axis, angle);
    }

    gizmos.circle(Isometry3d::from_translation(new_world_pos), 0.5, RED_100);
}

fn plane_line_intersect(
    viewport_position: Vec2,
    camera: &Camera,
    camera_global: &GlobalTransform,
    plane: &InfinitePlane3d,
    plane_origin: &Vec3,
) -> Option<Vec3> {
    let Ok(ray) = camera.viewport_to_world(camera_global, viewport_position) else {
        return None;
    };
    let dist = ray.intersect_plane(*plane_origin, *plane)?;
    Some(ray.get_point(dist))
}

fn angle_between_signed(a: Vec3, b: Vec3, normal: Vec3) -> f32 {
    let angle = a.angle_between(b);
    if normal.dot(a.cross(b)).signum().is_sign_negative() {
        return -angle;
    }
    angle
}
