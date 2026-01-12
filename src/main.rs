mod cms_parts;
mod particle;
mod camera;
mod b_field;


use macroquad::prelude::*;
// use glam::vec3;
use cms_parts::*;
use b_field::*;
use particle::*;
use camera::OrbitCamera;

// const MOVE_SPEED: f32 = 0.1;

const ORIGIN: Vec3 = vec3(0.0, 0.0, 0.0);

const NUM_PART: i32 = 12;

const MOVE_SPEED: f32 = 1.0;

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

let mut b_field = b_field(vec3(0.0, 0.0, 0.0));

// init orbital camera
let mut camera = OrbitCamera::new(vec3(-20., 15., 0.), ORIGIN);

//let mut sim_time: f32 = 0.0;

let mut frame_count: u64 = 0;

let update_track_frame: u64 = 60; // How many frames to pass before pushing another value of position to track list? Better performance at larger frames

// ---------------------------------

    loop {

        //let dt = get_frame_time();

        //sim_time += dt;
        frame_count += 1;
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let dt = get_frame_time();

        camera.update(dt, MOVE_SPEED);

        clear_background(LIGHTGRAY);

        camera.apply();

        // Going 3d!

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

        let mut update_track = false;
        if frame_count % update_track_frame == 0 { update_track = true };
        update_particles(&mut particles, b_field, update_track);

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
    draw_text(&dt.to_string(), 10.0, 40.0, 30.0, BLACK);
    //draw_text(seconds elapsed)

        next_frame().await
    }
}
