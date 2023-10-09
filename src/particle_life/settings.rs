use bevy::{prelude::*, render::{render_resource::{UniformBuffer, ShaderType, StorageBuffer}, Extract, renderer::{RenderDevice, RenderQueue}, extract_resource::ExtractResource}};

use super::{ui::UISettings, TEXTURE_SIZE, MAX_PARTICLE_TYPES};


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
    pub wrap: i32,
    
    // #[cfg(all(feature = "webgl", target_arch = "wasm32"))]
    // _padding: f32,
}


#[derive(Resource)]
pub struct SettingsBuffer {
    pub settings: UniformBuffer<SettingsUniform>,
    pub aspect_ratio: UniformBuffer<f32>,
    pub attraction_tables: StorageBuffer<[f32; (MAX_PARTICLE_TYPES * MAX_PARTICLE_TYPES) as usize]>,
}

impl Default for SettingsBuffer {
    fn default() -> Self {
        Self {
            settings: UniformBuffer::default(),
            aspect_ratio: UniformBuffer::default(),
            attraction_tables: StorageBuffer::from([0.0; (MAX_PARTICLE_TYPES * MAX_PARTICLE_TYPES) as usize]),
        }
    }
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
    
    settings_uniform.n_types = settings.num_particle_types;
    settings_uniform.n_particles = settings.num_particle_types * settings.num_particles_per_type;

    settings_uniform.min_r = settings.min_r;
    settings_uniform.max_r = settings.max_r;
    settings_uniform.friction = 1.0 - settings.friction;
    settings_uniform.speed = settings.speed;
    settings_uniform.wrap = if settings.wrap { 1 } else { 0 };

    let attractions = settings_buffer.attraction_tables.get_mut();
    *attractions = settings.attraction_table;
    // Row is attracted to column

    settings_buffer.attraction_tables.write_buffer(&device, &queue);
    settings_buffer.settings.write_buffer(&device, &queue);
    settings_buffer.aspect_ratio.write_buffer(&device, &queue);
}