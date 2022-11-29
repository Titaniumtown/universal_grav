use crate::{
    misc::{orbit_speed, DIMS, GRID_CENTER},
    particle::Particle,
};

use pixels::{PixelsBuilder, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod misc;
mod particle;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum Scenario {
    SimpleElliptical,
    Circle,
    Multi,
    Dual,
}

impl Scenario {
    fn incr(&self) -> Scenario {
        match *self {
            Scenario::SimpleElliptical => Scenario::Circle,
            Scenario::Circle => Scenario::Multi,
            Scenario::Multi => Scenario::Dual,
            Scenario::Dual => Scenario::SimpleElliptical,
        }
    }

    fn decr(&self) -> Scenario {
        match *self {
            Scenario::Circle => Scenario::SimpleElliptical,
            Scenario::Multi => Scenario::Circle,
            Scenario::Dual => Scenario::Multi,
            Scenario::SimpleElliptical => Scenario::Dual,
        }
    }
}

fn set_scenario(s: Scenario) -> Vec<Particle> {
    match s {
        Scenario::SimpleElliptical => {
            vec![
                Particle::new(10f32.powi(13), 0.0, 0.0, 50.0, 50.0, [255, 165, 0]),
                Particle::new(0.0, 0.0, -3.0, 75.0, 50.0, [0, 255, 255]),
            ]
        }
        Scenario::Circle => {
            let center_mass: f32 = 10f32.powi(13);
            let center: f32 = GRID_CENTER.0;
            let radius: f32 = 25.0;
            let orbit_speed = orbit_speed(center_mass as f64, radius as f64);

            // // internal time
            // let period = (2.0 * std::f32::consts::PI * radius) / orbit_speed;

            // // takes TIME_DELTA into account
            // let user_period = period * TIME_DELTA;

            // println!(
            //     "center mass: {}kg\norbit radius: {} meters\norbit speed: {} m/s\nperiod: {}s ({}s)",
            //     center_mass, radius, orbit_speed, period, user_period
            // );

            vec![
                Particle::new(center_mass, 0.0, 0.0, center, GRID_CENTER.1, [255, 165, 0]),
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

        Scenario::Multi => {
            vec![
                Particle::new(10f32.powi(13), 0.0, 0.0, 50.0, GRID_CENTER.1, [255, 165, 0]),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(10f64.powi(13), 5.0),
                    55.0,
                    GRID_CENTER.1,
                    [150, 0, 250],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(10f64.powi(13), 10.0),
                    60.0,
                    GRID_CENTER.1,
                    [0, 0, 250],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(10f64.powi(13), 15.0),
                    65.0,
                    GRID_CENTER.1,
                    [255, 150, 0],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(10f64.powi(13), 20.0),
                    70.0,
                    GRID_CENTER.1,
                    [255, 150, 100],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(10f64.powi(13), 25.0),
                    75.0,
                    GRID_CENTER.1,
                    [0, 150, 150],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(10f64.powi(13), 30.0),
                    80.0,
                    GRID_CENTER.1,
                    [150, 150, 150],
                ),
                Particle::new(0.0, 1.5, 0.0, GRID_CENTER.1, 10.0, [200, 200, 200]),
            ]
        }
        Scenario::Dual => {
            vec![
                Particle::new(10f32.powi(13), 0.0, -4.0, 45.0, GRID_CENTER.1, [0, 255, 0]),
                Particle::new(10f32.powi(13), 0.0, 4.0, 55.0, GRID_CENTER.1, [255, 0, 0]),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(2.0 * 10f64.powi(13), 30.0),
                    80.0,
                    GRID_CENTER.1,
                    [0, 0, 255],
                ),
            ]
        }
    }
}

fn main() {
    let mut scenario = Scenario::SimpleElliptical;
    let mut particles: Vec<Particle> = set_scenario(scenario);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(DIMS.0, DIMS.1);
        let scaled_size = LogicalSize::new(DIMS.0 * 10, DIMS.1 * 10);
        WindowBuilder::new()
            .with_title("Universal Gravitation Demo")
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
            .map_err(|e| panic!("failed to build pixels: {e}"))
            .unwrap()
    };

    let mut screen_data: Vec<(usize, [u8; 3])> = Vec::new();
    let mut screen_data_old: Vec<(usize, [u8; 3])> = Vec::new();
    let mut frames_rendered: u32 = 0;
    let mut skipped_frames: u32 = 0;

    event_loop.run(move |event, _, control_flow| {
        screen_data.clear();
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Right) {
                scenario = scenario.incr();
                particles = set_scenario(scenario);
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                scenario = scenario.decr();
                particles = set_scenario(scenario);
            }

            // exit if escape key pressed
            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
            }

            // print info when I key pressed
            if input.key_pressed(VirtualKeyCode::I) && frames_rendered != 0 {
                let percent_skipped: f32 =
                    ((skipped_frames as f32) / (frames_rendered as f32)) * 100.0;
                println!(
                    "frames skipped: {}% ({}/{})",
                    percent_skipped, skipped_frames, frames_rendered
                );
            }
        }
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

        particles.iter().for_each(|p| {
            // convert to u32 and clamp
            let x = (p.pos_x as u32).clamp(0, DIMS.0 - 1);
            let y = (p.pos_y as u32).clamp(0, DIMS.1 - 1);

            // calculate linear index
            let i = ((y * DIMS.0) + x) as usize * 4;

            // push data to screen_data
            screen_data.push((i, p.rgb));
        });

        let frame = pixels.get_frame_mut();
        if screen_data != screen_data_old {
            screen_data.iter().for_each(|(i, rgb)| {
                // set color
                frame[*i..=*i + 2].copy_from_slice(rgb);

                // set alpha channel
                frame[*i + 3] = 255u8;
            });
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {e}"))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
            }
            // no need to copy here as `screen_data` will be cleared on next iteration of the event loop
            std::mem::swap(&mut screen_data_old, &mut screen_data);
        } else {
            skipped_frames += 1;
        }
        frames_rendered += 1;
    });
}
