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
use bevy_egui::EguiUserTextures;

use crate::editor::camera_rig_orbital::CurrentCamera;

use super::ViewportRect;

#[derive(Deref, Resource)]
pub struct ViewportRT(pub Handle<Image>);

/// Create viewport render target image
pub fn create_viewport_img(
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 64,
        height: 64,
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
pub fn refresh_camera_target(
    mut q_camera: Query<&mut Camera, With<CurrentCamera>>,
    viewport_img: Res<ViewportRT>,
) {
    let Ok(mut camera) = q_camera.single_mut() else {
        return;
    };
    camera.viewport = None;
    camera.target = RenderTarget::Image(ImageRenderTarget::from(viewport_img.0.clone()));
}

pub fn update_viewport_img_size(
    mut images: ResMut<Assets<Image>>,
    viewport_img: Res<ViewportRT>,
    vp_rect: Res<ViewportRect>,
) {
    let img = images.get_mut(&viewport_img.0).expect("no viewport image");
    let size = vp_rect.size();

    // Zero-size image panics and is useless anyway.
    if size.x < 1.0 || size.y < 1.0 {
        return;
    }

    if img.width() != size.x as u32 || img.height() != size.y as u32 {
        img.resize(Extent3d {
            width: size.x as u32,
            height: size.y as u32,
            ..default()
        });
    }
}
