use std::ops::Deref;

use crate::prelude::*;



//-----------------------------------------------------------------------------------------------//
// WARNING: RawEntity.curr_grid_ feilds need to stay synced with EntityContainer.entities_by_pos //
//-----------------------------------------------------------------------------------------------//

pub type EntityID = (usize, u32);

pub struct EntityContainer<T: Entity> {
    pub master_list: Vec<(Option<T>, u32)>,
    pub current_index: usize,
    pub empty_slots: u32,
    pub entities_by_pos: Vec<Vec<EntityID>>, // 1d array for grid, 1d array for entities in that slot
}



#[derive(Debug, Clone)]
pub struct RawEntity {
    pub should_be_removed: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub current_grid_x: usize,
    pub current_grid_y: usize,
}



impl RawEntity {
    pub fn new (x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            should_be_removed: false,
            x,
            y,
            width,
            height,
            current_grid_x: x as usize,
            current_grid_y: y as usize,
        }
    }
}

pub trait Entity {
    fn get_texture<'a> (&self, textures: &'a ProgramTextures<'a>) -> &'a Texture<'a>;
}

pub trait Transform {
    type Target;
    fn transform (&self) -> Self::Target;
}





impl<T: Entity> EntityContainer<T> {

    pub fn new() -> Self {
        let grid = vec![vec!(); GRID_WIDTH * GRID_HEIGHT];
        Self {
            master_list: vec!(),
            current_index: 0,
            empty_slots: 0,
            entities_by_pos: grid,
        }
    }

    pub fn get (&self, id: EntityID) -> Option<&T> {
        let entity_data = &self.master_list[id.0];
        let Some(entity) = &entity_data.0 else {return None;};
        fns::some_if(entity_data.1 == id.1, || entity)
    }

    pub fn get_mut (&mut self, id: EntityID) -> Option<&mut T> {
        let entity_data = &mut self.master_list[id.0];
        let Some(entity) = &mut entity_data.0 else {return None;};
        fns::some_if(entity_data.1 == id.1, || entity)
    }

    pub fn id_is_valid (&self, id: EntityID) -> bool {
        let entity_data = &self.master_list[id.0];
        entity_data.0.is_some() && entity_data.1 == id.1
    }

}



impl<T: Entity + AsRef<RawEntity> + AsMut<RawEntity>> EntityContainer<T> {
    
    pub fn referenced_add_entity (&mut self, entity: T) -> Option<EntityID> {
        let raw_entity = entity.as_ref();

        if self.master_list.len() >= MAX_ENTITIES_COUNT {return None;}
        let (current_grid_x, current_grid_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);

        // add to master list
        let entity_id;
        let master_list_len = self.master_list.len();
        if self.empty_slots as f64 / master_list_len as f64 >= 0.05 {
            // re-use slot
            while self.master_list[self.current_index].0.is_some() {
                self.current_index = (self.current_index + 1) % master_list_len;
            }
            let existing_slot = &self.master_list[self.current_index];
            entity_id = (self.current_index, existing_slot.1 + 1);
            self.master_list[entity_id.0] = (Some(entity), entity_id.1);
            self.empty_slots -= 1;
        } else {
            // add to new slot
            entity_id = (master_list_len, 0);
            self.master_list.push((Some(entity), entity_id.1));
        }

        // add to entities_by_pos
        self.entities_by_pos[current_grid_x + current_grid_y * GRID_WIDTH].push(entity_id);

        Some(entity_id)
    }

    pub fn referenced_sync_feilds (&mut self) {

        let mut indicies_to_erase = vec!();

        // update entities_by_pos
        for (i, entity_data) in self.master_list.iter_mut().enumerate() {
            let Some(entity) = entity_data.0.as_mut() else {continue;};
            let raw_entity = entity.as_mut();
            let id = (i, entity_data.1);

            // remove if should_be_removed
            if raw_entity.should_be_removed {
                let (grid_x, grid_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);
                let slot = &mut self.entities_by_pos[grid_x + grid_y * GRID_WIDTH];
                let slot_pos = fns::find_item_index(slot, &id).unwrap();
                slot.remove(slot_pos);
                indicies_to_erase.push(id.0);
                self.empty_slots += 1;
                continue;
            }

            // skip if already synced
            let (old_x, old_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);
            let (new_x, new_y) = (raw_entity.x as usize, raw_entity.y as usize);
            if old_x == new_x && old_y == new_y {continue;}

            // sync
            raw_entity.current_grid_x = new_x;
            raw_entity.current_grid_y = new_y;

            // remove from old slot
            let old_slot = &mut self.entities_by_pos[old_x + old_y * GRID_WIDTH];
            let entity_index = fns::find_item_index(old_slot, &id).unwrap_or_else(|| panic!("An entity was listed as being in slot ({old_x}, {old_y}), but that entity's ID was not in that slot"));
            old_slot.remove(entity_index);

            // add to new slot
            let new_slot = &mut self.entities_by_pos[new_x + new_y * GRID_WIDTH];
            new_slot.push(id);

        }

        for current_index in indicies_to_erase {
            self.master_list[current_index].0 = None;
        }

    }

}



// TODO: find a way to remove this duplication
impl<T: Entity + AsRef<AtomicRefCell<dyn AsRef<RawEntity>>> + AsMut<AtomicRefCell<dyn AsMut<RawEntity>>>> EntityContainer<T> {
    
    pub fn locked_add_entity (&mut self, entity: T) -> Option<EntityID> {
        let raw_entity_1 = entity.as_ref().borrow();
        let raw_entity_2 = raw_entity_1.as_ref();

        if self.master_list.len() >= MAX_ENTITIES_COUNT {return None;}
        let (current_grid_x, current_grid_y) = (raw_entity_2.current_grid_x, raw_entity_2.current_grid_y);
        drop(raw_entity_1);

        // add to master list
        let entity_id;
        let master_list_len = self.master_list.len();
        if self.empty_slots as f64 / master_list_len as f64 >= 0.05 {
            // re-use slot
            while self.master_list[self.current_index].0.is_some() {
                self.current_index = (self.current_index + 1) % master_list_len;
            }
            let existing_slot = &self.master_list[self.current_index];
            entity_id = (self.current_index, existing_slot.1 + 1);
            self.master_list[entity_id.0] = (Some(entity), entity_id.1);
            self.empty_slots -= 1;
        } else {
            // add to new slot
            entity_id = (master_list_len, 0);
            self.master_list.push((Some(entity), entity_id.1));
        }

        // add to entities_by_pos
        self.entities_by_pos[current_grid_x + current_grid_y * GRID_WIDTH].push(entity_id);

        Some(entity_id)
    }

    pub fn locked_sync_feilds (&mut self) {

        let mut indicies_to_erase = vec!();

        // update entities_by_pos
        for (i, entity_data) in self.master_list.iter_mut().enumerate() {
            let Some(entity) = entity_data.0.as_mut() else {continue;};
            let mut raw_entity = entity.as_mut().borrow_mut();
            let raw_entity = raw_entity.as_mut();
            let id = (i, entity_data.1);

            // remove if should_be_removed
            if raw_entity.should_be_removed {
                let (grid_x, grid_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);
                let slot = &mut self.entities_by_pos[grid_x + grid_y * GRID_WIDTH];
                let slot_pos = fns::find_item_index(slot, &id).unwrap();
                slot.remove(slot_pos);
                indicies_to_erase.push(id.0);
                self.empty_slots += 1;
                continue;
            }

            // skip if already synced
            let (old_x, old_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);
            let (new_x, new_y) = (raw_entity.x as usize, raw_entity.y as usize);
            if old_x == new_x && old_y == new_y {continue;}

            // sync
            raw_entity.current_grid_x = new_x;
            raw_entity.current_grid_y = new_y;

            // remove from old slot
            let old_slot = &mut self.entities_by_pos[old_x + old_y * GRID_WIDTH];
            let entity_index = fns::find_item_index(old_slot, &id).unwrap_or_else(|| panic!("An entity was listed as being in slot ({old_x}, {old_y}), but that entity's ID was not in that slot"));
            old_slot.remove(entity_index);

            // add to new slot
            let new_slot = &mut self.entities_by_pos[new_x + new_y * GRID_WIDTH];
            new_slot.push(id);

        }

        for current_index in indicies_to_erase {
            self.master_list[current_index].0 = None;
        }

    }

}
