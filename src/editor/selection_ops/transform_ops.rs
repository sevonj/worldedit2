//! Transform operations for selection - Move, rotate, scale

use bevy::{
    color::palettes::tailwind::{CYAN_100, RED_100},
    prelude::*,
    window::PrimaryWindow,
};
use derive_more::Display;

use crate::editor::{ui::ViewportRect, utility::cursor_position_in_viewport, Colors};

use crate::editor::{camera_rig_orbital::CurrentCamera, selection::WithSelected, Selectable};

use super::SelectionOpsState;

#[derive(Resource, Debug, Default, PartialEq, Clone, Copy)]
pub enum TransformOp {
    #[default]
    None,
    Move {
        axis_lock: AxisLock,
        op_origin: Vec3,
    },
    Rotate {
        axis_lock: AxisLock,
        op_origin: Vec3,
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

/// Temporarily remember xform, used when canceling op
#[derive(Component, Debug)]
pub struct OriginalTransform(Transform);

/// Query selection for op switcher
type QXformOpPossible<'a> = (Entity, &'a mut Transform, Option<&'a OriginalTransform>);

/// Query: Transform op in progress
type QXformOp<'a> = (&'a mut Transform, &'a OriginalTransform);

type QCamXform<'a> = (&'a Camera, &'a Transform, &'a GlobalTransform);

type WithCurrentCam = (With<CurrentCamera>, Without<Selectable>);

pub struct TransformOpsPlugin;

impl Plugin for TransformOpsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TransformOp::default());
        app.add_systems(Update, (op_switcher, op_runner).chain());
    }
}

fn op_switcher(
    mut commands: Commands,
    mut op: ResMut<TransformOp>,
    mut selection: Query<QXformOpPossible, WithSelected>,
    mut selection_state: ResMut<SelectionOpsState>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    kb: Res<ButtonInput<KeyCode>>,
    mb: Res<ButtonInput<MouseButton>>,
) {
    #[allow(unreachable_patterns)] // There will be more
    match *selection_state {
        SelectionOpsState::None | SelectionOpsState::Transform => (),
        _ => return,
    }

    if selection.is_empty() {
        cleanup_op(&mut commands, &mut selection, &mut selection_state, &mut op);
        return;
    }

    let Ok(window) = q_windows.single() else {
        return;
    };

    if kb.just_pressed(KeyCode::Escape) {
        if *op != TransformOp::None {
            cancel_op(&mut commands, &mut selection, &mut selection_state, &mut op);
        }
        return;
    }

    if kb.just_pressed(KeyCode::Enter) || mb.just_pressed(MouseButton::Left) {
        commit_op(&mut commands, &mut selection, &mut selection_state, &mut op);
        return;
    }

    if kb.just_pressed(KeyCode::KeyG) {
        match *op {
            TransformOp::None => init_op(&mut commands, &selection, &mut selection_state),
            TransformOp::Move { .. } => return,
            _ => undo_changes(&mut selection),
        }
        *op = TransformOp::Move {
            axis_lock: AxisLock::default(),
            op_origin: selection_bb_center(&selection),
        };
    } else if kb.just_pressed(KeyCode::KeyR) {
        let Some(original_cursor_pos) = window.cursor_position() else {
            return;
        };

        match *op {
            TransformOp::None => init_op(&mut commands, &selection, &mut selection_state),
            TransformOp::Rotate { .. } => return,
            _ => undo_changes(&mut selection),
        }
        *op = TransformOp::Rotate {
            axis_lock: AxisLock::default(),
            op_origin: selection_bb_center(&selection),
            original_cursor_pos,
        };
    }

    update_axis_lock(&mut op, &kb);
}

fn op_runner(
    op: ResMut<TransformOp>,
    q_selection: Query<QXformOp, WithSelected>,
    q_camera: Query<QCamXform, WithCurrentCam>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
    vp_rect: Res<ViewportRect>,
) {
    let Ok((camera, camera_xform, camera_global)) = q_camera.single() else {
        return;
    };

    match op.as_ref() {
        TransformOp::None => return,
        TransformOp::Move {
            axis_lock,
            op_origin,
        }
        | TransformOp::Rotate {
            axis_lock,
            op_origin,
            ..
        } => draw_axis_gizmo_lines(&mut gizmos, axis_lock, op_origin),
        TransformOp::Scale(..) => todo!(),
    }

    let Ok(window) = q_windows.single() else {
        return;
    };

    let Some(cursor_pos) = cursor_position_in_viewport(&vp_rect, window) else {
        return;
    };

    match op.as_ref() {
        TransformOp::None => unreachable!(),
        TransformOp::Move {
            axis_lock,
            op_origin: original_pos,
        } => op_move(
            q_selection,
            camera,
            camera_xform,
            camera_global,
            original_pos,
            cursor_pos,
            axis_lock,
            gizmos,
        ),
        TransformOp::Rotate {
            axis_lock,
            op_origin: center_pos,
            original_cursor_pos,
        } => op_rotate(
            q_selection,
            camera,
            camera_xform,
            camera_global,
            center_pos,
            cursor_pos,
            *original_cursor_pos,
            axis_lock,
            gizmos,
        ),
        TransformOp::Scale(_axis_lock) => todo!(),
    }
}

fn draw_axis_gizmo_lines(gizmos: &mut Gizmos<'_, '_>, axis_lock: &AxisLock, op_origin: &Vec3) {
    const LINE_X: Vec3 = Vec3 {
        x: 1024.,
        y: 0.,
        z: 0.,
    };
    const LINE_Y: Vec3 = Vec3 {
        x: 0.,
        y: 1024.,
        z: 0.,
    };
    const LINE_Z: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 1024.,
    };
    match axis_lock {
        AxisLock::Free => (),
        AxisLock::X => gizmos.line(-LINE_X + op_origin, LINE_X + op_origin, Colors::AXIS_X),
        AxisLock::Y => gizmos.line(-LINE_Y + op_origin, LINE_Y + op_origin, Colors::AXIS_Y),
        AxisLock::Z => gizmos.line(-LINE_Z + op_origin, LINE_Z + op_origin, Colors::AXIS_Z),
        AxisLock::PlaneX => {
            gizmos.line(-LINE_Y + op_origin, LINE_Y + op_origin, Colors::AXIS_Y_SOFT);
            gizmos.line(-LINE_Z + op_origin, LINE_Z + op_origin, Colors::AXIS_Z_SOFT);
        }
        AxisLock::PlaneY => {
            gizmos.line(-LINE_X + op_origin, LINE_X + op_origin, Colors::AXIS_X_SOFT);
            gizmos.line(-LINE_Z + op_origin, LINE_Z + op_origin, Colors::AXIS_Z_SOFT);
        }
        AxisLock::PlaneZ => {
            gizmos.line(-LINE_X + op_origin, LINE_X + op_origin, Colors::AXIS_X_SOFT);
            gizmos.line(-LINE_Y + op_origin, LINE_Y + op_origin, Colors::AXIS_Y_SOFT);
        }
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

fn selection_bb_center(query: &Query<QXformOpPossible, WithSelected>) -> Vec3 {
    let mut min_translation = Vec3::INFINITY;
    let mut max_translation = -Vec3::INFINITY;
    for (_, xform, ..) in query.iter() {
        min_translation = min_translation.min(xform.translation);
        max_translation = max_translation.max(xform.translation);
    }
    (max_translation + min_translation) / 2.0
}

fn init_op(
    commands: &mut Commands,
    selection: &Query<QXformOpPossible, WithSelected>,
    selection_state: &mut ResMut<SelectionOpsState>,
) {
    for (entity, xform, og_xform) in selection.iter() {
        assert!(og_xform.is_none());
        commands.entity(entity).insert(OriginalTransform(*xform));
    }
    **selection_state = SelectionOpsState::Transform;
}

fn undo_changes(selection: &mut Query<QXformOpPossible, WithSelected>) {
    for (.., mut xform, og_xform) in selection.iter_mut() {
        let og_xform = og_xform.expect("og_xform was none?").0;
        *xform = og_xform;
    }
}

/// Cleans up the op, call after you've committed or reverted the changes
fn cleanup_op(
    commands: &mut Commands,
    selection: &mut Query<QXformOpPossible, WithSelected>,
    selection_state: &mut ResMut<SelectionOpsState>,
    op: &mut TransformOp,
) {
    match op {
        TransformOp::None => return,
        _ => {
            for (entity, ..) in selection.iter_mut() {
                commands.entity(entity).remove::<OriginalTransform>();
            }
        }
    }
    *op = TransformOp::None;
    **selection_state = SelectionOpsState::None;
}

fn cancel_op(
    commands: &mut Commands,
    selection: &mut Query<QXformOpPossible, WithSelected>,
    selection_state: &mut ResMut<SelectionOpsState>,
    op: &mut TransformOp,
) {
    undo_changes(selection);
    cleanup_op(commands, selection, selection_state, op);
}

fn commit_op(
    commands: &mut Commands,
    selection: &mut Query<QXformOpPossible, WithSelected>,
    selection_state: &mut ResMut<SelectionOpsState>,
    op: &mut TransformOp,
) {
    cleanup_op(commands, selection, selection_state, op);
}

#[allow(clippy::too_many_arguments)]
fn op_move(
    mut q_selection: Query<QXformOp, WithSelected>,
    camera: &Camera,
    camera_xform: &Transform,
    camera_global: &GlobalTransform,
    original_pos: &Vec3,
    cursor_pos: Vec2,
    axis_lock: &AxisLock,
    mut gizmos: Gizmos,
) {
    gizmos.circle(Isometry3d::from_translation(*original_pos), 0.5, CYAN_100);

    let look_dir = camera_xform.forward().as_vec3();
    let axis = match axis_lock {
        AxisLock::Free => look_dir,
        AxisLock::X => Dir3::X.cross(Dir3::X.cross(look_dir)), // Axis-Billboard
        AxisLock::Y => Dir3::Y.cross(Dir3::Y.cross(look_dir)), // Axis-Billboard
        AxisLock::Z => Dir3::Z.cross(Dir3::Z.cross(look_dir)), // Axis-Billboard
        AxisLock::PlaneX => Dir3::X.as_vec3(),
        AxisLock::PlaneY => Dir3::Y.as_vec3(),
        AxisLock::PlaneZ => Dir3::Z.as_vec3(),
    };
    let plane = InfinitePlane3d::new(axis);
    let Some(pos) = plane_line_intersect(cursor_pos, camera, camera_global, &plane, original_pos)
    else {
        return;
    };

    for (mut xform, ..) in q_selection.iter_mut() {
        match axis_lock {
            AxisLock::X => xform.translation.x = pos.x,
            AxisLock::Y => xform.translation.y = pos.y,
            AxisLock::Z => xform.translation.z = pos.z,
            AxisLock::Free | AxisLock::PlaneX | AxisLock::PlaneY | AxisLock::PlaneZ => {
                xform.translation = pos;
            }
        }
    }

    gizmos.circle(Isometry3d::from_translation(pos), 0.5, RED_100);
}

#[allow(clippy::too_many_arguments)]
fn op_rotate(
    mut q_selection: Query<QXformOp, WithSelected>,
    camera: &Camera,
    camera_xform: &Transform,
    camera_global: &GlobalTransform,
    op_origin: &Vec3,
    cursor_pos: Vec2,
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

    let plane = InfinitePlane3d::new(axis);
    let Some(new_world_pos) =
        plane_line_intersect(cursor_pos, camera, camera_global, &plane, op_origin)
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

    for (mut xform, og_xform) in q_selection.iter_mut() {
        xform.rotation = Quat::from_axis_angle(axis, angle) * og_xform.0.rotation;
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

/// Gets you an angle that's usable for rotating things.
fn angle_between_signed(a: Vec3, b: Vec3, normal: Vec3) -> f32 {
    let angle = a.angle_between(b);
    if normal.dot(a.cross(b)).signum().is_sign_negative() {
        return -angle;
    }
    angle
}
