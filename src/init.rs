use crate::prelude::*;
use sdl2::{Sdl,
    image::{self, LoadTexture, InitFlag},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext}
};
use ab_glyph::FontVec;





pub fn init_sdl2() -> (Sdl, Canvas<Window>) {

    let sdl_context = sdl2::init().expect("Could not initialize sdl2");
    let _image_context = image::init(InitFlag::PNG).expect("Could not retrieve sdl image context");
    let video_subsystem = sdl_context.video().expect("Could not retrieve video subsystem");
    let window = video_subsystem.window("Creatures Game", 1280, 720)
        .position_centered()
        .build()
        .expect("Could not build window");

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Could not build canvas");

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();
    canvas.present();

    (sdl_context, canvas)
}





pub fn init_program_data<'a> (canvas: &Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>) -> Result<ProgramData<'a>, ProgramError> {
    let (width, height) = canvas.output_size()?;

    let textures = load_textures(&texture_creator)?;

    let mut font_path = fns::get_program_dir();
    font_path.push("JetBrainsMono-Regular_0.ttf");
    let raw_font_bytes = fs::read(font_path).expect("Could not load the given font (fs::read error)");
    let font = FontVec::try_from_vec(raw_font_bytes).expect("Could not load the given font (FontVec::try_from_vec error)");

    Ok(ProgramData::new(textures, font, texture_creator))
}



pub fn load_textures (texture_creator: &TextureCreator<WindowContext>) -> Result<ProgramTextures<'_>, String> {
    Ok(ProgramTextures {
        ground: texture_creator.load_texture("assets/ground.png")?,
        black_ground: texture_creator.load_texture("assets/black_ground.png")?,
        food: texture_creator.load_texture("assets/food.png")?,
        circle: texture_creator.load_texture("assets/circle.png")?,
    })
}
