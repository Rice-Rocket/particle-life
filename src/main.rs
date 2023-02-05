use macroquad::prelude::*;
mod universe;
use crate::universe::*;

const TEXT_COLOR: Color = Color::new(0.53, 0.49, 0.48, 1.0);




struct App {
    running_sim: bool,
    font: Font,
    universe: Universe,
}

impl App {
    fn new(text_font: Font) -> Self {
        Self {
            running_sim: false,
            font: text_font,
            universe: Universe::new()
        }
    }
    fn init(&mut self) {
        self.universe.init(UniverseParams{
            n_types: 6,
            particle_per_type: 500,
            attract_mean: -0.5,
            attract_std: 0.5,
            min_r_lower: 5.0,
            min_r_upper: 10.0,
            max_r_lower: -100.0,
            max_r_upper: 100.0,
            friction: 0.1,
            speed: 5.0,
            wrap: true,
            ..UniverseParams::default()
        })
    }
    fn update(&mut self, dt: f64) {
        if self.running_sim {
            self.universe.step(dt);
        }
    }
    fn draw(&mut self, fps: i32) {
        clear_background(Color::from_rgba(40, 39, 40, 255));
        self.universe.draw();

        draw_text_ex(
            &format!("FPS: {}", fps),
            10.0,
            20.0,
            TextParams{font: self.font, font_size: 16u16, color: TEXT_COLOR, ..Default::default()}
        );
    }
    fn check_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.running_sim = !self.running_sim
        }
        if is_key_pressed(KeyCode::Enter) {
            self.universe.save_settings("presets/settings.json");
        }
    }
}






#[macroquad::main("Particle Life Simulation")]
async fn main() {
    request_new_screen_size(640., 400.);
    let font = load_ttf_font("assets/Monaco.ttf").await.unwrap();
    let mut app = App::new(font);

    app.init();
    app.universe.load_settings("presets/ecosystem-1.json");
    
    loop {
        let dt = get_frame_time() as f64;
        let fps = get_fps();
        app.check_input();
        app.update(dt);
        app.draw(fps);
        next_frame().await
    }
}
