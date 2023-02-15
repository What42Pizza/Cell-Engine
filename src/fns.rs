use crate::prelude::*;
use std::fs::OpenOptions;
use sdl2::{render::{TextureCreator, TextureValueError}, video::WindowContext, surface::Surface};





pub fn find_item_index<T: PartialEq> (input: &[T], item: &T) -> Option<usize> {
    for (i, curr_item) in input.iter().enumerate() {
        if curr_item == item {
            return Some(i);
        }
    }
    None
}





pub fn convert_grid_to_screen (grid_pos: (f64, f64), camera: &Camera, canvas_size: (u32, u32)) -> (i32, i32) {
    let x = convert_single_grid_to_screen(grid_pos.0, camera.x, camera.zoom, canvas_size);
    let y = convert_single_grid_to_screen(grid_pos.1, camera.y, camera.zoom, canvas_size);
    (x, y)
}

pub fn convert_single_grid_to_screen (grid_pos: f64, camera_pos: f64, zoom: f64, canvas_size: (u32, u32)) -> i32 {
    let pos_minus_camera = grid_pos - camera_pos;
    let pos_scaled = pos_minus_camera * zoom;
    let screen_pos = pos_scaled * canvas_size.1 as f64;
    screen_pos.round() as i32
}



pub fn convert_screen_to_grid (screen_pos: (i32, i32), camera: &Camera, canvas_size: (u32, u32)) -> (f64, f64) {
    let x = convert_single_screen_to_grid(screen_pos.0, camera.x, camera.zoom, canvas_size);
    let y = convert_single_screen_to_grid(screen_pos.1, camera.y, camera.zoom, canvas_size);
    (x, y)
}

pub fn convert_single_screen_to_grid (screen_pos: i32, camera_pos: f64, zoom: f64, canvas_size: (u32, u32)) -> f64 {
    let pos_scaled = screen_pos as f64 / canvas_size.1 as f64;
    let pos_scaled = pos_scaled / zoom;
    pos_scaled + camera_pos
}





pub fn get_texture_size (texture: &Texture) -> (u32, u32) {
    let query = texture.query();
    (query.width, query.height)
}

// THIS IS UNTESTED
pub fn get_spritesheet_src_from_index (spritesheet: &Texture, index: u32, sprite_width: u32, sprite_height: u32) -> Rect {
    let spritesheet_width = spritesheet.query().width;
    let sprites_per_row = spritesheet_width / sprite_width;
    let row_num = index % sprites_per_row;
    let column_num = index / sprites_per_row;
    Rect::new((row_num * sprite_width) as i32, (column_num * sprite_height) as i32, sprite_width, sprite_height)
}



pub fn get_program_dir() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("Could not retrieve the path for the current exe.");
    path.pop();
    path
}



pub fn get_file_exists (path: &Path) -> Result<bool, IoError> {
    let file = OpenOptions::new().read(true).open(path);
    if file.is_ok() {return Ok(true);}
    let err = file.unwrap_err();
    match err.kind() {
        IoErrorKind::NotFound => Ok(false),
        _ => Err(err),
    }
}



pub fn some_if<T> (condition: bool, some_fn: impl FnOnce() -> T) -> Option<T> {
    if condition {
        Some(some_fn())
    } else {
        None
    }
}



pub fn blend_colors (color1: Color, color2: Color, blend_amount: f64) -> Color {
    let (r1, g1, b1) = color1.rgb();
    let (r2, g2, b2) = color2.rgb();
    let r = (r1 as f64).lerp(r2 as f64, blend_amount) as u8;
    let g = (g1 as f64).lerp(g2 as f64, blend_amount) as u8;
    let b = (b1 as f64).lerp(b2 as f64, blend_amount) as u8;
    Color::RGB(r, g, b)
}



pub fn get_empty_texture (texture_creator: &TextureCreator<WindowContext>) -> Result<Texture<'_>, TextureValueError> {
    texture_creator.create_texture_from_surface(Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::ARGB8888).unwrap())
}
