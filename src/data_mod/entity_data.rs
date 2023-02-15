use crate::prelude::*;



// -------------------------------------------------------------------------------- //
// WARNING: entity.curr_grid_ feilds need to stay synced with world.entities_by_pos //
// -------------------------------------------------------------------------------- //

pub struct World {
    pub entities_list: HashMap<EntityID, Entity>,
    pub new_entity_id: EntityID,
    pub entities_by_pos: Vec<Vec<EntityID>>, // 1d array for grid, 1d array for entities in that slot
}

pub type EntityID = u32;

pub struct Entity  {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub current_grid_x: usize,
    pub current_grid_y: usize,
    pub data: EntityData,
}

pub enum EntityData {
    Cell {
        x_vel: f64,
        y_vel: f64,
    }
}





impl World {

    pub fn new() -> Self {
        let grid = vec![vec!(); GRID_WIDTH * GRID_HEIGHT];
        Self {
            entities_list: HashMap::new(),
            new_entity_id: 0,
            entities_by_pos: grid,
        }
    }

    pub fn add_entity (&mut self, mut entity: Entity) -> Option<EntityID> {

        if self.entities_list.len() >= MAX_ENTITIES_COUNT {return None;}
        entity.do_simple_pos_update();
        let (current_grid_x, current_grid_y) = (entity.current_grid_x, entity.current_grid_y);

        // add to list
        let mut entity_id;
        loop {
            entity_id = self.new_entity_id;
            self.new_entity_id += 1;
            match self.entities_list.insert(entity_id, entity) {
                Some(returned_entity) => entity = returned_entity,
                None => break,
            }
        }

        // add to entities_by_pos
        self.entities_by_pos[current_grid_x + current_grid_y * GRID_WIDTH].push(entity_id);

        Some(entity_id)
    }

    pub fn sync_feilds (&mut self) {
        for (id, entity) in self.entities_list.iter_mut() {

            // update entities_by_pos
            // remove from old slot
            let grid_x = entity.current_grid_x;
            let grid_y = entity.current_grid_y;
            if entity.x as usize == grid_x && entity.y as usize == grid_y {continue;}
            let old_slot = &mut self.entities_by_pos[grid_x + grid_y * GRID_WIDTH];
            let entity_index = fns::find_item_index(old_slot, id).unwrap_or_else(|| panic!("An entity was listed as being in slot ({grid_x}, {grid_y}), but that entity's ID was not in that slot"));
            old_slot.remove(entity_index);
            // add to new slot
            entity.do_simple_pos_update();
            let grid_x = entity.current_grid_x;
            let grid_y = entity.current_grid_y;
            let new_slot = &mut self.entities_by_pos[grid_x + grid_y * GRID_WIDTH];
            new_slot.push(*id);

        }
    }

}



impl Entity {

    pub fn new (x: f64, y: f64, width: f64, height: f64, data: EntityData) -> Self {
        Self {
            x,
            y,
            width,
            height,
            current_grid_x: x as usize,
            current_grid_y: y as usize,
            data,
        }
    }

    pub fn do_simple_pos_update (&mut self) {
        self.x = self.x.clamp(0., GRID_WIDTH  as f64 - 0.00000001);
        self.y = self.y.clamp(0., GRID_HEIGHT as f64 - 0.00000001);
        self.current_grid_x = self.x as usize;
        self.current_grid_y = self.y as usize;
    }

}



impl EntityData {

    pub fn get_texture<'a> (&self, textures: &'a ProgramTextures<'a>) -> &'a Texture<'a> {
        match self {
            Self::Cell {..} => &textures.circle,
        }
    }

}
