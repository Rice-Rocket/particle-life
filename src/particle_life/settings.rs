use bevy::{prelude::*, render::{render_resource::{UniformBuffer, ShaderType}, Extract, renderer::{RenderDevice, RenderQueue}, extract_resource::ExtractResource}};

use super::{ui::UISettings, TEXTURE_SIZE};


#[derive(Default, Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource)]
pub struct SettingsUniform {
    pub delta_time: f32,
    pub time: f32,
    pub map_width: i32,
    pub map_height: i32,

    pub attract_mean: f32,
    pub attract_std: f32,
    pub min_r_lower: f32,
    pub min_r_upper: f32,
    pub max_r_lower: f32,
    pub max_r_upper: f32,
    pub friction: f32,
    pub speed: f32,
    pub flat_force: i32,
    pub wrap: i32,
    
    // #[cfg(all(feature = "webgl", target_arch = "wasm32"))]
    // _padding: f32,
}


#[derive(Resource, Default)]
pub struct SettingsBuffer {
    pub buffer: UniformBuffer<SettingsUniform>,
}

pub fn extract_time(mut commands: Commands, time: Extract<Res<Time>>) {
    commands.insert_resource(time.clone());
}

pub fn extract_ui_settings(mut commands: Commands, settings: Extract<Res<UISettings>>) {
    commands.insert_resource(settings.clone());
}

pub fn prepare_settings_buffer(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut settings_buffer: ResMut<SettingsBuffer>,
    settings: Res<UISettings>,
    time: Res<Time>,
) {
    let buffer = settings_buffer.buffer.get_mut();
    buffer.delta_time = time.delta_seconds();
    buffer.time = time.elapsed_seconds();
    buffer.map_width = TEXTURE_SIZE.0 as i32;
    buffer.map_height = TEXTURE_SIZE.1 as i32;

    settings_buffer.buffer.write_buffer(&device, &queue);
}