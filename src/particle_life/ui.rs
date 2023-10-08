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
    pub min_r: f32,
    pub max_r: f32,
    pub friction: f32,
    pub speed: f32,
    pub flat_force: bool,
    pub wrap: bool,

    pub running: bool,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            min_r: 0.3,
            max_r: 0.05,
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