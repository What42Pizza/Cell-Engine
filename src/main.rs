// Started 02/11/23
// Last updated 02/26/23



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// nightly features
#![feature(box_syntax)]
#![feature(map_many_mut)]
#![feature(slice_take)]
#![feature(duration_constants)]
#![feature(type_alias_impl_trait)]



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
const CELL_INTERSECTION_FORCE: f64 = 50.0;

const CELL_ENERGY_USE_RATE: f64 = 0.001;
const CELL_HEALING_RATE: f64 = 0.1;
const CELL_HEALING_ENERGY_COST: f64 = 0.2;
const CELL_HEALING_MATERIAL_COST: f64 = 0.5;
const CELL_ENERGY_TRANSFER_RATE: f64 = 0.25;
const CELL_ENERGY_TRANSFER_THRESHOLD: f64 = 0.025;
const CELL_MATERIAL_TRANSFER_RATE: f64 = 0.1;
const CELL_MATERIAL_TRANSFER_THRESHOLD: f64 = 0.025;

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
mod logger;
mod prelude;



use prelude::*;





fn add_test_data (program_data: &mut ProgramData) {

    for x in 0..30 {
        for y in 0..30 {

            let pos_1 = (x as f64 * 3. + 1.5, y as f64 * 3. + 1.5);
            let pos_2 = (x as f64 * 3. + 2.5, y as f64 * 3. + 1.7);
            let pos_3 = (x as f64 * 3. + 1.7, y as f64 * 3. + 2.5);
            let cell_0 = Cell::new_with_vel(RawCell::new_fat_cell(), pos_1, 1.0, 1.0, 0.0, (5.0, 0.0));
            let cell_1 = Cell::new_with_vel(RawCell::new_fat_cell(), pos_2, 1.0, 1.0, 0.0, (-5.0, 5.0));
            let cell_2 = Cell::new_with_vel(RawCell::new_fat_cell(), pos_3, 1.0, 1.0, 0.0, (0.0, -5.0));
            let cell_0_id = program_data.cells.add_entity(cell_0).unwrap();
            let cell_1_id = program_data.cells.add_entity(cell_1).unwrap();
            let cell_2_id = program_data.cells.add_entity(cell_2).unwrap();
            program_data.cells.master_list[cell_0_id.0].0.as_mut().unwrap().connected_cells = vec!(cell_1_id, cell_2_id);
            program_data.cells.master_list[cell_1_id.0].0.as_mut().unwrap().connected_cells = vec!(cell_0_id, cell_2_id);
            program_data.cells.master_list[cell_2_id.0].0.as_mut().unwrap().connected_cells = vec!(cell_0_id, cell_1_id);

        }
    }

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
    
    //let mut log_path = fns::get_program_dir();
    //log_path.push("log.txt");
    //let mut logger = Logger::new(&log_path)?;

    add_test_data(&mut program_data);

    let mut last_fps_instant = Instant::now();
    let mut fps_count = 0;
    while !program_data.exit {

        let dt = last_update_instant.elapsed().as_secs_f64().min(0.03);
        last_update_instant = Instant::now();
        let events_data = EventsData::from_event_pump(&mut event_pump);
        //logger.log("\nNEXT UPDATE DATA:".as_bytes());
        //logger.log(format!("events: {events_data:?}").as_bytes());
        //logger.log(format!("dt: {dt}").as_bytes());
        //let _ = logger.flush();
        //let start = Instant::now();
        update::update(&mut program_data, &canvas, events_data, dt)?;
        //println!("{}", start.elapsed().as_secs_f64());

        render::render(&mut canvas, &mut program_data)?;

        fps_count += 1;
        if last_fps_instant.elapsed().as_millis() > 1000 {
            println!("FPS: {fps_count}");
            fps_count = 0;
            last_fps_instant = last_fps_instant.checked_add(Duration::SECOND).unwrap();
        }
    }

    Ok(())
}
