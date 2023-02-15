// Started 02/11/23
// Last updated 02/13/23



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// nightly features
#![feature(box_syntax)]



// settings

// takes ~22 bytes per empty cell (2^14 x 2^14 takes ~6 GB)
const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;
const MAX_ENTITIES_COUNT: usize = GRID_WIDTH * GRID_HEIGHT / 2;

const CAMERA_SPEED: f64 = 0.5;
const SCROLL_SPEED: f64 = 1.1;
const MAX_ZOOM_OUT: f64 = 1./128.;



mod update_mod;
mod render;
mod init;
mod data_mod;
mod fns;
mod prelude;



use prelude::*;



pub fn main() -> Result<(), ProgramError> {
    let mut last_update_instant = Instant::now();

    // sdl
    let (sdl_context, ttf_context, mut canvas) = init::init_sdl2();
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event loop.");
    let texture_creator = canvas.texture_creator();

    let mut program_data = init::init_program_data(&canvas, &texture_creator, &ttf_context)?;

    program_data.world.add_entity(Entity::new(1.5, 1.5, 1., 1.,
        EntityData::Cell {
            x_vel: 0.5,
            y_vel: 0.1,
        }
    ));

    let mut last_fps_instant = Instant::now();
    let mut fps_count = 0;
    while !program_data.exit {

        let dt = last_update_instant.elapsed();
        last_update_instant = Instant::now();
        update::update(&mut program_data, &mut event_pump, &canvas, &dt)?;

        render::render(&mut canvas, &mut program_data)?;

        fps_count += 1;
        if last_fps_instant.elapsed().as_millis() > 1000 {
            println!("FPS: {fps_count}");
            fps_count = 0;
            last_fps_instant = Instant::now();
        }
    }

    Ok(())
}
