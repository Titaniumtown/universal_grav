use crate::misc::{DIMS_F32, G, TIME_DELTA};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Particle {
    mass: f32,
    v_x: f32,
    v_y: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub rgb: [u8; 3],
}

impl Particle {
    pub const fn new(
        mass: f32,
        v_x: f32,
        v_y: f32,
        pos_x: f32,
        pos_y: f32,
        rgb: [u8; 3],
    ) -> Particle {
        Particle {
            mass,
            v_x,
            v_y,
            pos_x,
            pos_y,
            rgb,
        }
    }

    pub fn tick(&mut self) {
        self.pos_x += self.v_x * TIME_DELTA;
        self.pos_y += self.v_y * TIME_DELTA;
        self.wall_check();
    }

    // this does not work and i don't know why
    pub fn gravity(&mut self, other: &Particle) {
        // no need to do calculations if the other particle has mass of 0
        if other.mass == 0.0 {
            return;
        }

        let x_neg = self.pos_x - other.pos_x;
        let y_neg = self.pos_y - other.pos_y;

        // if particles are located at the exact same coordinate, don't do any calculations
        if x_neg == 0.0 && y_neg == 0.0 {
            return;
        }

        let sq_dist = x_neg.powi(2) + y_neg.powi(2);

        // calculate acceleration using Newton's laws of universal gravitation
        let acceleration = (G * other.mass) / sq_dist;

        // this shouldn't happen, but should be checked so infinite or NaN acceleration isn't attempted to be applied
        if !acceleration.is_normal() {
            return;
        }

        // calculate scalar of velocity change on the object in this time
        let diff_velocity = acceleration * TIME_DELTA;

        // interpret scalar change in velocity into velocity in the x and y direction
        let dist = sq_dist.sqrt();
        let y_add = diff_velocity * y_neg / dist;
        let x_add = diff_velocity * x_neg / dist;

        // actually apply to object
        self.v_y += y_add;
        self.v_x += x_add;
    }

    fn wall_check(&mut self) {
        let pos_x_clamped = self.pos_x.clamp(0.0, DIMS_F32.0);
        if pos_x_clamped != self.pos_x {
            self.pos_x = pos_x_clamped;
            self.v_x = -self.v_x;
        }

        let pos_y_clamped = self.pos_y.clamp(0.0, DIMS_F32.1);
        if pos_y_clamped != self.pos_y {
            self.pos_y = pos_y_clamped;
            self.v_y = -self.v_y;
        }
    }
}
