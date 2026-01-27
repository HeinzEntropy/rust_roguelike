use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        mb.fill(TileType::Floor); //把地图全部填充为墙
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant(); //把amulet放在最远的可达位置
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }
        mb
    }
}
