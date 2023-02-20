use crate::prelude::*;





pub struct Cell {
    pub is_active: bool,
    pub health: f64,
    pub energy: f64,
    pub material: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub x_vel_copy: f64,
    pub y_vel_copy: f64,
    pub raw_cell: RawCell,
    pub entity: RawEntity,
    pub connected_cells: Vec<EntityID>,
}

impl Cell {
    pub fn new (raw_cell: RawCell, x: f64, y: f64, health: f64, energy: f64, material: f64) -> Self {
        Self {
            is_active: true,
            health,
            energy,
            material,
            x_vel: 0.,
            y_vel: 0.,
            x_vel_copy: 0.,
            y_vel_copy: 0.,
            raw_cell,
            entity: RawEntity::new(x, y, 1., 1.),
            connected_cells: vec!(),
        }
    }
    pub fn new_with_vel (raw_cell: RawCell, x: f64, y: f64, health: f64, energy: f64, material: f64, x_vel: f64, y_vel: f64) -> Self {
        Self {
            is_active: true,
            health,
            energy,
            material,
            x_vel,
            y_vel,
            x_vel_copy: x_vel,
            y_vel_copy: y_vel,
            raw_cell,
            entity: RawEntity::new(x, y, 1., 1.),
            connected_cells: vec!(),
        }
    }
    pub fn pos_change_to (&self, other: &Cell) -> (f64, f64) {
        (other.entity.x - self.entity.x, other.entity.y - self.entity.y)
    }
    pub fn vel_change_to(&self, other: &Cell) -> (f64, f64) {
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





pub enum RawCell {
    Fat {extra_energy: f64, extra_material: f64},
    Photosynthesiser,
}
