use macroquad::prelude::*;
// use glam::vec3;

// const MOVE_SPEED: f32 = 0.1;

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


    loop {
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}