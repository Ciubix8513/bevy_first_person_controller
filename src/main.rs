use bevy::prelude::*;

use player::PlayerPlugin;
use world::World;

mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(World)
        .run();
}
