extern crate piston;
extern crate graphics;
extern crate piston_window;

use piston_window::*;
use graphics::radians::Radians;

const WIDTH:  u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let window: PistonWindow = WindowSettings::new("Fractal tree", [WIDTH, HEIGHT]).exit_on_esc(true).build().unwrap();

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            for l in lines(WIDTH as f64 / 2.0, HEIGHT as f64, -90.0, 10.0) {
                line([0.0, 1.0, 0.390, 1.0], 1.0, l, c.transform, g);
            }
        });
    }
}

fn lines(x: f64, y: f64, ang: f64, depth: f64) -> Vec<[f64; 4]> {
    let vec = Vec::<[f64; 4]>::new();

    if depth > 0.0 {
        let mut vec = Vec::<[f64; 4]>::new();
        let x1 = x + (ang.deg_to_rad().cos() * depth * 10.0);
        let y1 = y + (ang.deg_to_rad().sin() * depth * 10.0);

        vec.push([x, y, x1, y1]);

        vec.extend(lines(x1, y1, ang - 20.0, depth - 1.0));
        vec.extend(lines(x1, y1, ang + 20.0, depth - 1.0));
    }

    return vec;
}
