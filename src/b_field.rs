// Right now, only inside solenoid, so constant everywhere
// later, for outside the solenoid, field strength and direction more complicated because of steel return yokes

use macroquad::prelude::*;

pub fn b_field(_pos: Vec3) -> Vec3 {
    vec3(0.0, 0.0, 3.8) // Tesla, also this is a single unit vector, may need to transform to being the same vector at all points in space
}



pub fn draw_b_field(b_field: Vec3) {

    draw_line_3d(
            b_field,
            vec3(5.0, 5.0, 5.0),
            Color::new(1.0, 1.0, 0.0, 1.0),
        );
}




//yeah, later this should be a function that returns a vector function of the field strength and direction relative to origin.
