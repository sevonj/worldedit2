//! Viewport Render Target Texture

use bevy::{
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_egui::{egui, EguiUserTextures};

use crate::editor::camera_rig_orbital::CurrentCamera;

#[derive(Deref, Resource)]
pub struct ViewportRT(pub Handle<Image>);

pub fn build(app: &mut App) {
    app.add_systems(Startup, create_viewport_img);
    app.add_systems(PreUpdate, refresh_camera_rt_img);
}

/// Create viewport render target image
pub fn create_viewport_img(
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);
    egui_user_textures.add_image(image_handle.clone());
    commands.insert_resource(ViewportRT(image_handle.clone()));
}

/// Sets camera to target viewport RT image
pub fn refresh_camera_rt_img(
    mut q_camera: Query<&mut Camera, With<CurrentCamera>>,
    viewport_img: Res<ViewportRT>,
) {
    let Ok(mut camera) = q_camera.single_mut() else {
        return;
    };
    camera.target = RenderTarget::Image(ImageRenderTarget::from(viewport_img.0.clone()));
}

/// Build UI
pub fn viewport_rt_ui(
    viewport_img: Res<'_, ViewportRT>,
    images: ResMut<'_, Assets<Image>>,
    viewport_tex_id: egui::TextureId,
    ui: &mut egui::Ui,
) {
    let size = ui.available_size();

    egui::Image::new(egui::load::SizedTexture::new(viewport_tex_id, size))
        .corner_radius(4.)
        .paint_at(ui, ui.ctx().screen_rect());

    refresh_img_size(images, &viewport_img.0, size);
}

fn refresh_img_size(
    mut images: ResMut<'_, Assets<Image>>,
    handle: &Handle<Image>,
    size: bevy_egui::egui::Vec2,
) {
    let img = images.get_mut(handle).expect("no viewport image");
    if img.width() != size.x as u32 || img.height() != size.y as u32 {
        img.resize(Extent3d {
            width: size.x as u32,
            height: size.y as u32,
            ..default()
        });
    }
}
