use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};


#[derive(Resource, Default, PartialEq, Clone)]
pub enum UIVisibility {
    #[default]
    Visible, 
    Hidden,
}

#[derive(Resource, Clone)]
pub struct UISettings {
    pub attract_mean: f32,
    pub attract_std: f32,
    pub min_r_lower: f32,
    pub min_r_upper: f32,
    pub max_r_lower: f32,
    pub max_r_upper: f32,
    pub friction: f32,
    pub speed: f32,
    pub flat_force: bool,
    pub wrap: bool,

    pub running: bool,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            attract_mean: -0.05,
            attract_std: 0.05,
            min_r_lower: 0.5,
            min_r_upper: 1.0,
            max_r_lower: -10.0,
            max_r_upper: 10.0,
            friction: 0.1,
            speed: 1.0,
            flat_force: false,
            wrap: false,

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

        let button_text = match settings.running {
            true => "Pause Simulation",
            false => "Run Simulation",
        };
        if ui.button(button_text).clicked() {
            settings.running = !settings.running;
        }
    });
}