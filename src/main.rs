use bevy::prelude::*;

fn main() {
    let mut board = Board {height: 10, width: 20, grid: vec![vec![Cell{alive: false}]]};
    board.intialize(0);
    board.print();
}

struct Board {
    height: usize,
    width: usize,
    grid: Vec<Vec<Cell>>,
}

impl Board {
    fn intialize(&mut self, seed: usize) {
        self.grid = match seed {
            _ => vec![vec![Cell{ alive: false }; self.width]; self.height]
        }
    }

    fn print(&self) {
        for i in &self.grid {
            for j in i {
                print!("{} ", j.ascii_display());
            }
            println!();
        }
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
struct Cell
{
    alive: bool
}

impl Cell {

    fn status(&self) -> bool {
        self.alive
    }

    fn die(&mut self) {
        self.alive = false;
    }

    fn revive(&mut self) {
        self.alive = true;
    }

    fn switch(&mut self) {
        self.alive = !self.alive;
    }

    fn ascii_display(&self) -> char {
        if self.alive {
            'o'
        } else {
            'x'
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ramtet".to_string())));
    commands.spawn((Person, Name("Sepca".to_string())));
    commands.spawn((Person, Name("Meti".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}
