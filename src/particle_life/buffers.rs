use bevy::{prelude::*, render::{render_resource::{ShaderType, Buffer, BufferDescriptor, BufferUsages, BufferInitDescriptor}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};
use rand::Rng;

// use crate::particle_life::TEXTURE_SIZE;

use crate::particle_life::TEXTURE_SIZE;

use super::{NUM_PARTICLES, NUM_PARTICLES_PER_TYPE, NUM_PARTICLE_TYPES};


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
        let size = (NUM_PARTICLES * std::mem::size_of::<f32>() as u32 * 8) as u64;
        let particles = create_particles();
        
        let staging = device.create_buffer(&BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let storage = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&particles),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        });

        let (vertices, indices) = create_hexagon_data();
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


fn create_particles() -> [Particle; NUM_PARTICLES as usize] {
    const COLOR_A: Vec3 = Vec3::new(0.5, 0.5, 0.5);
    const COLOR_B: Vec3 = Vec3::new(0.5, 0.5, 0.5);
    const COLOR_C: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    const COLOR_D: Vec3 = Vec3::new(0.0, 0.333, 0.667);

    let mut particles = [Particle::new(); NUM_PARTICLES as usize];
    let mut rng = rand::thread_rng();

    for i in 0..NUM_PARTICLE_TYPES {
        let t = i as f32 / NUM_PARTICLE_TYPES as f32;
        let c1 = std::f32::consts::TAU * (COLOR_C * t + COLOR_D);
        let c1cosx = c1.x.cos();
        let c1cosy = c1.y.cos();
        let c1cosz = c1.z.cos();
        let color = COLOR_A + COLOR_B * Vec3::new(c1cosx, c1cosy, c1cosz);
        for j in 0..NUM_PARTICLES_PER_TYPE {
            particles[(i * NUM_PARTICLES_PER_TYPE + j) as usize] = Particle {
                pos: [rng.gen_range(0f32..(TEXTURE_SIZE.0 as f32 / TEXTURE_SIZE.1 as f32)), rng.gen_range(0f32..1f32)],
                vel: [0.0, 0.0],
                color: [color.x, color.y, color.z],
                type_idx: i,
            };
        }
    };

    return particles;
}


fn create_hexagon_data() -> (&'static [f32], &'static [u32]) {
    const RADIUS: f32 = 0.01;
    const HALF_R: f32 = RADIUS / 2.0;
    const APOTHEM: f32 = 1.73205081 * HALF_R;

    static VERTICES: [f32; 12] = [
        -HALF_R, APOTHEM, HALF_R, APOTHEM,
        RADIUS, 0.0, HALF_R, -APOTHEM,
        -HALF_R, -APOTHEM, -RADIUS, 0.0,
    ];

    static INDICES: [u32; 12] = [
        0, 2, 1, 
        0, 3, 2,
        0, 4, 3, 
        0, 5, 4,
    ];

    (&VERTICES, &INDICES)
}