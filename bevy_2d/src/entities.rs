use bevy::ecs::prelude::*;
use bevy::prelude::Plugin;
use bevy::time::prelude::*;

#[derive(Component)]
pub struct Age(usize);

#[derive(Component)]
pub struct Name(String);

#[derive(Bundle)]
struct Person {
    a: Name,
    b: Age
}

pub struct PeoplePlugin;

pub struct PeopleTimer(Timer);

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PeopleTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(make_people_bundle)
            .add_system(greet_people_bundle);
    }
}

fn make_people_bundle(mut cmds: Commands) -> () {
    cmds.spawn_bundle(Person {a: Name("Example".to_string()), b: Age(0)})
        .insert_bundle(Person {a: Name("PF".to_string()), b: Age(19)});
    cmds.spawn_bundle(Person {a: Name("Example".to_string()), b: Age(0)})
        .insert_bundle(Person {a: Name("JL".to_string()), b: Age(47)});
}

fn greet_people_bundle(time: Res<Time>, mut timer: ResMut<PeopleTimer>, query: Query<&Name, &Age>) -> () {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
