//! Viewport Render Target Texture

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_egui::EguiUserTextures;
use egui_tiles::Tile;

use crate::editor::{
    camera_rig_orbital::CurrentCamera,
    ui::{ui_tiling::{TileTree, TilingPane}, viewport_pane::ViewportPane},
};

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
    let mut camera = q_camera.single_mut();
    camera.viewport = None;
    camera.target = RenderTarget::Image(viewport_img.0.clone());
}

pub fn update_viewport_img_size(
    mut images: ResMut<Assets<Image>>,
    viewport_img: Res<ViewportRT>,
    tree: Res<TileTree>,
) {
    // TODO: yeah this is dumb
    fn find_the_pane<'a>(tree: &'a Res<'a, TileTree>) -> Option<&'a ViewportPane> {
        for tile in tree.0.tiles.tiles() {
            if let Tile::Pane(TilingPane::ViewPort(pane)) = tile {
                return Some(pane);
            }
        }
        None
    }

    let img = images.get_mut(&viewport_img.0).expect("no viewport image");
    let viewport_pane = find_the_pane(&tree).unwrap();
    let size = viewport_pane.size();

    if img.width() != size.x as u32 || img.height() != size.y as u32 {
        img.resize(Extent3d {
            width: size.x as u32,
            height: size.y as u32,
            ..default()
        });
    }
}
