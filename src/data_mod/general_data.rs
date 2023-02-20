use crate::prelude::*;
use sdl2::{keyboard::Keycode, render::TextureCreator, video::WindowContext};



pub struct ProgramData<'a> {

    pub start_instant: Instant,
    pub frame_count: u64,
    pub exit: bool,

    pub camera: Camera,
    pub selected_entity: EntitySelection,
    pub keys_pressed: HashMap<Keycode, ()>,

    pub textures: ProgramTextures<'a>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub font: Font<'a, 'a>,
    pub text_cache: HashMap<String, Texture<'a>>,

    pub cells: EntityContainer<Cell>,
    pub food: EntityContainer<Food>,

}

impl<'a> ProgramData<'a> {

    pub fn new (textures: ProgramTextures<'a>, font: Font<'a, 'a>, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {

            start_instant: Instant::now(),
            frame_count: 0,
            exit: false,

            camera: Camera {
                x: 0.,
                y: 0.,
                zoom: 0.2,
            },
            selected_entity: EntitySelection::None,
            keys_pressed: HashMap::new(),

            textures,
            texture_creator,
            font,
            text_cache: HashMap::new(),

            cells: EntityContainer::new(),
            food: EntityContainer::new(),

        }
    }

    pub fn key_is_pressed (&self, keycode: Keycode) -> bool {
        self.keys_pressed.contains_key(&keycode)
    }

    pub fn ensure_text_is_rendered (&mut self, input: impl Into<String>) -> Result<(), ProgramError> {
        let input = input.into();
        if self.text_cache.contains_key(&input) {return Ok(());}
        let new_texture = self.font
            .render("Cell Information")
            .blended(Color::RGB(255, 255, 255))?
            .as_texture(&self.texture_creator)?;
        self.text_cache.insert(input, new_texture);
        Ok(())
    }

}





#[derive(PartialEq)]
pub enum EntitySelection {
    None,
    Cell (EntityID),
    Food (EntityID),
}





pub struct Food {
    pub energy: f64,
    pub material: f64,
    pub entity: RawEntity,
}

impl Food {
    pub fn new (x: f64, y: f64, energy: f64, material: f64) -> Self {
        let size = material / 4. + 0.25;
        Self {
            energy,
            material,
            entity: RawEntity::new(x, y, size, size),
        }
    }
    pub fn from_cell (cell: &Cell) -> Self {
        Self::new(cell.entity.x, cell.entity.y, cell.energy, cell.material)
    }
}

impl Entity for Food {
    fn get_texture<'a> (&self, textures: &'a ProgramTextures<'a>) -> &'a Texture<'a> {
        &textures.food
    }
}

impl AsRef<RawEntity> for Food {
    fn as_ref (&self) -> &RawEntity {
        &self.entity
    }
}

impl AsMut<RawEntity> for Food {
    fn as_mut (&mut self) -> &mut RawEntity {
        &mut self.entity
    }
}





pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64, // >1 means zoomed in, <1 means zoomed out
}



pub struct ProgramTextures<'a> {
    pub ground: Texture<'a>,
    pub black_ground: Texture<'a>,
    pub food: Texture<'a>,
    pub circle: Texture<'a>,
}





pub struct FRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl FRect {
    pub fn new (x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {x, y, width, height}
    }
    pub fn to_rect (&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32)
    }
}
