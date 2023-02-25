use crate::prelude::*;





pub struct WorldUpdates {
    pub list: Vec<UpdateType>,
}

pub enum UpdateType {

    AddFood (Food),

    ChangeCellHealth (usize, f64),
    ChangeCellEnergy (usize, f64),
    ChangeCellMaterial (usize, f64),
    SetCellXAndY (usize, f64, f64),
    ChangeCellXVel (usize, f64),
    ChangeCellYVel (usize, f64),
    ChangeCellXAndYVel (usize, f64, f64),
    SetCellIsActive (usize, bool),
    SetCellShouldBeRemoved (usize, bool),
    ChangeCellFatExtraEnergy (usize, f64),
    ChangeCellFatExtraMaterial (usize, f64),

}

impl WorldUpdates {
    pub fn new() -> Self {
        Self {list: vec!()}
    }
    pub fn push (&mut self, update: UpdateType) {
        self.list.push(update);
    }
    pub fn append (&mut self, mut other: Self) {
        self.list.append(&mut other.list);
    }
}
