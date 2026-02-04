use crate::prelude::*;

pub struct Dungeon_Theme{}
impl Dungeon_Theme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
impl MapTheme for Dungeon_Theme {
    fn tile_to_render(&self, tile: TileType) -> FontCharType {
        match tile {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        }
    }
}

pub struct Forest_Theme{}
impl Forest_Theme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
impl MapTheme for Forest_Theme {
    fn tile_to_render(&self, tile: TileType) -> FontCharType {
        match tile {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
        }
    }
}