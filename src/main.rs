mod editor;
mod spline;

use bevy::camera::visibility::RenderLayers;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::RenderCreation;
use bevy::render::settings::WgpuFeatures;
use bevy::render::settings::WgpuSettings;
use bevy::window::WindowResolution;
use bevy_egui::EguiGlobalSettings;
use bevy_egui::PrimaryEguiContext;

use editor::Selectable;
use spline::Spline;

use bevy::pbr::wireframe::WireframePlugin;
use editor::EditorPlugin;
use spline::SplinePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "WorldEdit II".into(),
                        resolution: WindowResolution::new(1600, 900),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WireframePlugin::default())
        .add_plugins(HelloPlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(SplinePlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, mut egui_global_settings: ResMut<EguiGlobalSettings>) {
    egui_global_settings.auto_create_primary_context = false;

    let points = [[
        vec3(-6., 0., 0.),
        vec3(4., 0., 0.),
        vec3(-4., 4., 0.),
        vec3(6., 0., 0.),
    ]];

    let bezier = CubicBezier::new(points).to_curve().unwrap();

    commands.spawn((
        Spline { curve: bezier },
        Name::new("bezier"),
        Selectable,
        Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0)),
    ));

    // Egui camera
    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        RenderLayers::none(),
        Camera {
            order: 1,
            ..default()
        },
    ));
}
