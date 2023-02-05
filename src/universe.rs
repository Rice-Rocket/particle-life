use macroquad::{prelude::*};
extern crate rand;
use rand::distributions::{Distribution, Uniform};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Write, io::Read};

#[path = "particle.rs"] mod particle;
use self::particle::Particle;


pub struct UniverseParams {
    pub n_types: usize,
    pub particle_per_type: u128,
    pub attract_mean: f64,
    pub attract_std: f64,
    pub min_r_lower: f64,
    pub min_r_upper: f64,
    pub max_r_lower: f64,
    pub max_r_upper: f64,
    pub friction: f64,
    pub speed: f64,
    pub flat_force: bool,
    pub wrap: bool
}

impl Default for UniverseParams {
    fn default() -> Self {
        Self {
            n_types: 5,
            particle_per_type: 50,
            attract_mean: -0.05,
            attract_std: 0.05,
            min_r_lower: 0.5,
            min_r_upper: 1.0,
            max_r_lower: -10.0,
            max_r_upper: 10.0,
            friction: 0.1,
            speed: 1.0,
            flat_force: false,
            wrap: false
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct SimulationSettings {
    n_types: usize,
    particle_per_type: u128,
    attraction_matrix: Vec<Vec<f64>>,
    min_r_matrix: Vec<Vec<f64>>,
    max_r_matrix: Vec<Vec<f64>>,
    friction: f64
}


pub struct Universe {
    n_types: usize,
    particle_per_type: u128,
    attract_mean: f64,
    attract_std: f64,
    min_r_lower: f64,
    min_r_upper: f64,
    max_r_lower: f64,
    max_r_upper: f64,
    friction: f64,
    speed: f64,
    flat_force: bool,
    wrap: bool,

    particles: Vec<Particle>,
    width: f32,
    height: f32,

    attraction_matrix: Vec<Vec<f64>>,
    min_r_matrix: Vec<Vec<f64>>,
    max_r_matrix: Vec<Vec<f64>>
}


impl Universe {
    pub fn new() -> Self {
        Self {
            n_types: 1,
            particle_per_type: 100,
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

            particles: Vec::new(),
            width: screen_width(),
            height: screen_height(),
            attraction_matrix: Vec::new(),
            min_r_matrix: Vec::new(),
            max_r_matrix: Vec::new(),
        }
    }
    fn construct_matrices(&mut self) {
        self.attraction_matrix.clear();
        self.min_r_matrix.clear();
        self.max_r_matrix.clear();

        let attract_range = Uniform::from(self.attract_mean..self.attract_std);
        let min_r_range = Uniform::from(self.min_r_lower..self.min_r_upper);
        let max_r_range = Uniform::from(self.max_r_lower..self.max_r_upper);
        let mut rng = rand::thread_rng();

        for _ in 0..self.n_types {
            let mut attraction_row: Vec<f64> = Vec::new();
            let mut min_r_row: Vec<f64> = Vec::new();
            let mut max_r_row: Vec<f64> = Vec::new();
            for _ in 0..self.n_types {
                attraction_row.push(attract_range.sample(&mut rng));
                min_r_row.push(min_r_range.sample(&mut rng));
                max_r_row.push(max_r_range.sample(&mut rng));
            }
            self.attraction_matrix.push(attraction_row);
            self.min_r_matrix.push(min_r_row);
            self.max_r_matrix.push(max_r_row)
        }
    }
    pub fn init_particle(&mut self) {
        self.particles.clear();
        self.construct_matrices();

        let hor_range = Uniform::from(0..self.width as i32);
        let vert_range = Uniform::from(0..self.height as i32);
        let mut rng = rand::thread_rng();
        for i in 0..self.n_types {
            let color_val = Color::from_rgba(rand::random(), rand::random(), rand::random(), 255);
            for _ in 0..self.particle_per_type {
                self.particles.push(Particle::new(color_val, i,
                                        hor_range.sample(&mut rng) as f64,
                                        vert_range.sample(&mut rng) as f64))
            }
        }
    }
    pub fn init(&mut self, params: UniverseParams) {
        self.n_types = params.n_types + 1;
        self.particle_per_type = params.particle_per_type;
        self.attract_mean = params.attract_mean;
        self.attract_std = params.attract_std;
        self.min_r_lower = params.min_r_lower;
        self.min_r_upper = params.min_r_upper;
        self.max_r_lower = params.max_r_lower;
        self.max_r_upper = params.max_r_upper;
        self.friction = params.friction;
        self.speed = params.speed;
        self.flat_force = params.flat_force;
        self.wrap = params.wrap;

        self.init_particle();
    }
    pub fn step(&mut self, dt: f64) {
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i == j {
                    continue
                }
                let f: f64;
                let mut dx: f64;
                let mut dy: f64;
                {
                    let a = &self.particles[i];
                    let b = &self.particles[j];
                    dx = b.x - a.x;
                    dy = b.y - a.y;

                    if self.wrap {
                        dx -= self.width as f64 * (dx / self.width as f64).round();
                        dy -= self.height as f64 * (dy / self.height as f64).round();
                    }

                    let r2 = dx.powi(2) + dy.powi(2);
                    let min_r = &self.min_r_matrix[a.ptype][b.ptype];
                    let max_r = &self.max_r_matrix[a.ptype][b.ptype];

                    if (r2 > max_r.powi(2)) || (r2 < 0.01) {
                        continue
                    };

                    let r = r2.sqrt();
                    dx /= r;
                    dy /= r;

                    if r > *min_r {
                        let numer = 2.0 * (r - 0.5 * (max_r + min_r)).abs();
                        let denom = max_r - min_r;
                        f = self.attraction_matrix[a.ptype][b.ptype] * (1.0 - numer / denom);
                    }
                    else {
                        let r_smooth = 2.0;
                        f = r_smooth * min_r * (1.0 / (min_r + r_smooth) - 1.0 / (r + r_smooth));
                    }
                }
                self.particles[i].vx += f * dx * self.speed;
                self.particles[i].vy += f * dy * self.speed;
            }
        }

        for particle in self.particles.iter_mut() {
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;

            particle.vx *= 1.0 - self.friction;
            particle.vy *= 1.0 - self.friction;

            match self.wrap {
                false => {
                    if (particle.x >= self.width as f64) || (particle.x < 0f64) {
                        particle.vx *= -1.5;
                    }
                    if (particle.y >= self.height as f64) || (particle.y < 0f64) {
                        particle.vy *= -1.5;
                    }
                },
                true => {
                    if (particle.x >= self.width as f64) || (particle.x < 0f64) {
                        particle.x = (particle.x - self.width as f64).abs();
                    }
                    if (particle.y >= self.height as f64) || (particle.y < 0f64) {
                        particle.y = (particle.y - self.height as f64).abs();
                    }
                }
            }
        }
    }
    pub fn save_settings(&self, file_path: &str) {
        let settings = SimulationSettings{
            n_types: self.n_types,
            particle_per_type: self.particle_per_type,
            attraction_matrix: self.attraction_matrix.clone(),
            min_r_matrix: self.min_r_matrix.clone(),
            max_r_matrix: self.max_r_matrix.clone(),
            friction: self.friction
        };
        let save_text = serde_json::to_string(&settings).unwrap();

        let mut file = File::create(file_path).unwrap();
        file.write_all(save_text.as_bytes()).unwrap();
    }
    pub fn load_settings(&mut self, file_path: &str) {
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let settings: SimulationSettings = serde_json::from_str(&contents).unwrap();

        self.n_types = settings.n_types;
        self.particle_per_type = settings.particle_per_type;
        self.attraction_matrix = settings.attraction_matrix;
        self.min_r_matrix = settings.min_r_matrix;
        self.max_r_matrix = settings.max_r_matrix;
        self.friction = settings.friction;

        self.init_particle();
    }
    pub fn draw(&self) {
        for particle in self.particles.iter() {
            draw_circle(particle.x as f32, particle.y as f32, 2.0, particle.color);
        }
    }
}