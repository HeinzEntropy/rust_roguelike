use crate::prelude::*;

const PLAYER_MAX_HEALTH: i32 = 50;
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: PLAYER_MAX_HEALTH,
            max: PLAYER_MAX_HEALTH,
        },
        FeildOfView::new(8),
        RestCounter(0),
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_enemy(ecs, rng, pos),
    }
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, view_radius) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: glyph,
        },
        //MovingRandomly{},
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FeildOfView::new(view_radius),
    ));
}

pub fn goblin() -> (i32, String, FontCharType, i32) {
    (1, "Goblin".to_string(), to_cp437('g'), 6)
}
pub fn orc() -> (i32, String, FontCharType, i32) {
    (2, "Orc".to_string(), to_cp437('O'), 5)
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(YELLOW, BLACK),
            glyph: to_cp437('!'),
        },
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap,
    ));
}
