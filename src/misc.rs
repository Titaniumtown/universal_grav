pub const TIME_DELTA: f32 = 0.1;
const_assert!(TIME_DELTA > 0.0);
const_assert!(TIME_DELTA.is_normal());

/// "Big G" gravitational constant
pub const G: f32 = -6.67430E-11;
const_assert!(0.0 > G);
const_assert!(G.is_normal());

/// DIMS of board, not pixels
pub const DIMS: (usize, usize) = (100, 100);
const_assert!(DIMS.0 > 0);
const_assert!(DIMS.1 > 0);

/// DIMS converted to u32
pub const DIMS_U32: (u32, u32) = (DIMS.0 as u32, DIMS.1 as u32);

/// DIMS converted to f32
pub const DIMS_F32: (f32, f32) = (DIMS.0 as f32, DIMS.1 as f32);

/// Center of board
pub const GRID_CENTER: (f32, f32) = (DIMS_F32.0 / 2.0, DIMS_F32.1 / 2.0);

pub fn orbit_speed(mass: f32, radius: f32) -> f32 {
    ((-G * mass) / radius).sqrt()
}
