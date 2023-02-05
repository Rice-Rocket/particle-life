// use macroquad::prelude::*;
// use std::default::Default;



// pub struct Button {
//     pub text: String,
//     pub position: (f32, f32),
//     pub width: f32,
//     pub height: f32,
//     pub border: f64,
//     pub temp: Color,
//     pub color: Color,
//     pub border_color: Color,
//     pub font_size: u16,
//     pub text_color: Color,
//     pub hover_color: Color,
//     pub state: bool
// }

// impl Default for Button {
//     fn default() -> Self {
//         Self {
//             text: "".to_string(),
//             position: (0.0, 0.0),
//             width: 100.0,
//             height: 50.0,
//             border: 0.0,
//             temp: Color::new(0.95, 0.93, 0.90, 1.0),
//             color: Color::new(0.95, 0.93, 0.90, 1.0),
//             border_color: Color::new(0.95, 0.93, 0.90, 1.0),
//             font_size: 25,
//             text_color: Color::new(0.27, 0.2, 0.15, 1.0),
//             hover_color: Color::new(0.93, 0.8, 0.58, 1.0),
//             state: false
//         }
//     }
// }


// pub struct Panel {
//     pub position: (f32, f32),
//     pub width: f32,
//     pub height: f32,
//     pub color: Color,
//     pub alpha: f64
// }

// impl Default for Panel {
//     fn default() -> Self {
//         Self {
//             position: (0.0, 0.0),
//             width: 100.0,
//             height: 50.0,
//             color: Color::new(0.95, 0.93, 0.90, 1.0),
//             alpha: 0.5
//         }
//     }
// }




// pub struct ToggleButton {
//     pub position: (f32, f32),
//     pub width: f32,
//     pub height: f32,
//     pub color: Color,
//     pub active_color: Color,
//     pub hover_color: Color,
//     pub temp: (Color, Color),
//     pub clicked: bool,
//     pub state: bool
// }

// impl Default for ToggleButton {
//     fn default() -> Self {
//         Self {
//             position: (0.0, 0.0),
//             width: 100.0,
//             height: 50.0,
//             temp: (Color::new(0.76, 0.5, 0.25, 1.0), Color::new(0.95, 0.93, 0.90, 1.0)),
//             color: Color::new(0.95, 0.93, 0.90, 1.0),
//             active_color: Color::new(0.76, 0.5, 0.25, 1.0),
//             hover_color: Color::new(0.93, 0.8, 0.58, 1.0),
//             clicked: false,
//             state: false
//         }
//     }
// }





// pub struct Text {
//     pub text: String,
//     pub position: (f32, f32),
//     pub color: Color,
//     pub font_size: u16
// }

// impl Default for Text {
//     fn default() -> Self {
//         Self {
//             text: "".to_string(),
//             position: (0.0, 0.0),
//             color: Color::new(0.53, 0.49, 0.48, 1.0),
//             font_size: 18,
//         }
//     }
// }




// pub struct DigitInput {
//     pub start_val: i32,
//     pub position: (f32, f32),
//     pub width: f32,
//     pub height: f32,
//     pub color: Color,
//     pub font_size: u16,
//     pub text_color: Color,
//     pub percentage: bool,
//     pub hover_enter: bool
// }

// impl Default for DigitInput {
//     fn default() -> Self {
//         Self {
//             start_val: 0,
//             position: (0.0, 0.0),
//             width: 100.0,
//             height: 50.0,
//             color: Color::new(0.95, 0.93, 0.90, 1.0),
//             font_size: 25,
//             text_color: Color::new(0.27, 0.2, 0.15, 1.0),
//             percentage: false,
//             hover_enter: false
//         }
//     }
// }




// pub struct Slider {
//     pub position: (f32, f32),
//     pub width: f32,
//     pub height: f32,
//     pub line_color: Color,
//     pub active_color: Color,
//     pub nub_color: Color,

//     pub start_val: i32,
//     pub min_val: i32, 
//     pub max_val: i32,
//     pub v: f64,

//     pub rect_width: i32,
//     pub hightlight_width: i32,
//     pub temp_width: i32
// }

// impl Default for Slider {
//     fn default() -> Self {
//         Self {
//             position: (0.0, 0.0),
//             width: 100.0,
//             height: 50.0,
//             line_color: Color::new(0.3, 0.29, 0.3, 1.0),
//             active_color: Color::new(0.97, 0.96, 0.97, 1.0),
//             nub_color: Color::new(0.76, 0.75, 0.76, 1.0),

//             start_val: 50,
//             min_val: 0,
//             max_val: 100,
//             v: 0.5,

//             rect_width: 8,
//             hightlight_width: 10,
//             temp_width: 8
//         }
//     }
// }





// impl Button {
//     fn input(&mut self) {
//         let m = mouse_position();
//         self.state = false;
//         if (m.0 >= self.position.0) && (m.0 <= self.position.0 + self.width) {
//             if (m.1 >= self.position.1) && (m.1 <= self.position.1 + self.height) {
//                 self.color = self.hover_color;
//                 if is_mouse_button_pressed(MouseButton::Left) {
//                     self.color = Color::new(0.78, 0.78, 0.78, 1.0);
//                     self.state = true;
//                 };
//             }
//             else {
//                 self.color = self.temp
//             }
//         }
//         else {
//             self.color = self.temp
//         }
//     }
//     pub fn draw(&mut self, text_font: Font) {
//         self.input();
//         let text = format!("{}", self.text);
//         let dims = measure_text(&text, Some(text_font), self.font_size, 1.0);
//         draw_rectangle(self.position.0, self.position.1, self.width, self.height, self.color);
//         draw_text_ex(
//             &text,
//             self.position.0 + self.width / 2f32 - dims.width / 2f32,
//             self.position.1 + self.height / 2f32 - dims.height / 2f32,
//             TextParams{font: text_font, font_size: self.font_size, color: self.text_color, ..Default::default()}
//         )
//     }
// }