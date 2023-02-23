// Started 02/11/23
// Last updated 02/22/23



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// nightly features
#![feature(box_syntax)]
#![feature(map_many_mut)]
#![feature(slice_take)]



// General Settings

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const MAX_ENTITIES_COUNT: usize = GRID_WIDTH * GRID_HEIGHT / 2;

const CAMERA_SPEED: f64 = 0.75;
const SCROLL_SPEED: f64 = 1.1;
const MAX_ZOOM_OUT: f64 = 1./128.;

// Cell Settings

const CELL_DRAG_COEF: f64 = 0.1;
const CELL_CONNECTION_FORCE: f64 = 10.0;
const CELL_CONNECTION_DRAG: f64 = 3.0;
const CELL_CONNECTION_DISTANCE: f64 = 1.1;
const CELL_INTERSECTION_FORCE: f64 = 100.0;

const CELL_ENERGY_USE_RATE: f64 = 0.001;
const CELL_HEALING_RATE: f64 = 0.1;
const CELL_HEALING_ENERGY_COST: f64 = 0.2;
const CELL_HEALING_MATERIAL_COST: f64 = 0.5;
const CELL_ENERGY_TRANSFER_RATE: f64 = 0.25;
const CELL_MATERIAL_TRANSFER_RATE: f64 = 0.1;

// Cell Type Settings

const CELL_FAT_ENERGY_STORE_THRESHOLD: (f64, f64, f64)     = (0.0, 1.0, 0.75);
const CELL_FAT_ENERGY_RELEASE_THRESHOLD: (f64, f64, f64)   = (0.0, 1.0, 0.5);
const CELL_FAT_ENERGY_STORE_RATE: (f64, f64, f64)          = (0.0, 0.2, 0.1);
const CELL_FAT_ENERGY_RELEASE_RATE: (f64, f64, f64)        = (0.0, 0.2, 0.1);
const CELL_FAT_MATERIAL_STORE_THRESHOLD: (f64, f64, f64)   = (0.0, 1.0, 0.75);
const CELL_FAT_MATERIAL_RELEASE_THRESHOLD: (f64, f64, f64) = (0.0, 1.0, 0.5);
const CELL_FAT_MATERIAL_STORE_RATE: (f64, f64, f64)        = (0.0, 0.2, 0.1);
const CELL_FAT_MATERIAL_RELEASE_RATE: (f64, f64, f64)      = (0.0, 0.2, 0.1);

const CELL_PHOTOSYNTHESISER_RATE: f64 = 0.025;



mod update_mod;
mod render_mod;
mod init;
mod data_mod;
mod fns;
mod prelude;



use prelude::*;





fn add_test_data (program_data: &mut ProgramData) {

    let cell_1 = Cell::new_with_vel(RawCell::new_fat_cell(), (1.5, 1.5), 1.0, 1.0, 0.0, (5.0, 0.0));
    let cell_2 = Cell::new_with_vel(RawCell::new_fat_cell(), (2.5, 1.7), 1.0, 1.0, 0.0, (-5.0, 5.0));
    let cell_3 = Cell::new_with_vel(RawCell::new_fat_cell(), (1.7, 2.5), 1.0, 1.0, 0.0, (0.0, -5.0));
    let cell_1_id = program_data.cells.add_entity(cell_1).unwrap();
    let cell_2_id = program_data.cells.add_entity(cell_2).unwrap();
    let cell_3_id = program_data.cells.add_entity(cell_3).unwrap();
    program_data.cells.master_list[cell_1_id.0].0.as_mut().unwrap().connected_cells.append(&mut vec!(cell_2_id, cell_3_id));
    program_data.cells.master_list[cell_2_id.0].0.as_mut().unwrap().connected_cells.append(&mut vec!(cell_1_id, cell_3_id));
    program_data.cells.master_list[cell_3_id.0].0.as_mut().unwrap().connected_cells.append(&mut vec!(cell_1_id, cell_2_id));

    program_data.food.add_entity(Food::new(3.5, 2.5, 1.0, 1.0));

}





pub fn main() -> Result<(), ProgramError> {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut last_update_instant = Instant::now();

    // sdl
    let (sdl_context, mut canvas) = init::init_sdl2();
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event loop.");
    let texture_creator = canvas.texture_creator();

    let mut program_data = init::init_program_data(&canvas, &texture_creator)?;

    add_test_data(&mut program_data);

    let mut last_fps_instant = Instant::now();
    let mut fps_count = 0;
    while !program_data.exit {

        let dt = last_update_instant.elapsed().as_secs_f64().min(0.03);
        last_update_instant = Instant::now();
        update::update(&mut program_data, &mut event_pump, &canvas, dt)?;

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
