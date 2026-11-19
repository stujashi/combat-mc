mod combat;
mod player;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Combat MC".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)))
        .add_plugins((world::WorldPlugin, player::PlayerPlugin, combat::CombatPlugin))
        .run();
}
