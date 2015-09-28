extern crate piston;
extern crate graphics;
extern crate piston_window;

use piston_window::*;
use graphics::radians::Radians;

const WIDTH:  u32 = 1024;
const HEIGHT: u32 = 768;
const DEPTH:  f64 = 12.0;

struct LLine {
    line: Line,
    coordinates: graphics::types::Line
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Fractal tree", [WIDTH, HEIGHT]).exit_on_esc(true).build().unwrap();

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            for l in lines(WIDTH as f64 / 2.0, HEIGHT as f64, -90.0, DEPTH) {
                l.line.draw(l.coordinates, &c.draw_state, c.transform, g);
            }
        });
    }
}

fn lines(x: f64, y: f64, ang: f64, depth: f64) -> Vec<LLine> {
    let mut vec = Vec::<LLine>::new();

    if depth > 0.0 {
        let x1 = x + (ang.deg_to_rad().cos() * depth * 0.9 * 10.0);
        let y1 = y + (ang.deg_to_rad().sin() * depth * 0.9 * 10.0);

        let line = LLine { line: Line::new([0.0, 1.0, 0.0, 1.0], depth * 0.35), coordinates: [x, y, x1, y1] };

        vec.push(line);

        vec.extend(lines(x1, y1, ang - 25.0, depth - 1.0));
        vec.extend(lines(x1, y1, ang + 25.0, depth - 1.0));
    }

    return vec;
}
