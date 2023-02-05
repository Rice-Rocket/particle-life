use std::sync::Arc;

use macroquad::{prelude::*};
use ndarray;
extern crate rand;
use rand::Rng;


#[path = "particle.rs"] mod particle;
use self::particle::Particle;



pub struct World {
    particles: Vec<Particle>,
    groups: Vec<Vec<usize>>,
    pub attractions: ndarray::Array2<f64>,
    // colors: Vec<Color>,

    repel_strength: f64,
    speed: f64,
    particle_size: f64,
    r_max: f64,
    force_mult: f64,
    friction: f64,
    bounce_mult: f64,
    field_w: f64,
    field_h: f64
}


impl World {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            groups: Vec::new(),
            attractions: ndarray::Array2::from_elem((1, 1), 0.5f64),
            // colors: Vec::from(color_vals),

            repel_strength: 20.0,
            speed: 1000.0,
            particle_size: 3.0,
            r_max: 200.0,
            force_mult: 2.0,
            friction: 0.5,
            bounce_mult: 0.25,
            field_w: screen_width() as f64,
            field_h: screen_height() as f64
        }
    }
    pub fn reset(&mut self) {
        self.particles.clear();
        self.groups.clear();
        self.attractions = ndarray::Array2::from_elem((1, 1), 0.5f64);
    }
    pub fn add_particle(&mut self, color: Color, x: f64, y: f64) -> Particle {
        return Particle::new(color, x, y);
    }
    pub fn create(&mut self, color: Color, n_particles: i32, start: (f64, f64), end: (f64, f64)) {
        let mut rng = rand::thread_rng();
        let mut group: Vec<usize> = Vec::new();
        for _ in 0..n_particles {
            let x = rng.gen_range(start.0..end.0);
            let y = rng.gen_range(start.1..end.1);
            let particle = self.add_particle(color, x, y);
            self.particles.push(particle);
            group.push(self.particles.len() as usize - 1);
        }
        self.groups.push(group);
        // self.attractions.insert_axis(ndarray::Axis(0));
        // self.attractions.insert_axis(ndarray::Axis(1));
    }
    pub fn step(&mut self, dt: f64) {
        let r_min = self.particle_size * 5f64;
        let r_mean = (self.r_max + self.particle_size * 5f64) / 2f64;
        let repel_strength = self.repel_strength * self.speed;
        let force_strength = self.force_mult * self.speed;
        let hitbox_size = self.particle_size * 2f64;

        for i in 0..self.attractions.nrows() {
            for j in 0..self.attractions.ncols() {
                let color1 = &self.groups[i];
                let color2 = &self.groups[j];
                let g = self.attractions.get((j, i)).unwrap();

                for k in color1.iter() {
                    let (mut fx, mut fy) = (0.0, 0.0);
                    {
                        let a = &self.particles[*k];
                        for l in color2.iter() {
                            let b = &self.particles[*l];
                            let mut dx = a.x - b.x;
                            dx = dx - self.field_w * (dx / self.field_w).round();
                            let mut dy = a.y - b.y;
                            dy = dy - self.field_h * (dy / self.field_h).round();
                            let mag = (dx * dx + dy * dy).sqrt();
    
                            if mag <= 0.0 {
                                continue;
                            }
                            let norm = (dx / mag, dy / mag);
                            if mag > r_mean {
                                if mag < self.r_max {
                                    let f = g * ((self.r_max - mag) / (self.r_max - r_mean));
                                    fx += f * norm.0 * force_strength;
                                    fy += f * norm.1 * force_strength;
                                }
                                else {
                                    continue
                                }
                            }
                            else if mag > r_min {
                                let f = g * ((mag - r_min) / (r_mean - r_min));
                                fx += f * norm.0 * force_strength;
                                fy += f * norm.1 * force_strength;
                            }
                            else {
                                let f = (r_min - mag) / r_min;
                                fx += f * norm.0 * repel_strength;
                                fy += f * norm.1 * repel_strength;
                            }

                            if mag < hitbox_size {
                                fx *= -self.bounce_mult;
                                fy *= -self.bounce_mult;
                            }
                        };
                    };
                    self.particles[*k].vx = (self.particles[*k].vx + fx) * (1.0 - self.friction) * dt;
                    self.particles[*k].vy = (self.particles[*k].vy + fy) * (1.0 - self.friction) * dt;
                }
            };
        };
    }
    pub fn update_pos(&mut self, dt: f64) {
        for p in self.particles.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            
            if p.x >= self.field_w {
                p.x -= self.field_w;
            }
            else if p.x < 0.0 {
                p.x += self.field_w;
            }
            if p.y >= self.field_h {
                p.y -= self.field_h;
            }
            else if p.y < 0.0 {
                p.y += self.field_h;
            }
        }
    }
    pub fn draw(&mut self) {
        for p in self.particles.iter_mut() {
            draw_circle(p.x as f32, p.y as f32, self.particle_size as f32, p.color);
        }
    }
}