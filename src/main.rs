mod level1;
mod player;

use bevy::prelude::*;
use player::*;

fn main() {
    let player = Player {pos: Vec3::ZERO, goon_points: 0, goon_speed: 0.0};
    // crazy builder pattern taaaaa
    App::new().add_plugins(DefaultPlugins).add_plugins(level1::Level1).run();
}
