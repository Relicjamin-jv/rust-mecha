use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Collin".to_string())));
    commands.spawn((Person, Name("Tanya".to_string())));
}

fn hello_world(){
    println!("Hello World!");
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people)).run();
}
