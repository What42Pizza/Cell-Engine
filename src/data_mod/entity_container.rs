use crate::prelude::*;



//-----------------------------------------------------------------------------------------------//
// WARNING: RawEntity.curr_grid_ feilds need to stay synced with EntityContainer.entities_by_pos //
//-----------------------------------------------------------------------------------------------//

pub type EntityID = u32;

pub struct EntityContainer<T: Entity> {
    pub master_list: HashMap<EntityID, T>,
    pub new_entity_id: EntityID,
    pub entities_by_pos: Vec<Vec<EntityID>>, // 1d array for grid, 1d array for entities in that slot
}



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

pub trait Entity: AsRef<RawEntity> + AsMut<RawEntity> {
    fn get_texture<'a> (&self, textures: &'a ProgramTextures<'a>) -> &'a Texture<'a>;
}




impl<T: Entity> EntityContainer<T> {

    pub fn new() -> Self {
        let grid = vec![vec!(); GRID_WIDTH * GRID_HEIGHT];
        Self {
            master_list: HashMap::new(),
            new_entity_id: 0,
            entities_by_pos: grid,
        }
    }

    pub fn add_entity (&mut self, mut entity: T) -> Option<EntityID> {
        let raw_entity = entity.as_mut();

        if self.master_list.len() >= MAX_ENTITIES_COUNT {return None;}
        let (current_grid_x, current_grid_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);

        // add to list
        let mut entity_id;
        loop {
            entity_id = self.new_entity_id;
            self.new_entity_id += 1;
            match self.master_list.insert(entity_id, entity) {
                Some(returned_entity) => entity = returned_entity,
                None => break,
            }
        }

        // add to entities_by_pos
        self.entities_by_pos[current_grid_x + current_grid_y * GRID_WIDTH].push(entity_id);

        Some(entity_id)
    }

    pub fn sync_feilds (&mut self) {

        let keys: Vec<u32> = self.master_list.keys().copied().collect();

        // update entities_by_pos
        for id in keys {
            let entity = self.master_list.get_mut(&id).unwrap();
            let raw_entity = entity.as_mut();

            // remove if should_be_removed
            if raw_entity.should_be_removed {
                let (grid_x, grid_y) = (raw_entity.current_grid_x, raw_entity.current_grid_y);
                let slot = &mut self.entities_by_pos[grid_x + grid_y * GRID_WIDTH];
                let slot_pos = fns::find_item_index(slot, &id).unwrap();
                slot.remove(slot_pos);
                self.master_list.remove(&id);
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

    }

}
