mod particle_life;
use bevy_egui::EguiPlugin;
use particle_life::{*, ui::{ui_update, UIVisibility, UISettings, ui_render_update, ui_particles_update}};


#[allow(unused_imports)]
use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice, RenderQueue},
        Render, RenderApp, RenderSet, Extract,
    },
    window::{WindowPlugin, PrimaryWindow},
};



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<UIVisibility>()
        .init_resource::<UISettings>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // uncomment for unthrottled FPS
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    title: String::from("Particle Life"),
                    ..default()
                }),
                ..default()
            }).set(ImagePlugin::default_linear()),
            ParticleLifeComputePlugin,
            EguiPlugin,
        ))
        .add_systems(Update, (ui_update, ui_render_update, ui_particles_update))
        .run();
}


