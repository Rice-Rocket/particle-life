use bevy::{prelude::*, window::PrimaryWindow, render::{render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, extract_resource::ExtractResource}, core_pipeline::tonemapping::Tonemapping};

use super::TEXTURE_SIZE;

#[derive(Component)]
pub struct ParticleLifeOutputImageEntity {}

pub fn setup_texture(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    mut images: ResMut<Assets<Image>>
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE.0,
            height: TEXTURE_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    let image = images.add(image);

    let window = window_query.get_single().unwrap();

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        texture: image.clone(),
        ..default()
    }, ParticleLifeOutputImageEntity {}));
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::None,
        ..default()
    });

    commands.insert_resource(ParticleLifeImage(image));
}


#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct ParticleLifeImage(pub Handle<Image>);
