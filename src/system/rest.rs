use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(Rest)]
#[read_component(Entity)]
pub fn rest(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let rest_and_object = <(Entity, &Rest)>::query()
        .iter(ecs)
        .filter(|(_, rest)| {
            ecs.entry_ref(rest.who_rested)
                .unwrap()
                .get_component::<Health>()
                .is_ok()
        })
        .map(|(entity, rest)| (*entity,rest.who_rested))
        .collect::<Vec<(Entity,Entity)>>();
     rest_and_object.iter().for_each(|(entity, object)|{
        if let Ok(health) = ecs.entry_mut(*object).unwrap().get_component_mut::<Health>() {
            health.current = i32::min(health.current + 1, health.max);
        }
        commands.remove(*entity);
     });
}
