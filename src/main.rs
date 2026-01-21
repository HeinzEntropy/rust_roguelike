#![warn(clippy::pedantic)]

mod camera;
mod map;
mod map_builder;
//mod player;已经被弃用，将转为Legion的ECS模式
mod components;
mod spawner;
mod system;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    //pub use crate::player::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::system::*;
    pub use crate::turn_state::*;
}

use prelude::*;

/*struct State {//原有的类/方法模式
    map: Map,
    //player: Player,
    camera: Camera,
}*/
struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}
impl State {
    /*fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            //player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }*/
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos|{
                spawn_enemy(&mut ecs, &mut rng, pos);
            });
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            systems: build_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        //self.player.update(ctx, &self.map, &mut self.camera);
        //self.map.render(ctx, &self.camera);
        //self.player.render(ctx, &self.camera);
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new() // (1)
        .with_title("Rust Roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // (2)
        .with_tile_dimensions(32, 32) // (3)
        .with_resource_path("resources/") // (4)
        .with_font("dungeonfont.png", 32, 32) // (5)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // (6)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // (7)
        .build()?;

    main_loop(context, State::new())
}
