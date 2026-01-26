pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render{//渲染组件
    pub color : ColorPair,
    pub glyph : FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;//玩家组件

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;//敌人组件

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;//随机移动组件

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove{//移动实现组件
    pub entity : Entity,
    pub destination : Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack{//攻击实现组件
    pub attacker : Entity,
    pub victim : Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health{//生命值组件
    pub current : i32,
    pub max : i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);//名称组件

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;//追逐玩家组件

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct  Item;//物品组件
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct  AmuletOfYala;//雅拉的护身符

