use bevy::app;
mod entities;

fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    app::App::new()
        .add_startup_system(entities::make_people)
        .add_system(hello_world)
        .add_system(entities::greet_people)
        .run();
}
