use crate::prelude::*;

#[system]
#[read_component(Rest)]
#[read_component(RestCounter)]
#[read_component(Health)]
pub fn rest(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // 1. 收集所有需要的信息
    let mut rest_info = Vec::new();

    // 查询所有Rest组件
    <(Entity, &Rest)>::query()
        .iter(ecs)
        .for_each(|(rest_entity, rest)| {
            let who_rested = rest.who_rested;

            // 获取当前计数器值
            let current_counter = if let Ok(entry) = ecs.entry_ref(who_rested) {
                if let Ok(counter) = entry.get_component::<RestCounter>() {
                    counter.0
                } else {
                    0
                }
            } else {
                0
            };

            // 获取当前健康值
            let current_health = if let Ok(entry) = ecs.entry_ref(who_rested) {
                if let Ok(health) = entry.get_component::<Health>() {
                    (health.current, health.max)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            };

            rest_info.push((*rest_entity, who_rested, current_counter, current_health));
        });

    // 2. 使用CommandBuffer执行所有修改操作
    for (rest_entity, who_rested, current_counter, (current_hp, max_hp)) in rest_info {
        // 更新休息计数器
        let new_counter = current_counter + 1;

        // 根据计数器增加生命值（这里实现为每10次休息增加1点）
        if new_counter >= 10 {
            let heal_amount = 1;
            let new_hp = (current_hp + heal_amount).min(max_hp);

            // 更新健康值
            commands.add_component(
                who_rested,
                Health {
                    current: new_hp,
                    max: max_hp,
                },
            );
            commands.add_component(who_rested, RestCounter(0));
        } else {
            commands.add_component(who_rested, RestCounter(new_counter));
        }

        // 移除已处理的Rest组件
        commands.remove(rest_entity);
    }
}
