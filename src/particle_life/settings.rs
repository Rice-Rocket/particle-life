use bevy::{prelude::*, render::{render_resource::{UniformBuffer, ShaderType, StorageBuffer}, Extract, renderer::{RenderDevice, RenderQueue}, extract_resource::ExtractResource}};

use super::{ui::UISettings, NUM_PARTICLE_TYPES, NUM_PARTICLES, TEXTURE_SIZE};


#[derive(Default, Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource)]
pub struct SettingsUniform {
    pub delta_time: f32,
    pub time: f32,
    pub inv_aspect_ratio: f32,

    pub n_types: u32,
    pub n_particles: u32,

    pub min_r: f32,
    pub max_r: f32,
    pub friction: f32,
    pub speed: f32,
    pub flat_force: i32,
    pub wrap: i32,
    
    // #[cfg(all(feature = "webgl", target_arch = "wasm32"))]
    // _padding: f32,
}


#[derive(Resource, Default)]
pub struct SettingsBuffer {
    pub settings: UniformBuffer<SettingsUniform>,
    pub aspect_ratio: UniformBuffer<f32>,
    pub attraction_tables: StorageBuffer<[f32; (NUM_PARTICLE_TYPES * NUM_PARTICLE_TYPES) as usize]>,
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
    let aspect_ratio_val = TEXTURE_SIZE.1 as f32 / TEXTURE_SIZE.0 as f32;
    let aspect_ratio = settings_buffer.aspect_ratio.get_mut();
    *aspect_ratio = aspect_ratio_val;

    let settings_uniform = settings_buffer.settings.get_mut();
    settings_uniform.delta_time = time.delta_seconds();
    settings_uniform.time = time.elapsed_seconds();
    settings_uniform.inv_aspect_ratio = 1.0 / aspect_ratio_val;
    
    settings_uniform.n_types = NUM_PARTICLE_TYPES;
    settings_uniform.n_particles = NUM_PARTICLES;

    settings_uniform.min_r = 0.3;
    settings_uniform.max_r = 0.3;
    settings_uniform.friction = 0.1;
    settings_uniform.speed = 10.0;
    settings_uniform.wrap = 1;

    let attractions = settings_buffer.attraction_tables.get_mut();
    attractions[0] = 1.0;
    attractions[1] = -1.0;
    attractions[2] = 0.2;
    attractions[3] = 0.0;
    // Row is attracted to column

    settings_buffer.attraction_tables.write_buffer(&device, &queue);
    settings_buffer.settings.write_buffer(&device, &queue);
    settings_buffer.aspect_ratio.write_buffer(&device, &queue);
}