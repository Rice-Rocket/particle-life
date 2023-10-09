use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::{INIT_NUM_TYPES, INIT_NUM_PARTICLES_PER_TYPE, MAX_PARTICLE_TYPES, buffers::create_particle_colors};


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

    pub min_r: f32,
    pub max_r: f32,
    pub friction: f32,
    pub speed: f32,
    pub wrap: bool,

    pub just_started: bool,
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

            min_r: 0.3,
            max_r: 0.3,
            friction: 0.9,
            speed: 5.0,
            wrap: true,

            just_started: false,
            particle_count_changed: false,
            running: false,
        }
    }
}


pub fn ui_update(
    mut contexts: EguiContexts,
    mut ui_visibility: ResMut<UIVisibility>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut settings: ResMut<UISettings>,
) {
    settings.just_started = false;
    settings.particle_count_changed = false;
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
            ui.label("Types:");
            if ui.small_button("-").clicked() {
                settings.num_particle_types -= 1;
                settings.ptype_colors = create_particle_colors(settings.num_particle_types);
                settings.particle_count_changed = true;
            }
            ui.label(format!("{}", settings.num_particle_types));
            if ui.small_button("+").clicked() {
                settings.num_particle_types += 1;
                settings.ptype_colors = create_particle_colors(settings.num_particle_types);
                settings.particle_count_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Particles Per Type:");
            let prev_particles_per_type = settings.num_particles_per_type;
            ui.add(egui::widgets::DragValue::new(&mut settings.num_particles_per_type));
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
            ui.label("Friction:");
            ui.add(egui::widgets::DragValue::new(&mut settings.friction).clamp_range(0f32..=1f32).speed(0.025).min_decimals(2));
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
    });
}