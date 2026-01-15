use bevy::prelude::*;

use crate::editor::resources::ViewportRect;

pub fn is_cursor_within_viewport(vp_rect: &ViewportRect, window: &Window) -> bool {
    let Some(cursor_pos) = window.cursor_position() else {
        return false;
    };
    cursor_pos.x > vp_rect.min_x
        && cursor_pos.y > vp_rect.min_y
        && cursor_pos.x < vp_rect.max_x
        && cursor_pos.y < vp_rect.max_y
}

pub fn cursor_position_in_viewport(vp_rect: &ViewportRect, window: &Window) -> Option<Vec2> {
    let mut cursor_pos = window.cursor_position()?;
    if !is_cursor_within_viewport(vp_rect, window) {
        return None;
    }
    cursor_pos.x -= vp_rect.min_x;
    cursor_pos.y -= vp_rect.min_y;

    Some(cursor_pos)
}
