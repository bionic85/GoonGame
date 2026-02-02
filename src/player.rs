use bevy::prelude::*;

pub struct Player {
    pub(crate) pos: Vec3,
    pub(crate) goon_points: i32,
    pub(crate) goon_speed: f32,
}

pub fn player_move(mut player: Player) -> Player {
    player.pos = Vec3::ZERO;
    player
}

fn calculate_goon_stage(player: Player) -> i32 {
    return 0; //TODO: implement goon stages so that the player model grows
}

// fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
//     for mut transform in &mut query {
//         transform.rotate_y(time.delta_secs() / 2.);
//     }
// }