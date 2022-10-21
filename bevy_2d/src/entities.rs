use bevy::ecs::prelude::*;

#[derive(Component)]
pub struct Age(usize);

#[derive(Component)]
pub struct Name(String);

#[derive(Bundle)]
pub struct Person {
    a: Name,
    b: Age
}

pub fn make_people(mut cmds: Commands) -> () {
    cmds.spawn_bundle(Person {a: Name("Example".to_string()), b: Age(0)})
        .insert_bundle(Person {a: Name("PF".to_string()), b: Age(19)})
        .insert_bundle(Person {a: Name("JL".to_string()), b: Age(47)});
}

pub fn greet_people(query: Query<&Name, &Age>) -> () {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
