mod ca;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate fps_counter;
extern crate palette;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::input::Button::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use fps_counter::FPSCounter;
use palette::{Rgb, Hue, IntoColor};
use std::time::{Duration, Instant};

use ca::CA;

pub struct App {
    ca: CA,
    cell_size: u32,
    rule: u8,

    gl: GlGraphics,
    color: Rgb,
    update_timeout: Instant,
    color_timeout: Instant,
    _fps: FPSCounter,
}

impl App {
    fn key_pressed(&mut self, key: &Key) {
        match *key {
            Key::Up => {
                self.rule = (self.rule + 1) % 255;
            },
            Key::Down => {
                self.rule = (self.rule - 1) % 255;
            }
            _ => {}
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let ca_rule = self.ca.rule();
        let exp_width = (args.width / self.cell_size) as usize;
        let width_ok = exp_width == self.ca.width();
        let exp_height = (args.height / self.cell_size) as usize;
        let height_ok = exp_height == self.ca.height();

        if !width_ok || !height_ok || self.rule != ca_rule {
            self.ca = CA::new(exp_width, exp_height, self.rule).unwrap();
            self.ca.fill(exp_width / 2, exp_height - 1);
        }


        let fg: Rgb = self.color.into_hsl().shift_hue(180.0.into()).into_rgb();

        let bg_color: [f32; 4] = [self.color.red, self.color.green, self.color.blue, 1.0];
        let fg_color: [f32; 4] = [fg.red, fg.green, fg.blue, 1.0];

        let pixel = rectangle::square(0.0, 0.0, self.cell_size as f64);

        let cell_size = self.cell_size as usize;
        let grid = &self.ca.grid();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(bg_color, gl);

            for (y, row) in grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if !cell {
                        continue;
                    }

                    let transform = c.transform.trans(
                        (x * cell_size) as f64,
                        (y * cell_size) as f64,
                    );
                    rectangle(fg_color, pixel, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.update_grid();
        self.update_color();
    }

    fn update_grid(&mut self) {
        let now = Instant::now();

        if self.update_timeout > now {
            return;
        }
        self.update_timeout = now + Duration::from_millis(30);

        self.ca.update();
    }

    fn update_color(&mut self) {
        let now = Instant::now();

        if self.color_timeout > now {
            return;
        }
        self.color_timeout = now + Duration::from_millis(10);

        self.color = self.color.into_hsl().shift_hue(0.5.into()).into_rgb();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("elementary cellular automaton", [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        ca: CA::new(2, 2, 0).unwrap(),
        cell_size: 5,
        rule: 22,

        gl: GlGraphics::new(opengl),
        color: Rgb::new(1.0, 0.0, 0.0),
        update_timeout: Instant::now(),
        color_timeout: Instant::now(),
        _fps: FPSCounter::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(k) = e.press_args() {
            match k {
                Keyboard(key) => app.key_pressed(&key),
                _ => {}
            }
        }
    }
}
