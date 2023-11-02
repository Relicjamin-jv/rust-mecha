use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position {x : f32, y: f32}

fn print_position_system(query: Query<&Position>){
    for position in &query{
        println!("position: {} {}", position.x, position.y)
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        // Add some plugin stuff
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn hello_world(){
    println!("Hello World")
}

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Collin".to_string())));
    commands.spawn((Person, Name("Tanya".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer> ,query: Query<&Name, With<Person>>){
    if timer.0.tick(time.delta()).just_finished(){
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
