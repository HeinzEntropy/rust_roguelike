use crate::prelude::*;

const MAX_HEALTH: i32 = 20;
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: MAX_HEALTH,
            max: MAX_HEALTH,
        },
        FeildOfView::new(8),
    ));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph,view_radius) = match rng.roll_dice(1, 10) {
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
