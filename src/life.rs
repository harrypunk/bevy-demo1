use bevy::prelude::*;

pub struct RemoveTag;

pub struct Manager;

impl Plugin for Manager {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PostUpdate, despawn.system());
    }
}

fn despawn(mut commands: Commands, query: Query<Entity, With<RemoveTag>>) {
    query.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}

#[cfg(test)]
mod tests {
    use super::{Manager, RemoveTag};
    use bevy::prelude::*;

    struct SomeObj;

    fn check_after_remove(query: Query<&SomeObj>) {
        let n = query.iter().count();
        assert_eq!(n, 2);
    }

    fn create_obj(mut cm: Commands) {
        cm.spawn().insert(SomeObj);
        cm.spawn().insert(SomeObj);
        (0..10).for_each(|_| {
            cm.spawn().insert(SomeObj).insert(RemoveTag);
        });
    }

    #[test]
    fn despawn_entity() {
        App::build()
            .add_startup_system(create_obj.system())
            .add_plugin(Manager)
            .add_system_to_stage(CoreStage::Last, check_after_remove.system())
            .run();
    }
}
