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
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        /*map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| {
            spawn_enemy(&mut ecs, &mut rng, pos);
        });*/
        map_builder.monster_spawns.iter().for_each(|pos| {
            spawn_enemy(&mut ecs, &mut rng, *pos);
        });
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_schedule(),
            player_systems: build_player_schedule(),
            monster_systems: build_monster_schedule(),
        }
    }
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        /*map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| {
            spawn_enemy(&mut self.ecs, &mut rng, pos);
        });*/
        map_builder.monster_spawns.iter().for_each(|pos| {
            spawn_enemy(&mut self.ecs, &mut rng, *pos);
        });
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        //根据当前的TurnState，执行不同的系统
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
        }
        render_draw_buffer(ctx).expect("Render Error");
    }
}

/// 游戏的主函数，负责初始化游戏环境并启动主循环
/// 返回BError类型，用于处理可能的初始化错误
fn main() -> BError {
    // 创建BTermBuilder实例，用于配置游戏窗口和渲染环境
    let context = BTermBuilder::new()
        // 设置游戏窗口标题为"小张的地下城冒险"
        .with_title("小张的地下城冒险")
        // 设置游戏帧率上限为30.0fps，平衡游戏流畅度和性能消耗
        .with_fps_cap(30.0)
        // 设置游戏显示尺寸，使用预定义的DISPLAY_WIDTH和DISPLAY_HEIGHT常量
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // 设置每个游戏图块的像素尺寸为32x32
        .with_tile_dimensions(32, 32)
        // 设置游戏资源文件的路径
        .with_resource_path("resources/")
        // 添加32x32的地下城字体，用于渲染游戏场景
        .with_font("dungeonfont.png", 32, 32)
        // 添加8x8的终端字体，用于渲染文本信息
        .with_font("terminal8x8.png", 8, 8)
        // 创建主游戏控制台（控制台0），带背景，使用地下城字体
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // 创建第二控制台（控制台1），不带背景，使用地下城字体
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // 创建第三控制台（控制台2），不带背景，使用终端字体，用于显示游戏UI
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        // 构建BTerm上下文，?操作符用于错误传播
        .build()?;

    // 启动游戏主循环，传入构建好的上下文和初始游戏状态
    // main_loop会持续运行，直到游戏结束或发生错误
    main_loop(context, State::new())
}
