use bevy::prelude::*;

use bevy_egui::egui;

/// Stores area occupied by viewport
#[derive(Debug, Component, Clone, Copy)]
pub struct ViewportRect {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Default for ViewportRect {
    fn default() -> Self {
        // Non-zero placeholder size
        Self {
            min_x: 16.,
            min_y: 16.,
            max_x: 16.,
            max_y: 16.,
        }
    }
}

impl From<egui::Rect> for ViewportRect {
    fn from(rect: egui::Rect) -> Self {
        Self {
            min_x: rect.min.x,
            min_y: rect.min.y,
            max_x: rect.max.x,
            max_y: rect.max.y,
        }
    }
}

impl ViewportRect {
    pub const fn size(&self) -> egui::Vec2 {
        egui::Vec2 {
            x: self.max_x - self.min_x,
            y: self.max_y - self.min_y,
        }
    }

    pub fn contains_cursor(&self, window: &Window) -> bool {
        let Some(cursor_pos) = window.cursor_position() else {
            return false;
        };
        cursor_pos.x > self.min_x
            && cursor_pos.y > self.min_y
            && cursor_pos.x < self.max_x
            && cursor_pos.y < self.max_y
    }

    pub fn cursor_position(&self, window: &Window) -> Option<Vec2> {
        let mut cursor_pos = window.cursor_position()?;
        if !self.contains_cursor(window) {
            return None;
        }
        cursor_pos.x -= self.min_x;
        cursor_pos.y -= self.min_y;

        Some(cursor_pos)
    }
}
