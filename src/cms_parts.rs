use macroquad::prelude::*;

pub fn make_tracker_pixels(origin: Vec3) {
    // center cylinder on origin if you want:
    let h = 100.0;
    draw_cylinder_wires(origin + vec3(0.0, -h / 2.0, 0.0), 10.0, 10.0, h, None, BLACK);
}
