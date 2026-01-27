use crate::prelude::*;
use super::MapArchitect;

pub struct RoomArchitect{}

impl MapArchitect for RoomArchitect{
    fn new(&mut self, rng: &mut RandomNumberGenerator)->MapBuilder{
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        mb.fill(TileType::Wall);//把地图全部填充为墙
        mb.build_random_rooms(rng);//构造随机大小的房间
        mb.build_corridors(rng);//构造走廊
        mb.player_start = mb.rooms[0].center();// 把player放在第一个房间的中心
        mb.amulet_start = mb.find_most_distant();//把amulet放在最远的可达位置
        for room in mb.rooms.iter().skip(1){
            mb.monster_spawns.push(room.center());
        }
        mb//返回mb
    }
}
