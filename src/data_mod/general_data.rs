use crate::prelude::*;
use sdl2::{keyboard::Keycode, render::{TextureCreator, WindowCanvas}, video::WindowContext, surface::Surface, event::Event, mouse::MouseState, EventPump};
use ab_glyph::FontVec;



pub struct ProgramData<'a> {

    pub start_instant: Instant,
    pub frame_count: u64,
    pub exit: bool,

    pub camera: Camera,
    pub selected_entity: EntitySelection,
    pub keys_pressed: HashMap<Keycode, ()>,

    pub render_data: RenderData<'a>,

    pub cells: EntityContainer<Buffer<Cell>>,
    pub food: EntityContainer<Food>,

}

impl<'a> ProgramData<'a> {

    pub fn new (render_data: RenderData<'a>) -> Self {
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

            render_data,

            cells: EntityContainer::new(),
            food: EntityContainer::new(),

        }
    }

    pub fn key_is_pressed (&self, keycode: Keycode) -> bool {
        self.keys_pressed.contains_key(&keycode)
    }

}



pub type GlyphCache<'a> = HashMap<HashableGlyph, GlyphTexture<'a>>;





pub struct RenderData<'a> {
    pub textures: ProgramTextures<'a>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub font: FontVec,
    pub glyph_cache: GlyphCache<'a>,
}

impl<'a> RenderData<'a> {
    pub fn new (textures: ProgramTextures<'a>, font: FontVec, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            textures,
            texture_creator,
            font,
            glyph_cache: HashMap::new(),
        }
    }
}



#[derive(PartialEq)]
pub enum EntitySelection {
    None,
    Cell (EntityID),
    Food (EntityID),
}



#[derive(Hash, Eq, PartialEq)]
pub struct HashableGlyph {
    pub glyph_id: GlyphId,
    pub scale_x: u32,
    pub scale_y: u32,
}

impl HashableGlyph {
    pub fn from_glyph (glyph: &Glyph) -> Self {
        Self {
            glyph_id: glyph.id,
            scale_x: glyph.scale.x as u32,
            scale_y: glyph.scale.y as u32,
        }
    }
}



pub struct GlyphTexture<'a> {
    pub texture: Texture<'a>,
    pub origin_x: i32,
    pub origin_y: i32,
}





#[derive(Debug)]
pub struct EventsData {
    pub list: Vec<Event>,
    pub mouse_state: MouseState,
}

impl EventsData {

    pub fn from_event_pump (event_pump: &mut EventPump) -> Self {
        Self {
            list: event_pump.poll_iter().filter(Self::filter_event).collect(),
            mouse_state: event_pump.mouse_state(),
        }
    }

    pub fn filter_event (event: &Event) -> bool {
        matches!(event,
            Event::Quit {..} |
            Event::KeyDown {..} |
            Event::KeyUp {..} |
            Event::MouseWheel {..} |
            Event::MouseButtonDown {..}
        )
    }

}





pub struct Buffer<T> {
    pub main: AtomicRefCell<T>,
    pub alt: AtomicRefCell<T>,
}

impl<T> Buffer<T> {

    pub fn main (&self) -> AtomicRef<T> {
        self.main.borrow()
    }
    pub fn main_mut (&self) -> AtomicRefMut<T> {
        self.main.borrow_mut()
    }

    pub fn alt (&self) -> AtomicRef<T> {
        self.alt.borrow()
    }
    pub fn alt_mut (&mut self) -> AtomicRefMut<T> {
        self.alt.borrow_mut()
    }

    pub fn both (&self) -> (AtomicRef<T>, AtomicRef<T>) {
        (self.main.borrow(), self.alt.borrow())
    }
    pub fn both_mut (&mut self) -> (AtomicRefMut<T>, AtomicRef<T>) {
        (self.main.borrow_mut(), self.alt.borrow())
    }

}

impl<T: Clone> Buffer<T> {
    pub fn new (input: T) -> Buffer<T> {
        Self {
            main: AtomicRefCell::new(input.clone()),
            alt: AtomicRefCell::new(input),
        }
    }
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
        Self::new(cell.entity.x, cell.entity.y, cell.main_data.energy, cell.main_data.material)
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





#[derive(Debug)]
pub struct Area {
    pub screen_size: (u32, u32),
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area {

    pub fn new (screen_size: (u32, u32)) -> Self {
        Self {
            screen_size,
            x: 0.,
            y: 0.,
            width: 1.,
            height: 1.,
        }
    }

    pub fn get_basic_sub_area (&self, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            screen_size: self.screen_size,
            x: self.x + x * self.width,
            y: self.y + y * self.height,
            width:  width  * self.width,
            height: height * self.height,
        }
    }

    pub fn get_sub_area (&self, x: f64, y: f64, width: f64, height: f64, natural_x: f64, natural_width: f64) -> Self {
        let aspect_ratio = (self.screen_size.0 as f64 / self.screen_size.1 as f64) * (self.width / self.height);
        Self {
            screen_size: self.screen_size,
            x: self.x + x * self.width + natural_x * self.width / aspect_ratio,
            y: self.y + y * self.height,
            width:  width  * self.width + natural_width * self.width / aspect_ratio,
            height: height * self.height,
        }
    }

    pub fn get_point (&self, x: f64, y: f64, natural_x: f64) -> (i32, i32) {
        let aspect_ratio = (self.screen_size.0 as f64 / self.screen_size.1 as f64) * (self.width / self.height);
        let mut point_x = self.x + x * self.width + natural_x * self.width / aspect_ratio;
        let mut point_y = self.y + y * self.height;
        point_x *= self.screen_size.0 as f64;
        point_y *= self.screen_size.1 as f64;
        (point_x.round() as i32, point_y.round() as i32)
    }

    pub fn to_rect (&self) -> Rect {
        let x = self.x * self.screen_size.0 as f64;
        let y = self.y * self.screen_size.1 as f64;
        let width  = self.width  * self.screen_size.0 as f64;
        let height = self.height * self.screen_size.1 as f64;
        let end_x = x + width;
        let end_y = y + height;
        let final_x = x.round() as i32;
        let final_y = y.round() as i32;
        let final_width = (end_x.round() as i32) - final_x;
        let final_height = (end_y.round() as i32) - final_y;
        Rect::new(final_x, final_y, final_width as u32, final_height as u32)
    }

}
