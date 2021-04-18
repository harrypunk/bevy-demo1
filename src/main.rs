use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_buttons.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, apply_red.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, apply_blue.system())
        .run();
}

struct BtnRed;
struct BtnBlue;

fn add_buttons(mut commands: Commands) {
    commands.spawn().insert(BtnRed);
    commands.spawn().insert(BtnRed);

    commands.spawn().insert(BtnBlue);
}

fn apply_red(query: Query<&BtnRed>) {
    query.iter().for_each(|_| {
        println!("apply red style");
    });
}

fn apply_blue(query: Query<&BtnBlue>) {
    query.iter().for_each(|_| {
        println!("apply blue style");
    });
}
