use macroquad::prelude::*;

// So, since I'm using macroquad's prelude it imports a lot of different things for me.
// Macroquad uses glam for the Vec2, it also uses quad_rand for me 

pub const DELTA_T: f32 = 0.001; //This can maybe replaced with let delta = get_frame_time(); ? but idk what that does

pub const B_FIELD: f32 = 3.8;

pub const EM_CONST: f32 = 0.001;

pub struct Particle {
    pub pos: Vec3,
    pub mom: Vec3,
    pub mass: f32,
    pub size: f32,
    pub pdg_id: i32,
    pub charge: i8,
}

pub fn spawn(v: Vec<Particle>, location: Vec3, mom: Vec3) -> Vec<Particle> {
    
    let mut v_temp = v;

    v_temp.push(Particle {
        pos: location,
        mom: mom,
        mass: 10.0,
        size: 1.0,
        pdg_id: 10,
        charge: rand::gen_range(-2, 2),
    });

    v_temp
}

/* pub fn despawn(v: &Particle) -> bool {

    let mut result: bool = true;

    if v.pos.x > screen_width() { result = false; }
    if v.pos.x < 0.0 { result = false; }
    if v.pos.y > screen_height() { result = false; }
    if v.pos.y < 0.0 { result = false; }
    result
} */

pub fn update_particles(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {

            p.pos += p.mom * DELTA_T;

            let color = if p.charge < 0 {
                    RED
                } else if p.charge > 0 {
                    BLUE
                } else {
                    BLACK
                };

            draw_sphere(p.pos, p.size, None, color);
        }
    }

/* pub fn lorentz_force(p: &mut Particle) {
    
    // Lorentz force: F = q * v × B
    // For 2D with B field in z-direction: F_x = c * q * v_y * B, F_y = c* -q * v_x * B
    let acc_x = EM_CONST * p.charge as f32 * p.vel.y * B_FIELD;
    let acc_y = EM_CONST * -(p.charge as f32) * p.vel.x * B_FIELD;
    
    // Update velocity
    p.vel.x += acc_x * DELTA_T;
    p.vel.y += acc_y * DELTA_T;
    
} */

/* pub fn mom_exchange(p: &mut Particle) {
    
    if p.pos.x > (screen_width() / 2.0 - p.size) {
        p.vel.x = -p.vel.x;
    }
    if p.pos.y == 2.0 {
        p.vel.y = -p.vel.y;
    }
} */