use bevy::{prelude::*, render::{extract_resource::ExtractResourcePlugin, RenderApp, Render, render_graph::RenderGraph, RenderSet}};

use self::{texture::{ParticleLifeImage, setup_texture}, buffers::ParticlesBuffer, compute::{queue_bind_group, ParticleLifeNode, ParticleLifePipeline}, ui::UISettings, settings::{SettingsBuffer, extract_time, extract_ui_settings, prepare_settings_buffer}};

pub mod compute;
pub mod texture;
pub mod buffers;
pub mod settings;
pub mod ui;


pub const NUM_PARTICLE_TYPES: u32 = 2;
pub const NUM_PARTICLES_PER_TYPE: u32 = 32;
pub const NUM_PARTICLES: u32 = NUM_PARTICLE_TYPES * NUM_PARTICLES_PER_TYPE;

pub const TEXTURE_SIZE: (u32, u32) = (1280, 720);
// pub const TEXTURE_SIZE: (u32, u32) = (640, 360);
pub const WORKGROUP_SIZE: u32 = 64;
pub const INITIAL_STATE: &str = "initAgentsInwardRing";


#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum SimulationState {
    #[default]
    Uninitialized,
    Started,
}


pub struct ParticleLifeComputePlugin;

impl Plugin for ParticleLifeComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>();
        app.add_systems(Startup, setup_texture);
        app.add_plugins(ExtractResourcePlugin::<ParticleLifeImage>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<SettingsBuffer>()
            .init_resource::<Time>()
            .init_resource::<UISettings>()
            .add_state::<SimulationState>()
            .add_systems(ExtractSchedule, (extract_time, extract_ui_settings))
            .add_systems(Render, prepare_settings_buffer.in_set(RenderSet::Prepare))
            .add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));
        
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("particle_life", ParticleLifeNode::default());
        render_graph.add_node_edge(
            "particle_life",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }
    
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ParticlesBuffer>();
        render_app.init_resource::<ParticleLifePipeline>();
    }
}