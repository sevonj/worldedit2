use bevy::prelude::*;

use bevy::camera::ImageRenderTarget;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDescriptor;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_resource::TextureUsages;
use bevy_egui::EguiContexts;
use bevy_egui::EguiTextureHandle;

use crate::editor::camera_rig_orbital::CurrentCamera;
use crate::editor::resources::ViewportRect;

/// Viewport Render Target Texture
#[derive(Asset, Reflect)]
pub struct ViewportRenderTarget {
    pub img: Handle<Image>,
}

impl ViewportRenderTarget {
    pub fn new(contexts: &mut EguiContexts, mut images: ResMut<Assets<Image>>) -> Self {
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
        contexts.add_image(strong);
        Self {
            img: image_handle.clone(),
        }
    }

    pub fn update_size(&self, images: &mut ResMut<Assets<Image>>, vp_rect: &Res<ViewportRect>) {
        let img = images.get_mut(&self.img).expect("no viewport image");
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

    // Sets camera to target viewport RT image
    fn refresh_camera_target(&self, q_camera: &mut Query<&mut Camera, With<CurrentCamera>>) {
        let Ok(mut camera) = q_camera.single_mut() else {
            return;
        };

        camera.viewport = None;
        camera.target = RenderTarget::Image(ImageRenderTarget::from(self.img.clone()));
    }
}

#[derive(Debug)]
pub struct ViewportRenderTargetPlugin;

impl Plugin for ViewportRenderTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::pre_startup);
        app.add_systems(PreUpdate, Self::pre_update);
        app.add_systems(PostUpdate, Self::post_update);
    }
}

impl ViewportRenderTargetPlugin {
    fn pre_startup(mut commands: Commands) {
        commands.insert_resource(Assets::<ViewportRenderTarget>::default());
    }

    fn pre_update(
        mut q_camera: Query<&mut Camera, With<CurrentCamera>>,
        assets: Res<Assets<ViewportRenderTarget>>,
    ) {
        for (_, rt) in assets.iter() {
            rt.refresh_camera_target(&mut q_camera);
        }
    }

    fn post_update(
        mut images: ResMut<Assets<Image>>,
        vp_rect: Option<Res<ViewportRect>>,
        assets: Res<Assets<ViewportRenderTarget>>,
    ) {
        let Some(vp_rect) = vp_rect else {
            return;
        };
        for (_, rt) in assets.iter() {
            rt.update_size(&mut images, &vp_rect);
        }
    }
}
