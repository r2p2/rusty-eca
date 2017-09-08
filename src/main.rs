extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate fps_counter;
extern crate palette;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use fps_counter::FPSCounter;
use palette::{ Rgb, Hue, IntoColor };
use std::time::{ Duration, Instant };

type Line = Vec<bool>;
type Lines = Vec<Line>;

pub struct App {
    gl: GlGraphics,
    width: u32,
    height: u32,
    lines: Lines,
    rule: u8,
    color: Rgb,
    cell_size: u32,
    update_timeout: Instant,
    _fps: FPSCounter,
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        if self.width != args.width || self.height != args.height {
            self.width = args.width;
            self.height = args.height;
            Self::reset_lines(&mut self.lines, self.cell_size, args.width, args.height);
        }

        use graphics::*;

        let fg: Rgb = self.color.into_hsl().shift_hue(180.0.into()).into_rgb();

        let bg_color: [f32; 4] = [self.color.red, self.color.green, self.color.blue, 1.0];
        let fg_color: [f32; 4] = [fg.red        , fg.green        , fg.blue        , 1.0];

        let pixel = rectangle::square(0.0, 0.0, self.cell_size as f64);
        let lines = &self.lines;
        let cell_size = self.cell_size as usize;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(bg_color, gl);

            for (y, line) in lines.iter().enumerate() {
                for (x, cell) in line.iter().enumerate() {
                    if ! cell {
                        continue
                    }

                    let transform = c.transform.trans((x*cell_size) as f64, (y*cell_size) as f64);
                    rectangle(fg_color, pixel, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let now = Instant::now();
        if self.update_timeout > now {
            return;
        }
        self.update_timeout = now + Duration::from_millis(30);

        if self.lines.len() < 2 {
            return;
        }

        for y in 0..self.lines.len()-1 {
            self.lines[y] = self.lines[y+1].clone();
        }

        let last_line = self.lines.last_mut().expect("");
        let mut new_line = last_line.clone();

        for x in 1..last_line.len()-1 {
            let l = last_line[x-1] as u8;
            let m = last_line[x] as u8;
            let r = last_line[x+1] as u8;
            let sig = (l << 2) | (m << 1) | r ;
            new_line[x] = if (self.rule & (1 << sig)) > 0 {
                true
            } else {
                false
            }
        }

        *last_line = new_line;

        self.color = self.color.into_hsl().shift_hue(0.5.into()).into_rgb();
    }

    fn reset_lines(lines: &mut Lines, cell_size: u32, width: u32, height: u32) {
        lines.clear();
        *lines = vec!(vec!(false; (width/cell_size) as usize); (height/cell_size) as usize);
        lines[(height/cell_size-1) as usize][(width/cell_size/2) as usize] = true;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning square",
        [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        width: 0,
        height: 0,
        lines: vec!(vec!(false; 0); 0),
        rule: 150,
        color: Rgb::new(1.0, 0.0, 0.0),
        cell_size: 5,
        update_timeout: Instant::now(),
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
    }
}
