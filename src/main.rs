use crate::{
    misc::{DIMS, G, GRID_CENTER, TIME_DELTA},
    particle::Particle,
};

use pixels::{PixelsBuilder, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

mod misc;
mod particle;

#[allow(dead_code)]
enum Instance {
    SimpleElliptical,
    Circle,
}

fn main() {
    let instance: Instance = Instance::SimpleElliptical;
    let mut particles: Vec<Particle> = match instance {
        Instance::SimpleElliptical => {
            vec![
                Particle::new(10f32.powi(13), 0.0, 0.0, 50.0, 50.0, [255, 165, 0]),
                Particle::new(1.0, 0.0, -3.0, 75.0, 50.0, [0, 0, 255]),
            ]
        }
        Instance::Circle => {
            let center_size: f32 = 10f32.powi(13);
            let center: f32 = GRID_CENTER.0;
            let radius: f32 = 25.0;
            let orbit_speed = ((-G * (center_size as f64)) / radius as f64).sqrt() as f32;

            // internal time
            let period = (2.0 * std::f32::consts::PI * radius) / orbit_speed;

            // takes TIME_DELTA into account
            let user_period = period * TIME_DELTA;

            println!(
                "center mass: {}kg\norbit radius: {} meters\norbit speed: {} m/s\nperiod: {}s ({}s)",
                center_size, radius, orbit_speed, period, user_period
            );

            vec![
                Particle::new(center_size, 0.0, 0.0, center, GRID_CENTER.1, [255, 165, 0]),
                Particle::new(
                    1.0,
                    0.0,
                    orbit_speed,
                    center + radius,
                    GRID_CENTER.1,
                    [0, 0, 255],
                ),
            ]
        }
    };

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
            // apply gravity from every object
            particles_copy
                .iter()
                .enumerate()
                .for_each(|(i_c, p_other)| {
                    if i_c != i {
                        p.gravity(p_other)
                    }
                });

            // tick the particle
            p.tick()
        });

        let frame = pixels.get_frame_mut();
        particles.iter().for_each(|p| {
            // convert to u32 and clamp
            let x = (p.pos_x as u32).clamp(0, DIMS.0 - 1);
            let y = (p.pos_y as u32).clamp(0, DIMS.1 - 1);

            // calculate linear index
            let i = ((y * DIMS.0) + x) as usize * 4;

            // set color
            frame[i..=i + 2].copy_from_slice(&p.rgb);
            // set alpha channel
            frame[i + 3] = 255u8;
        });
        pixels.render().unwrap();
    });
}
