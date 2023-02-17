use crate::prelude::*;
use sdl2::{Sdl, ttf::Sdl2TtfContext,
    image::{self, LoadTexture, InitFlag},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext}
};





pub fn init_sdl2() -> (Sdl, Sdl2TtfContext, Canvas<Window>) {

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

    let ttf_context = sdl2::ttf::init().expect("Could not initialize sdl2::ttf");

    (sdl_context, ttf_context, canvas)
}





pub fn init_program_data<'a> (canvas: &Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<ProgramData<'a>, ProgramError> {
    let (width, height) = canvas.output_size()?;

    let textures = load_textures(texture_creator)?;

    let mut font_path = fns::get_program_dir();
    font_path.push("JetBrainsMono-Regular_0.ttf");
    let font = ttf_context.load_font(font_path, height as u16 * 3 / 100)?;

    Ok(ProgramData {

        exit: false,
        frame_count: 0,

        camera: Camera {
            x: 0.,
            y: 0.,
            zoom: 0.2,
        },
        textures,
        font,
        keys_pressed: HashMap::new(),

        cells: EntityContainer::new(),
        food: EntityContainer::new(),

    })
}



pub fn load_textures (texture_creator: &TextureCreator<WindowContext>) -> Result<ProgramTextures<'_>, String> {
    Ok(ProgramTextures {
        ground: texture_creator.load_texture("assets/ground.png")?,
        black_ground: texture_creator.load_texture("assets/black_ground.png")?,
        food: texture_creator.load_texture("assets/food.png")?,
        circle: texture_creator.load_texture("assets/circle.png")?,
    })
}
