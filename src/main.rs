use pixels::{PixelsBuilder, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Particle {
    mass: f32,
    v_x: f32,
    v_y: f32,
    pos_x: f32,
    pos_y: f32,
}
pub const TIME_DELTA: f32 = 0.1;

impl Particle {
    fn new(mass: f32, v_x: f32, v_y: f32, pos_x: f32, pos_y: f32) -> Particle {
        Particle {
            mass,
            v_x,
            v_y,
            pos_x: pos_x.clamp(0.0, DIMS_F32.0),
            pos_y: pos_y.clamp(0.0, DIMS_F32.1),
        }
    }

    fn tick(&mut self) {
        self.pos_x += self.v_x * TIME_DELTA;
        self.pos_y += self.v_y * TIME_DELTA;
        self.wall_check();
    }

    // this does not work and i don't know why
    fn gravity(&mut self, other: &Particle) {
        if *self == *other {
            return;
        }

        let x_neg = self.pos_x - other.pos_x;
        let y_neg = self.pos_y - other.pos_y;

        const G: f64 = -0.0000000000667430;
        // const G: f64 = 0.0001;
        let acceleration = (G * other.mass as f64) / ((x_neg.powi(2) + y_neg.powi(2)) as f64);
        println!("a: {}", acceleration);

        if !acceleration.is_normal() {
            return;
        }
        let velocity = acceleration * TIME_DELTA as f64;
        // println!("v: {}", velocity);

        let degree = ((y_neg / x_neg).atan()) as f64;
        // println!("d: {}", degree);

        let y_add = degree.sin() * velocity;
        let x_add = degree.cos() * velocity;
        self.v_y += y_add as f32;
        self.v_x += x_add as f32;
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

const DIMS: (u32, u32) = (100, 100);
const DIMS_F32: (f32, f32) = (DIMS.0 as f32, DIMS.1 as f32);
// const PARTICLE_NUM: usize = 2;
fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    // let mut rand = rand::thread_rng();
    // for _ in 0..PARTICLE_NUM {
    //     let x = rand.gen_range(0.0..DIMS_F32.0);
    //     let y = rand.gen_range(0.0..DIMS_F32.1);
    //     let v_x = rand.gen_range(-1.0..1.0);
    //     let v_y = rand.gen_range(-1.0..1.0);

    //     // let x = 42.33694;
    //     // let y = 52.714787;
    //     // let v_x = 0.49794364;
    //     // let v_y = -0.5317668;

    //     let particle = Particle::new(1.0, v_x, v_y, x, y);
    //     particles.push(particle);
    // }
    particles.push(Particle::new(10f32.powi(13), 0.0, 0.0, 50.0, 50.0));
    particles.push(Particle::new(1.0, 0.0, -3.0, 75.0, 50.0));

    let event_loop = EventLoop::new();
    // let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(DIMS.0, DIMS.1);
        let scaled_size = LogicalSize::new(DIMS.0 * 10, DIMS.1 * 10);
        WindowBuilder::new()
            .with_title("Universal Gravitation")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .with_decorations(false) // weird graphical issue happens without this (at least on gnome + wayland) further investigation needed
            .build(&event_loop)
            .expect("WindowBuilder failed")
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(DIMS.0, DIMS.1, surface_texture)
            .enable_vsync(true)
            .build()
            .expect("failed to create pixels")
    };

    event_loop.run(move |_, _, _| {
        pixels.get_frame_mut().fill(0u8);
        let particles_copy = particles.clone();
        particles.iter_mut().enumerate().for_each(|(i, p)| {
            particles_copy
                .iter()
                .enumerate()
                .for_each(|(i_c, p_other)| {
                    if i_c != i {
                        p.gravity(p_other)
                    }
                });
            p.tick()
        });

        let frame = pixels.get_frame_mut();
        particles.iter().for_each(|p| {
            println!("{:?}", p);
            let x = (p.pos_x as u32).clamp(0, DIMS.0 - 1);
            let y = (p.pos_y as u32).clamp(0, DIMS.1 - 1);

            let i = ((y * DIMS.0) + x) as usize * 4;

            frame[i..i + 4].copy_from_slice(&[255u8, 255u8, 255u8, 255u8])
        });
        pixels.render().unwrap();
    });
}
