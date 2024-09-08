mod pawn;
use pawn::Pawn;

mod item;
use item::Item;

pub struct Grid {
    pub max_x: i32,
    pub max_y: i32,
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn get_all_tiles(&self) -> Vec<&Tile> {
        self.tiles
            .iter()
            .flat_map(|v| v.into_iter())
            .collect()
    }

    fn get_all_pawns(&self) -> Vec<&Pawn> {
        Self::get_all_tiles(self)
            .into_iter()
            .flat_map(|t| &t.pawns)
            .collect()
    }
}

pub struct Tile {
    terrain_type: Terrain,
    pawns: Vec<Pawn>,
    items: Vec<Item>,
}

pub enum Terrain {
    Floor,
    Wall,
}
