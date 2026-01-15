use bevy::math::prelude::*;

use crate::terrain_processing::heightmap::GrayF32Image;

use crate::terrain_processing::WORLD_HEIGHT;
use crate::terrain_processing::WORLD_HEIGHT_OFFSET;

#[derive(Debug)]
pub struct HeightmapBundle {
    size: UVec2,
    base_map: GrayF32Image,
}

impl HeightmapBundle {
    pub fn new(base_map: GrayF32Image) -> Self {
        Self {
            size: uvec2(base_map.width(), base_map.height()),
            base_map,
        }
    }

    pub const fn size(&self) -> UVec2 {
        self.size
    }

    pub const fn base_map(&self) -> &GrayF32Image {
        &self.base_map
    }

    pub fn set_base_map(&mut self, base_map: GrayF32Image) {
        self.size = uvec2(base_map.width(), base_map.height());
        self.base_map = base_map;
    }

    /// Returns height for a given position. Use like a fragment shader.
    pub fn height(&self, mut position: UVec2) -> f32 {
        position = position.min(self.size - UVec2::ONE);
        let mut h = self.base_map.get_pixel(position.x, position.y)[0];
        h *= WORLD_HEIGHT;
        h += WORLD_HEIGHT_OFFSET;
        h
    }
}
