pub const TIME_DELTA: f32 = 0.1;
const_assert!(TIME_DELTA > 0.0);
const_assert!(TIME_DELTA.is_normal());

pub const G: f64 = -6.67430E-11;
const_assert!(0.0 > G);
const_assert!(G.is_normal());

pub const DIMS: (usize, usize) = (100, 100);
const_assert!(DIMS.0 > 0);
const_assert!(DIMS.1 > 0);

pub const GRID_CENTER: (f32, f32) = (DIMS.0 as f32 / 2.0, DIMS.1 as f32 / 2.0);
pub const DIMS_F32: (f32, f32) = (DIMS.0 as f32, DIMS.1 as f32);

pub fn orbit_speed(mass: f64, radius: f64) -> f32 {
    ((-G * mass) / radius).sqrt() as f32
}
