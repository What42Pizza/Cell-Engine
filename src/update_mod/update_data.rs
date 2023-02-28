use crate::prelude::*;





pub struct WorldUpdates {
    pub changes: Vec<ChangeUpdate>,
    pub additions: Vec<AdditionUpdate>,
}

pub enum ChangeUpdate {

    ChangeCellHealth (usize, f64),
    ChangeCellEnergy (usize, f64),
    ChangeCellMaterial (usize, f64),
    SetCellPos (usize, f64, f64),
    ChangeCellVel (usize, f64, f64),
    SetCellIsActive (usize, bool),
    SetCellShouldBeRemoved (usize, bool),
    ChangeCellFatExtraEnergy (usize, f64),
    ChangeCellFatExtraMaterial (usize, f64),

}

pub enum AdditionUpdate {
    Food (Food),
}



impl WorldUpdates {
    pub fn new() -> Self {
        Self {
            changes: vec!(),
            additions: vec!(),
        }
    }
    pub fn push_change (&mut self, change: ChangeUpdate) {
        self.changes.push(change);
    }
    pub fn push_addition (&mut self, addition: AdditionUpdate) {
        self.additions.push(addition);
    }
}





pub struct CellChangesGroup {
    pub x_vel_change: f64,
    pub y_vel_change: f64,
    pub energy_change: f64,
    pub material_change: f64,
}

impl CellChangesGroup {
    pub fn new() -> Self {
        Self {
            x_vel_change: 0.,
            y_vel_change: 0.,
            energy_change: 0.,
            material_change: 0.
        }
    }
    pub fn add_self_to_world_updates (self, all_updates: &mut WorldUpdates, cell_id: EntityID) {
        all_updates.changes.push(ChangeUpdate::ChangeCellVel (cell_id.0, self.x_vel_change, self.y_vel_change));
        all_updates.changes.push(ChangeUpdate::ChangeCellEnergy (cell_id.0, self.energy_change));
        if self.material_change != 0. {
            all_updates.changes.push(ChangeUpdate::ChangeCellMaterial (cell_id.0, self.material_change));
        }
    }
}
