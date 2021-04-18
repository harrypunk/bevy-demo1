use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_buttons.system())
        .run();
}

struct BtnRed;
struct BtnBlue;

fn add_buttons(mut commands: Commands) {
    commands.spawn().insert(BtnRed);
    commands.spawn().insert(BtnRed);

    commands.spawn().insert(BtnBlue);
}
