use bevy::{color::palettes::css::WHITE, prelude::*};

#[derive(Component)]
pub struct Spline(pub CubicCurve<Vec3>);

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw);
    }
}

fn draw(query: Query<&Spline>, mut gizmos: Gizmos) {
    for spline in &query {
        gizmos.linestrip(spline.0.iter_positions(50), WHITE);
    }
}

/*
impl Default for Spline {
    fn default() -> Self {
        Self {}
    }
}*/
