mod cms_parts;
mod particle;

use macroquad::prelude::*;
// use glam::vec3;
use cms_parts::*;
use particle::*;

// const MOVE_SPEED: f32 = 0.1;

const ORIGIN: Vec3 = vec3(0.0, 0.0, 0.0);

const NUM_PART: i32 = 10;

const MOVE_SPEED: f32 = 0.1;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}



#[macroquad::main(conf)]
async fn main() {

let mut particles: Vec<Particle> = Vec::new();

let mut camera_position = vec3(-20., 15., 0.);

    loop {
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: camera_position,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        if is_key_down(KeyCode::Up) {
            camera_position.x += MOVE_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            camera_position -= MOVE_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            camera_position -= MOVE_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            camera_position += MOVE_SPEED;
        }

        draw_grid(20, 1., BLACK, GRAY);

        if particles.len() < NUM_PART as usize {
        
        let mom = vec3(
            rand::gen_range(-2.0, 2.0),
            rand::gen_range(-2.0, 2.0),
            rand::gen_range(-2.0, 2.0),
        );

        particles = spawn(particles, ORIGIN, mom);
        }
        
        //update position, draw particles

        update_particles(&mut particles);


        //make_tracker_pixels(ORIGIN);

        // Back to screen space, render some text

        set_default_camera();

    let cam_text = format!(
        "camera position: x={:.2}, y={:.2}, z={:.2}",
        camera_position.x,
        camera_position.y,
        camera_position.z
    );

    draw_text(&cam_text, 10.0, 20.0, 30.0, BLACK);


        next_frame().await
    }
}