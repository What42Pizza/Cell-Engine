use crate::prelude::*;
use sdl2::keyboard::Keycode;



pub struct ProgramData<'a> {

    pub frame_count: u64,
    pub exit: bool,

    pub camera: Camera,
    pub textures: ProgramTextures<'a>,
    pub font: Font<'a, 'a>,
    pub keys_pressed: HashMap<Keycode, ()>,

    pub world: World,

}



pub struct World {
    pub cells: HashMap<CellID, Cell>,
    pub current_cell_id: CellID,
    pub grid: Box<[[Vec<CellID>; GRID_HEIGHT]; GRID_WIDTH]>,
}



impl World {

    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            current_cell_id: 0,
            grid: fns::init_boxed_2d_array(|_, _| vec!()),
        }
    }

    pub fn add_cell (&mut self, mut cell: Cell) -> Option<CellID> {

        if self.cells.len() >= MAX_CELLS_COUNT {return None;}
        let (cell_x, cell_y) = (
            cell.x.clamp(0., GRID_WIDTH  as f64) as usize,
            cell.y.clamp(0., GRID_HEIGHT as f64) as usize,
        );

        // add to list
        let mut cell_id;
        loop {
            cell_id = self.current_cell_id;
            self.current_cell_id += 1;
            match self.cells.insert(cell_id, cell) {
                Some(returned_cell) => cell = returned_cell,
                None => break,
            }
        }

        // add to grid
        self.grid[cell_x][cell_y].push(cell_id);

        Some(cell_id)
    }

}





pub type CellID = u32;

pub struct Cell {
    pub x: f64,
    pub y: f64,
    pub xvel: f64,
    pub yvel: f64,
}

impl Cell {
    pub fn new (x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            xvel: 0.,
            yvel: 0.,
        }
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
