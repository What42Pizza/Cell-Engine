use crate::prelude::*;
use sdl2::keyboard::Keycode;



pub struct ProgramData<'a> {

    pub frame_count: u64,
    pub exit: bool,

    pub camera: Camera,
    pub textures: ProgramTextures<'a>,
    pub font: Font<'a, 'a>,
    pub keys_pressed: HashMap<Keycode, ()>,

    pub cells_list: HashMap<EntityID, ()>,
    pub world: World,

}





pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64, // >1 means zoomed in, <1 means zoomed out
}



pub struct ProgramTextures<'a> {
    pub ground: Texture<'a>,
    pub circle: Texture<'a>,
}
