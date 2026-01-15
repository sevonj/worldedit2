use bevy::prelude::*;

use bevy::camera::ImageRenderTarget;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDescriptor;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_resource::TextureUsages;
use bevy_egui::EguiTextureHandle;
use bevy_egui::EguiUserTextures;

use crate::editor::camera_rig_orbital::CurrentCamera;
use crate::editor::resources::ViewportRect;

/// Viewport Render Target Texture
#[derive(Deref, Resource)]
pub struct ViewportRenderTarget(pub Handle<Image>);

impl ViewportRenderTarget {
    pub fn new(
        mut egui_user_textures: ResMut<EguiUserTextures>,
        mut images: ResMut<Assets<Image>>,
    ) -> Self {
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
        let strong = EguiTextureHandle::Strong(image_handle.clone());
        egui_user_textures.add_image(strong);
        Self(image_handle.clone())
    }
}

#[derive(Debug)]
pub struct ViewportRenderTargetPlugin;

impl Plugin for ViewportRenderTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, Self::pre_update);
        app.add_systems(PostUpdate, Self::post_update);
    }
}

impl ViewportRenderTargetPlugin {
    fn pre_update(
        q_camera: Query<&mut Camera, With<CurrentCamera>>,
        viewport_img: Res<ViewportRenderTarget>,
    ) {
        Self::refresh_camera_target(q_camera, viewport_img);
    }

    fn post_update(
        images: ResMut<Assets<Image>>,
        vp_rect: Res<ViewportRect>,
        viewport_img: Res<ViewportRenderTarget>,
    ) {
        Self::update_size(images, vp_rect, viewport_img);
    }

    // Sets camera to target viewport RT image
    fn refresh_camera_target(
        mut q_camera: Query<&mut Camera, With<CurrentCamera>>,
        viewport_img: Res<ViewportRenderTarget>,
    ) {
        let Ok(mut camera) = q_camera.single_mut() else {
            return;
        };
        camera.viewport = None;
        camera.target = RenderTarget::Image(ImageRenderTarget::from(viewport_img.0.clone()));
    }

    fn update_size(
        mut images: ResMut<Assets<Image>>,
        vp_rect: Res<ViewportRect>,
        viewport_img: Res<ViewportRenderTarget>,
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
}
