use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rand::{Rng, thread_rng};

use super::{INIT_NUM_TYPES, INIT_NUM_PARTICLES_PER_TYPE, MAX_PARTICLE_TYPES, buffers::create_particle_colors, MAX_PARTICLES_PER_TYPE};


#[derive(Resource, Default, PartialEq, Clone)]
pub enum UIVisibility {
    #[default]
    Visible, 
    Hidden,
}

#[derive(Resource, Clone)]
pub struct UISettings {
    pub num_particle_types: u32,
    pub num_particles_per_type: u32,

    pub attraction_table: [f32; (MAX_PARTICLE_TYPES * MAX_PARTICLE_TYPES) as usize],
    pub ptype_colors: [[f32; 3]; MAX_PARTICLE_TYPES as usize],

    pub particle_size: f32,

    pub min_r: f32,
    pub max_r: f32,
    pub friction_half_time: f32,
    pub speed: f32,
    pub wrap: bool,

    pub just_started: bool,
    pub just_reset: bool,
    pub particle_size_changed: bool,
    pub particle_count_changed: bool,
    pub running: bool,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            num_particle_types: INIT_NUM_TYPES,
            num_particles_per_type: INIT_NUM_PARTICLES_PER_TYPE,

            attraction_table: [0.0; (MAX_PARTICLE_TYPES * MAX_PARTICLE_TYPES) as usize],
            ptype_colors: [[1.0, 0.25090736, 0.25090742]; MAX_PARTICLE_TYPES as usize],

            particle_size: 1.0,

            min_r: 0.3,
            max_r: 0.3,
            friction_half_time: 0.1,
            speed: 1.0,
            wrap: true,

            just_started: false,
            just_reset: false,
            particle_size_changed: false,
            particle_count_changed: false,
            running: false,
        }
    }
}


pub fn ui_render_update(
    mut contexts: EguiContexts,
    ui_visibility: Res<UIVisibility>,
    mut settings: ResMut<UISettings>,
) {
    settings.particle_size_changed = false;
    if ui_visibility.clone() == UIVisibility::Hidden { return; }

    egui::Window::new("Render Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Particle Size:");
            let prev_particle_size = settings.particle_size;
            ui.add(egui::widgets::DragValue::new(&mut settings.particle_size).speed(0.25).clamp_range(0.2f32..=100f32));
            if prev_particle_size != settings.particle_size {
                settings.particle_size_changed = true;
            }
        });
    });
}

pub fn ui_particles_update(
    mut contexts: EguiContexts,
    ui_visibility: Res<UIVisibility>,
    mut settings: ResMut<UISettings>,
) {
    settings.particle_count_changed = false;
    if ui_visibility.clone() == UIVisibility::Hidden { return; }

    egui::Window::new("Particle Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Types:");
            if ui.small_button("-").clicked() && settings.num_particle_types > 1 {
                let old_n_types = settings.num_particle_types;
                settings.num_particle_types -= 1;
                let new_n_types = settings.num_particle_types;
                adjust_attraction_table(&mut settings.attraction_table, old_n_types, new_n_types);
                settings.ptype_colors = create_particle_colors(settings.num_particle_types);
                settings.particle_count_changed = true;
            }
            ui.label(format!("{}", settings.num_particle_types));
            if ui.small_button("+").clicked() && settings.num_particle_types < MAX_PARTICLE_TYPES {
                let old_n_types = settings.num_particle_types;
                settings.num_particle_types += 1;
                let new_n_types = settings.num_particle_types;
                adjust_attraction_table(&mut settings.attraction_table, old_n_types, new_n_types);
                settings.ptype_colors = create_particle_colors(settings.num_particle_types);
                settings.particle_count_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Particles Per Type:");
            let prev_particles_per_type = settings.num_particles_per_type;
            ui.add(egui::widgets::DragValue::new(&mut settings.num_particles_per_type).clamp_range(0..=MAX_PARTICLES_PER_TYPE));
            if prev_particles_per_type != settings.num_particles_per_type {
                settings.particle_count_changed = true;
            }
        });

        for i in 0..(settings.num_particle_types + 1) {
            ui.horizontal(|ui| {
                for j in 0..(settings.num_particle_types + 1) {
                    if i == 0 && j == 0 {
                        let col = egui::Color32::DARK_GRAY;
                        egui::color_picker::show_color(ui, col, ui.spacing().interact_size);
                        continue;
                    }
                    if i == 0 {
                        let col = settings.ptype_colors[j as usize - 1];
                        let colrgb = egui::Color32::from_rgb((col[0] * 255.0) as u8, (col[1] * 255.0) as u8, (col[2] * 255.0) as u8);
                        egui::color_picker::show_color(ui, colrgb, ui.spacing().interact_size).on_hover_ui(|ui| {
                            ui.label("The Attractor");
                        });
                        continue;
                    }
                    if j == 0 {
                        let col = settings.ptype_colors[i as usize - 1];
                        let colrgb = egui::Color32::from_rgb((col[0] * 255.0) as u8, (col[1] * 255.0) as u8, (col[2] * 255.0) as u8);
                        egui::color_picker::show_color(ui, colrgb, ui.spacing().interact_size).on_hover_ui(|ui| {
                            ui.label("The Attracted");
                        });
                        continue;
                    }
                    
                    let idx = ((i - 1) * settings.num_particle_types + (j - 1)) as usize;
                    ui.add(egui::widgets::DragValue::new(&mut settings.attraction_table[idx])
                        .clamp_range(-1f32..=1f32).speed(0.05).min_decimals(1));
                }
            });
        }

        if ui.button("Randomize Attraction Table").clicked() {
            let mut rng = thread_rng();
            for i in 0..settings.num_particle_types {
                for j in 0..settings.num_particle_types {
                    let idx = (i * settings.num_particle_types + j) as usize;
                    settings.attraction_table[idx] = rng.gen_range(-1f32..=1f32);
                }
            }
        }
    });
}

pub fn ui_update(
    mut contexts: EguiContexts,
    mut ui_visibility: ResMut<UIVisibility>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut settings: ResMut<UISettings>,
) {
    settings.just_started = false;
    settings.just_reset = false;
    if keyboard.just_pressed(KeyCode::Tab) {
        *ui_visibility = match ui_visibility.clone() {
            UIVisibility::Visible => UIVisibility::Hidden,
            UIVisibility::Hidden => UIVisibility::Visible,
        }
    }
    if ui_visibility.clone() == UIVisibility::Hidden { return; }

    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.1}", 1.0 / time.delta_seconds()));
        ui.label("Press [TAB] to Toggle UI");

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Repulsor Distance:");
            ui.add(egui::widgets::DragValue::new(&mut settings.min_r).clamp_range(0f32..=1f32).speed(0.025).min_decimals(2));
        });
        ui.horizontal(|ui| {
            ui.label("Max Force Distance:");
            ui.add(egui::widgets::DragValue::new(&mut settings.max_r).clamp_range(0f32..=1f32).speed(0.025).min_decimals(2));
        });
        ui.horizontal(|ui| {
            ui.label("Friction Half Time:");
            ui.add(egui::widgets::DragValue::new(&mut settings.friction_half_time).clamp_range(0f32..=1f32).speed(0.025).min_decimals(2));
        });
        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::widgets::DragValue::new(&mut settings.speed).speed(0.25));
        });
        ui.horizontal(|ui| {
            ui.label("Wrap:");
            ui.add(egui::widgets::Checkbox::new(&mut settings.wrap, ""));
        });

        ui.separator();

        ui.horizontal(|ui| {
            let button_text = match settings.running {
                true => "Pause Simulation",
                false => "Run Simulation",
            };
            if ui.button(button_text).clicked() {
                settings.running = !settings.running;
                if settings.running {
                    settings.just_started = true;
                }
            }

            if ui.button("Reset").clicked() {
                settings.just_reset = true;
            }
        });
    });
}

fn adjust_attraction_table(table: &mut [f32; (MAX_PARTICLE_TYPES * MAX_PARTICLE_TYPES) as usize], old_n_types: u32, new_n_types: u32) {
    let mut new = [0.0; (MAX_PARTICLE_TYPES * MAX_PARTICLE_TYPES) as usize];
    for i in 0..old_n_types {
        for j in 0..old_n_types {
            let old_idx = (i * old_n_types + j) as usize;
            let new_idx = (i * new_n_types + j) as usize;
            let old_val = table[old_idx];
            new[new_idx] = old_val;
        }
    }
    *table = new;
}