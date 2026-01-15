use bevy::ecs::prelude::*;

use bevy_egui::egui;

/// Stores area occupied by viewport
#[derive(Debug, Resource, Clone, Copy)]
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
}
