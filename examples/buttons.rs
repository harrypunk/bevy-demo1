use bevy::prelude::*;
use game1::button;

static FONT1: &str = "/usr/share/fonts/TTF/Hack-Italic.ttf";

fn main() {
    App::build()
        .add_startup_system(add_resource.system())
        .add_startup_system(setup.system())
        .add_plugin(button::StylePlugin)
        .add_plugins(DefaultPlugins)
        .add_system(change_status.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(button::Status::Normal)
        .insert(button::Display)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Hello Button",
                    TextStyle {
                        font: asset_server.load(FONT1),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn add_resource(mut cm: Commands, mut assets: ResMut<Assets<ColorMaterial>>) {
    cm.insert_resource(button::DisplayStyle {
        normal: assets.add(Color::BLUE.into()),
        focused: assets.add(Color::YELLOW.into()),
        disabled: assets.add(Color::GRAY.into()),
    });
}

fn change_status(
    mut cm: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<(Entity,), (With<button::Display>,)>,
) {
    let st = {
        if input.pressed(KeyCode::R) {
            button::Status::Focused
        } else if input.pressed(KeyCode::T) {
            button::Status::Disabled
        } else {
            button::Status::Normal
        }
    };

    query.iter().for_each(|(ent,)| {
        cm.entity(ent).insert(st.clone());
    });
}
