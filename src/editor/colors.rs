use bevy::prelude::*;

use bevy::color::palettes::tailwind::*;

pub struct Colors;

impl Colors {
    pub const AXIS_X: Srgba = RED_400;
    pub const AXIS_X_SOFT: Srgba = RED_200;
    pub const AXIS_Y: Srgba = GREEN_400;
    pub const AXIS_Y_SOFT: Srgba = GREEN_200;
    pub const AXIS_Z: Srgba = BLUE_400;
    pub const AXIS_Z_SOFT: Srgba = BLUE_200;
    pub const GRID_MAJOR: Srgba = GRAY_500;
    pub const GRID_MINOR: Srgba = GRAY_600;
}
