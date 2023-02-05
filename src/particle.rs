use macroquad::prelude::*;



pub struct Particle {
    pub color: Color,
    pub ptype: usize,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64
}


impl Particle {
    pub fn new(color_val: Color, part_type: usize, x_pos: f64, y_pos: f64) -> Self {
        Self {
            color: color_val,
            ptype: part_type,
            x: x_pos,
            y: y_pos,
            vx: 0.0,
            vy: 0.0
        }
    }
}
