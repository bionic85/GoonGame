mod level1;

use bevy::prelude::*;

fn main() {
    // crazy builder pattern taaaaa
    App::new().add_plugins(DefaultPlugins).add_plugins(level1::Level1).run();
}
