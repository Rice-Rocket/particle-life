use bevy::{prelude::*, render::{render_resource::{ShaderType, Buffer, BufferUsages, BufferInitDescriptor}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};
use rand::Rng;

// use crate::particle_life::TEXTURE_SIZE;

use crate::particle_life::TEXTURE_SIZE;

use super::{MAX_PARTICLES, INIT_NUM_TYPES, INIT_NUM_PARTICLES_PER_TYPE, ui::UISettings, MAX_PARTICLE_TYPES, INIT_PARTICLE_RADIUS};


#[derive(Debug, Clone, Copy, Reflect, ShaderType, Pod, Zeroable)]
#[repr(C)]
pub struct Particle {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub color: [f32; 3],
    pub type_idx: u32,
}

impl Particle {
    fn new() -> Self {
        Self {
            pos: [0.0; 2],
            vel: [0.0; 2],
            color: [0.0; 3],
            type_idx: 0,
        }
    }
}

#[derive(Resource)]
pub struct ParticlesBuffer {
    pub storage: Buffer,
    pub staging: Buffer,
    pub vertex_data: Buffer,
    pub index_data: Buffer,
    pub size: u64,
}

impl FromWorld for ParticlesBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let size = (MAX_PARTICLES * std::mem::size_of::<f32>() as u32 * 8) as u64;
        let particles = create_particles(INIT_NUM_TYPES, INIT_NUM_PARTICLES_PER_TYPE);
        
        let staging = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&particles),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST | BufferUsages::VERTEX,
        });

        let storage = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&particles),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        });

        let (vertices, indices) = create_hexagon_data(INIT_PARTICLE_RADIUS);
        let vertex_data = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let index_data = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        Self {
            storage,
            staging,
            vertex_data,
            index_data,
            size,
        }
    }
}

pub fn create_particle_colors(n_types: u32) -> [[f32; 3]; MAX_PARTICLE_TYPES as usize] {
    const COLOR_A: Vec3 = Vec3::new(0.5, 0.5, 0.5);
    const COLOR_B: Vec3 = Vec3::new(0.5, 0.5, 0.5);
    const COLOR_C: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    const COLOR_D: Vec3 = Vec3::new(0.0, 0.333, 0.667);

    let mut colors = [[0.0; 3]; MAX_PARTICLE_TYPES as usize];
    for i in 0..n_types {
        let t = i as f32 / n_types as f32;
        let c1 = std::f32::consts::TAU * (COLOR_C * t + COLOR_D);
        let c1cosx = c1.x.cos();
        let c1cosy = c1.y.cos();
        let c1cosz = c1.z.cos();
        let color = COLOR_A + COLOR_B * Vec3::new(c1cosx, c1cosy, c1cosz);
        colors[i as usize] = [color.x, color.y, color.z];
    };
    return colors;
}

fn create_particles(n_types: u32, n_per_type: u32) -> [Particle; MAX_PARTICLES as usize] {
    let mut particles = [Particle::new(); MAX_PARTICLES as usize];
    let mut rng = rand::thread_rng();
    let colors = create_particle_colors(n_types);

    for i in 0..n_types {
        let color = colors[i as usize];
        for j in 0..n_per_type {
            particles[(i * n_per_type + j) as usize] = Particle {
                pos: [rng.gen_range(0f32..(TEXTURE_SIZE.0 as f32 / TEXTURE_SIZE.1 as f32)), rng.gen_range(0f32..1f32)],
                vel: [0.0, 0.0],
                color,
                type_idx: i,
            };
        }
    };

    return particles;
}


fn create_hexagon_data(mut radius: f32) -> ([f32; 12], [u32; 12]) {
    radius /= 100.0;
    let half_r: f32 = radius / 2.0;
    let apothem: f32 = 1.73205081 * half_r;

    let vertices: [f32; 12] = [
        -half_r, apothem, half_r, apothem,
        radius, 0.0, half_r, -apothem,
        -half_r, -apothem, -radius, 0.0,
    ];

    let indices: [u32; 12] = [
        0, 2, 1, 
        0, 3, 2,
        0, 4, 3, 
        0, 5, 4,
    ];

    (vertices, indices)
}

pub fn write_particles_buffer(
    mut particles_buf: ResMut<ParticlesBuffer>,
    ui_settings: Res<UISettings>,
    render_device: Res<RenderDevice>,
) {
    if ui_settings.particle_count_changed || ui_settings.just_reset {
        let particles = create_particles(ui_settings.num_particle_types, ui_settings.num_particles_per_type);
        particles_buf.storage = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&particles),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        });
    }
}

pub fn write_vertex_buffer(
    mut particles_buf: ResMut<ParticlesBuffer>,
    ui_settings: Res<UISettings>,
    render_device: Res<RenderDevice>,
) {
    if ui_settings.particle_size_changed {
        let (vertices, _indices) = create_hexagon_data(ui_settings.particle_size);
        particles_buf.vertex_data = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
    }
}