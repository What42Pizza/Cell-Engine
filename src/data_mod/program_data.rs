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



pub struct World {
    pub entities: HashMap<EntityID, Entity>,
    pub current_entity_id: EntityID,
    pub grid: Vec<Vec<Vec<EntityID>>>,
}

pub type EntityID = u32;

pub struct Entity  {
    pub x: f64,
    pub y: f64,
    pub data: EntityData
}

pub enum EntityData {
    Cell {

    }
}



impl World {

    pub fn new() -> Self {
        let mut grid = Vec::with_capacity(GRID_WIDTH);
        for _ in 0..GRID_WIDTH {
            let mut column = Vec::with_capacity(GRID_HEIGHT);
            for _ in 0..GRID_HEIGHT {
                column.push(vec!());
            }
            grid.push(column);
        }
        Self {
            entities: HashMap::new(),
            current_entity_id: 0,
            grid,
        }
    }

    pub fn add_entity (&mut self, mut entity: Entity) -> Option<EntityID> {

        if self.entities.len() >= MAX_ENTITIES_COUNT {return None;}
        let (x_int, y_int) = (
            entity.x.clamp(0., GRID_WIDTH  as f64) as usize,
            entity.y.clamp(0., GRID_HEIGHT as f64) as usize,
        );

        // add to list
        let mut entity_id;
        loop {
            entity_id = self.current_entity_id;
            self.current_entity_id += 1;
            match self.entities.insert(entity_id, entity) {
                Some(returned_entity) => entity = returned_entity,
                None => break,
            }
        }

        // add to grid
        self.grid[x_int][y_int].push(entity_id);

        Some(entity_id)
    }

}





pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64, // >1 means zoomed in, <1 means zoomed out
}

/*
to zoom in/out centered on a point:
1: find where the point is in the world pre-zoom
2: change the zoom
3: find where the point is in the world post-zoom
4: find the difference in position
5: move the camera so that the point is the same as pre-zoom
*/



pub struct ProgramTextures<'a> {
    pub ground: Texture<'a>,
    pub circle: Texture<'a>,
}
