use std::borrow::Cow;

use bevy::{prelude::*, render::{render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, CachedComputePipelineId, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, TextureFormat, BufferBindingType, PipelineCache, ComputePipelineDescriptor, CachedPipelineState, ComputePassDescriptor, VertexState, VertexBufferLayout, VertexStepMode, VertexAttribute, VertexFormat, RenderPipelineDescriptor, FragmentState, PrimitiveState, MultisampleState, ColorTargetState, ColorWrites, CachedRenderPipelineId, RenderPassDescriptor, RenderPassColorAttachment, Operations, IndexFormat}, render_asset::RenderAssets, renderer::{RenderDevice, RenderContext}, render_graph, texture::BevyDefault}};

use super::{MAX_PARTICLES, WORKGROUP_SIZE, texture::ParticleLifeImage, buffers::ParticlesBuffer, ui::UISettings, settings::SettingsBuffer};


#[derive(Resource)]
struct ParticleLifeBindGroups(BindGroup, BindGroup, BindGroup);

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<ParticleLifePipeline>,
    particle_life_particle_buf: Res<ParticlesBuffer>,
    particle_life_settings: Res<SettingsBuffer>,
    render_device: Res<RenderDevice>,
) {
    let bind_group_buf = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.particle_buf_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: particle_life_particle_buf.storage.as_entire_binding(),
        }],
    });
    let bind_group_settings = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.settings_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: particle_life_settings.settings.binding().unwrap(),
        }, BindGroupEntry {
            binding: 1,
            resource: particle_life_settings.attraction_tables.binding().unwrap(),
        }],
    });
    let bind_group_draw = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.render_layout,
        entries: &[BindGroupEntry {
            binding: 1,
            resource: particle_life_settings.aspect_ratio.binding().unwrap(),
        }]
    });
    commands.insert_resource(ParticleLifeBindGroups(bind_group_buf, bind_group_settings, bind_group_draw));
}

#[derive(Resource)]
pub struct ParticleLifePipeline {
    particle_buf_bind_group_layout: BindGroupLayout,
    settings_bind_group_layout: BindGroupLayout,
    render_layout: BindGroupLayout,
    // init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
    render_pipeline: CachedRenderPipelineId,
}

impl FromWorld for ParticleLifePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let particle_buf_bind_group_layout = 
            render_device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage {
                                read_only: false,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            // min_binding_size: BufferSize::new((MAX_PARTICLES * std::mem::size_of::<f32>() as u32 * 8) as u64),
                        },
                        count: None,
                    }]
                });
        let settings_bind_group_layout = 
            render_device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }, BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage {
                                read_only: true,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }]
                });
        let render_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    // binding: 0,
                    // visibility: ShaderStages::FRAGMENT,
                    // ty: BindingType::Texture {
                    //     sample_type: TextureSampleType::Float { filterable: true },
                    //     view_dimension: TextureViewDimension::D2,
                    //     multisampled: false,
                    // },
                    // count: None,
                // }, BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });
        let compute_shader = world
            .resource::<AssetServer>()
            .load("shaders/particle_life.wgsl");
        let draw_shader = world
            .resource::<AssetServer>()
            .load("shaders/draw.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let render_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: None,
            layout: vec![render_layout.clone()],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: draw_shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("main_vs"),
                buffers: vec![VertexBufferLayout {
                    array_stride: 8 * 4,
                    step_mode: VertexStepMode::Instance,
                    attributes: vec![VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }, VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: std::mem::size_of::<[f32; 2]>() as u64,
                        shader_location: 1,
                    }, VertexAttribute {
                        format: VertexFormat::Float32x3,
                        offset: std::mem::size_of::<[f32; 4]>() as u64,
                        shader_location: 2,
                    }, VertexAttribute {
                        format: VertexFormat::Float32,
                        offset: std::mem::size_of::<[f32; 7]>() as u64,
                        shader_location: 3,
                    }]
                }, VertexBufferLayout {
                    array_stride: 2 * 4,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vec![VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 4,
                    }]
                }],
            },
            fragment: Some(FragmentState {
                shader: draw_shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("main_fs"),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })]
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![particle_buf_bind_group_layout.clone(), settings_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: compute_shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        ParticleLifePipeline {
            particle_buf_bind_group_layout,
            settings_bind_group_layout,
            render_layout,
            update_pipeline,
            render_pipeline,
        }
    }
}

enum ParticleLifeState {
    Init,
    Waiting,
    Update,
}

pub struct ParticleLifeNode {
    state: ParticleLifeState,
}

impl Default for ParticleLifeNode {
    fn default() -> Self {
        Self {
            state: ParticleLifeState::Init,
        }
    }
}

impl render_graph::Node for ParticleLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<ParticleLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            ParticleLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = ParticleLifeState::Waiting;
                }
            }
            ParticleLifeState::Waiting => {
                if let Some(ui_settings) = world.get_resource::<UISettings>() {
                    if ui_settings.running {
                        self.state = ParticleLifeState::Update;
                    }
                }
            }
            ParticleLifeState::Update => {
                if let Some(ui_settings) = world.get_resource::<UISettings>() {
                    if !ui_settings.running {
                        self.state = ParticleLifeState::Waiting;
                    }
                }
            }
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let particles_buf_bind_group = &world.resource::<ParticleLifeBindGroups>().0;
        let settings_bind_group = &world.resource::<ParticleLifeBindGroups>().1;
        let aspect_ratio_bind_group = &world.resource::<ParticleLifeBindGroups>().2;
        let particles_buf = &world.resource::<ParticlesBuffer>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ParticleLifePipeline>();

        let encoder = render_context.command_encoder();
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());

            compute_pass.set_bind_group(0, particles_buf_bind_group, &[]);
            compute_pass.set_bind_group(1, settings_bind_group, &[]);

            match self.state {
                ParticleLifeState::Init => {}
                ParticleLifeState::Waiting => {}
                ParticleLifeState::Update => {
                    let update_particles_pipeline = pipeline_cache
                        .get_compute_pipeline(pipeline.update_pipeline)
                        .unwrap();
                    compute_pass.set_pipeline(update_particles_pipeline);
                    compute_pass.dispatch_workgroups(MAX_PARTICLES / WORKGROUP_SIZE, 1, 1);
                }
            }
        }

        encoder.copy_buffer_to_buffer(&particles_buf.storage, 0, &particles_buf.staging, 0, particles_buf.size);

        {
            let gpu_images = world.resource::<RenderAssets<Image>>();
            let particle_life_image = world.resource::<ParticleLifeImage>();
            let view = &gpu_images[&particle_life_image.0];
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view.texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: Default::default(),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_bind_group(0, &aspect_ratio_bind_group, &[]);
            
            match self.state {
                ParticleLifeState::Update | ParticleLifeState::Waiting => {
                    let render_pipeline = pipeline_cache
                        .get_render_pipeline(pipeline.render_pipeline)
                        .unwrap();
                    render_pass.set_pipeline(render_pipeline);
                    render_pass.set_vertex_buffer(0, *particles_buf.staging.slice(..));
                    render_pass.set_vertex_buffer(1, *particles_buf.vertex_data.slice(..));
                    render_pass.set_index_buffer(*particles_buf.index_data.slice(..), IndexFormat::Uint32);
                    render_pass.draw_indexed(0..12, 0, 0..MAX_PARTICLES);
                },
                _ => ()
            }
        }

        Ok(())
    }
}