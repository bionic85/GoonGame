mod level1;
mod player;

use bevy::prelude::*;
use player::*;

fn main() {
    // crazy builder pattern taaaaa
    App::new().add_plugins(DefaultPlugins).add_plugins(level1::Level1).add_plugins(player::PlayerPlugin).run();
}
