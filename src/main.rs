#![feature(const_float_classify)]

#[macro_use]
extern crate static_assertions;

use crate::{
    misc::{orbit_speed, DIMS, DIMS_U32, GRID_CENTER, TIME_DELTA},
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
    // todo, find a much cleaner way of doing this
    const fn incr(&self) -> Scenario {
        match *self {
            Scenario::SimpleElliptical => Scenario::Circle,
            Scenario::Circle => Scenario::Multi,
            Scenario::Multi => Scenario::Dual,
            Scenario::Dual => Scenario::SimpleElliptical,
        }
    }

    const fn decr(&self) -> Scenario {
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
                Particle::new(1e+13, 0.0, 0.0, 50.0, 50.0, [255, 165, 0]),
                Particle::new(0.0, 0.0, -3.0, 75.0, 50.0, [0, 255, 255]),
            ]
        }
        Scenario::Circle => {
            let center_mass: f32 = 1e+13;
            let center: f32 = GRID_CENTER.0;
            let radius: f32 = 25.0;
            let orbit_speed: f32 = orbit_speed(center_mass, radius);

            // internal time
            let period: f32 = (2.0 * std::f32::consts::PI * radius) / orbit_speed;

            // takes TIME_DELTA into account
            let user_period: f32 = period * TIME_DELTA;

            println!(
                "center mass: {center_mass}kg\norbit radius: {radius} meters\norbit speed: {orbit_speed} m/s\nperiod: {period}s ({user_period}s)"
            );

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
                Particle::new(1e+13, 0.0, 0.0, 50.0, GRID_CENTER.1, [255, 165, 0]),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(1e+13, 5.0),
                    55.0,
                    GRID_CENTER.1,
                    [150, 0, 250],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(1e+13, 10.0),
                    60.0,
                    GRID_CENTER.1,
                    [0, 0, 250],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(1e+13, 15.0),
                    65.0,
                    GRID_CENTER.1,
                    [255, 150, 0],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(1e+13, 20.0),
                    70.0,
                    GRID_CENTER.1,
                    [255, 150, 100],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(1e+13, 25.0),
                    75.0,
                    GRID_CENTER.1,
                    [0, 150, 150],
                ),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(1e+13, 30.0),
                    80.0,
                    GRID_CENTER.1,
                    [150, 150, 150],
                ),
                Particle::new(0.0, 1.5, 0.0, GRID_CENTER.1, 10.0, [200, 200, 200]),
            ]
        }
        Scenario::Dual => {
            vec![
                Particle::new(1e+13, 0.0, -4.0, 45.0, GRID_CENTER.1, [0, 255, 0]),
                Particle::new(1e+13, 0.0, 4.0, 55.0, GRID_CENTER.1, [255, 0, 0]),
                Particle::new(
                    0.0,
                    0.0,
                    orbit_speed(2e+13, 30.0),
                    80.0,
                    GRID_CENTER.1,
                    [0, 0, 255],
                ),
            ]
        }
    }
}

const SCALING_FACTOR: u32 = 10;

fn main() {
    let mut scenario = Scenario::SimpleElliptical;
    let mut particles: Vec<Particle> = set_scenario(scenario);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(DIMS_U32.0, DIMS_U32.1);
        let scaled_size =
            LogicalSize::new(DIMS_U32.0 * SCALING_FACTOR, DIMS_U32.1 * SCALING_FACTOR);
        WindowBuilder::new()
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .with_decorations(false) // weird graphical issue happens without this (at least on gnome + wayland) further investigation needed
            .build(&event_loop)
            .map_err(|e| panic!("failed to build Window: {e}"))
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(DIMS_U32.0, DIMS_U32.1, surface_texture)
            .enable_vsync(true)
            .build()
            .map_err(|e| panic!("failed to build pixels: {e}"))
            .unwrap()
    };

    let mut screen_data_old: Vec<(usize, [u8; 3])> = Vec::new();
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Right) {
                scenario = scenario.incr();
                particles = set_scenario(scenario);
            } else if input.key_pressed(VirtualKeyCode::Left) {
                scenario = scenario.decr();
                particles = set_scenario(scenario);
            } else if input.key_pressed(VirtualKeyCode::Escape) {
                // exit if escape key pressed
                *control_flow = ControlFlow::Exit;
            } else if input.key_pressed(VirtualKeyCode::I) {
                // print info when I key pressed
                println!("number of particles: {}", particles.len());
            }
        }

        let particles_copy = particles.clone();
        particles.iter_mut().enumerate().for_each(|(i, p)| {
            // apply gravity from every object
            for (other_i, other_p) in particles_copy.iter().enumerate() {
                if other_i != i {
                    p.gravity(other_p)
                }
            }

            // tick the particle
            p.tick()
        });

        let screen_data: Vec<(usize, [u8; 3])> = particles
            .iter()
            .map(|p| {
                // convert to u32 and clamp
                let x = (p.pos_x as usize).saturating_sub(1);
                let y = (p.pos_y as usize).saturating_sub(1);

                // calculate linear index
                let i = ((y * DIMS.0) + x) * 4;

                // push data to screen_data
                (i, p.rgb)
            })
            .collect();

        if screen_data != screen_data_old {
            let frame = pixels.get_frame_mut();
            frame.fill(0u8);
            screen_data.iter().for_each(|(i, rgb)| {
                unsafe { frame.get_unchecked_mut(*i..=*i + 3) }
                    .copy_from_slice(&[rgb[0], rgb[1], rgb[2], 255u8]);
            });

            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {e}"))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
            }

            // no need to copy here as `screen_data` will be cleared on next iteration of the event loop
            screen_data_old = screen_data;
        }
    });
}
