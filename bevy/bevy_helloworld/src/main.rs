use bevy::prelude::*;

struct Person;
struct Name(String);
struct GreetTimer(Timer);

fn hello_world() {
    println!("Hello, world!");
}

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    // update timer with time elapsed since last update
    // if the timer hasn't finished -> return
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    // simplest bevy program
    App::build()
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
