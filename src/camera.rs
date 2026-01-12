use macroquad::prelude::*;

pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub r: f32,
    pub target: Vec3,
}

impl OrbitCamera {
    pub fn new(position: Vec3, target: Vec3) -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.3,
            r: position.length(),
            target,
        }
    }

    pub fn update(&mut self, dt: f32, speed: f32) {
        // rotation
        if is_key_down(KeyCode::Left)  { self.yaw   -= speed * dt; }
        if is_key_down(KeyCode::Right) { self.yaw   += speed * dt; }
        if is_key_down(KeyCode::Up)    { self.pitch += speed * dt; }
        if is_key_down(KeyCode::Down)  { self.pitch -= speed * dt; }

        let max_pitch = 1.55;
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);

        // zoom
        if is_key_down(KeyCode::W) { self.r -= speed * 20.0 * dt; }
        if is_key_down(KeyCode::S) { self.r += speed * 20.0 * dt; }

        self.r = self.r.clamp(5.0, 200.0);
    }

    pub fn position(&self) -> Vec3 {
        self.target + vec3(
            self.r * self.yaw.cos() * self.pitch.cos(),
            self.r * self.pitch.sin(),
            self.r * self.yaw.sin() * self.pitch.cos(),
        )
    }

    pub fn apply(&self) {
        set_camera(&Camera3D {
            position: self.position(),
            target: self.target,
            up: vec3(0., 1., 0.),
            ..Default::default()
        });
    }
}
