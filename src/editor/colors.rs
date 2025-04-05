use bevy::prelude::*;

use bevy::color::palettes::tailwind::{
    BLUE_200, BLUE_400, GRAY_500, GRAY_600, GREEN_200, GREEN_400, RED_200, RED_400,
};

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
