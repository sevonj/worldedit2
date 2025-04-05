use bevy::{color::palettes::css::WHITE, prelude::*};

#[derive(Component)]
pub struct Spline {
    //pub name: Name,
    //pub selectable: Selectable,
    //pub transform: Transform,
    pub curve: CubicCurve<Vec3>,
}

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw);
    }
}

fn draw(query: Query<(&Spline, &Transform)>, mut gizmos: Gizmos) {
    for (spline, xform) in &query {
        gizmos.linestrip(
            spline.curve.iter_positions(50).map(|val| *xform * val),
            WHITE,
        );
    }
}

/*
impl Default for Spline {
    fn default() -> Self {
        Self {}
    }
}*/
