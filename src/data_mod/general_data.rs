use crate::prelude::*;
use sdl2::keyboard::Keycode;



pub struct ProgramData<'a> {

    pub frame_count: u64,
    pub exit: bool,

    pub camera: Camera,
    pub textures: ProgramTextures<'a>,
    pub font: Font<'a, 'a>,
    pub keys_pressed: HashMap<Keycode, ()>,

    pub cells: EntityContainer<Cell>,
    pub food: EntityContainer<Food>,

}





pub struct Cell {
    pub health: f64,
    pub energy: f64,
    pub material: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub entity: RawEntity,
    pub connected_cells: Vec<EntityID>,
}

impl Cell {
    pub fn new (x: f64, y: f64, health: f64, energy: f64, material: f64) -> Self {
        Self {
            health,
            energy,
            material,
            x_vel: 0.,
            y_vel: 0.,
            entity: RawEntity::new(x, y, 1., 1.),
            connected_cells: vec!(),
        }
    }
    pub fn new_with_vel (x: f64, y: f64, health: f64, energy: f64, material: f64, x_vel: f64, y_vel: f64) -> Self {
        Self {
            health,
            energy,
            material,
            x_vel,
            y_vel,
            entity: RawEntity::new(x, y, 1., 1.),
            connected_cells: vec!(),
        }
    }
    pub fn pos_change (&self, other: &Cell) -> (f64, f64) {
        (other.entity.x - self.entity.x, other.entity.y - self.entity.y)
    }
    pub fn vel_change(&self, other: &Cell) -> (f64, f64) {
        (other.x_vel - self.x_vel, other.y_vel - self.y_vel)
    }
    pub fn distance_to (&self, other: &Cell) -> f64 {
        let (self_x, self_y) = (self.entity.x, self.entity.y);
        let (other_x, other_y) = (other.entity.x, other.entity.y);
        let (dx, dy) = (other_x - self_x, other_y - self_y);
        (dx * dx + dy * dy).sqrt()
    }
}

impl Entity for Cell {
    fn get_texture<'a> (&self, textures: &'a ProgramTextures<'a>) -> &'a Texture<'a> {
        &textures.circle
    }
}

impl AsRef<RawEntity> for Cell {
    fn as_ref (&self) -> &RawEntity {
        &self.entity
    }
}

impl AsMut<RawEntity> for Cell {
    fn as_mut (&mut self) -> &mut RawEntity {
        &mut self.entity
    }
}





pub struct Food {
    pub amount: f64,
    pub entity: RawEntity,
}

impl Food {
    pub fn new (x: f64, y: f64, amount: f64) -> Self {
        Self {
            amount,
            entity: RawEntity::new(x, y, 0.5, 0.5),
        }
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
