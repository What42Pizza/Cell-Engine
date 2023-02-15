use crate::prelude::*;
use sdl2::{event::Event, keyboard::Keycode, EventPump, render::WindowCanvas, mouse::MouseState};



pub fn process_events (program_data: &mut ProgramData, event_pump: &mut EventPump, canvas: &WindowCanvas) -> Result<(), ProgramError> {
    let mouse_state = event_pump.mouse_state();
    
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => program_data.exit = true,
            Event::KeyDown {keycode: Some(keycode), ..} => {program_data.keys_pressed.insert(keycode, ());},
            Event::KeyUp {keycode: Some(keycode), ..} => {program_data.keys_pressed.remove(&keycode);},
            Event::MouseWheel {y, ..} => process_mouse_wheel(program_data, y, &mouse_state, canvas)?,
            _ => {}
        }
    }

    Ok(())
}



pub fn process_mouse_wheel (program_data: &mut ProgramData, y: i32, mouse_state: &MouseState, canvas: &WindowCanvas) -> Result<(), ProgramError> {
    let canvas_size = canvas.output_size()?;
    let mouse_pos = (mouse_state.x(), mouse_state.y());
    let start_grid_pos = fns::convert_screen_to_grid(mouse_pos, &program_data.camera, canvas_size);
    program_data.camera.zoom *= SCROLL_SPEED.pow(y as f64);
    program_data.camera.zoom = program_data.camera.zoom.max(MAX_ZOOM_OUT);
    let end_grid_pos = fns::convert_screen_to_grid(mouse_pos, &program_data.camera, canvas_size);
    program_data.camera.x -= end_grid_pos.0 - start_grid_pos.0;
    program_data.camera.y -= end_grid_pos.1 - start_grid_pos.1;
    Ok(())
}
