use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct Player {
    transform: Transform,
    goon_points: i32,
    goon_speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gooner);
        app.add_systems(Update, keyboard_inputs);
    }
}

fn spawn_gooner(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    let player = Player {transform: Transform::from_xyz(0.0,0.0,0.0), goon_points: 0, goon_speed: 0.0};
    commands.insert_resource(player)
}
fn update_gooner(player: ResMut<Player>, mut cube: Query<(Entity, &mut Transform)>) {
    for (entity, mut transform) in cube.iter_mut() {
        *transform = player.transform;
    }
}

// fn player_move(mut player: Player, offset: Vec3) {
//     player.pos += offset;
// }

fn calculate_goon_stage(player: Player) -> i32 {
    return 0; //TODO: implement goon stages so that the player model grows
}

// fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
//     for mut transform in &mut query {
//         transform.rotate_y(time.delta_secs() / 2.);
//     }
// }

pub fn keyboard_inputs(mut player: ResMut<Player>, presses: Res<ButtonInput<KeyCode>>) {
    let mut dir = Vec3::ZERO;
    let speed = player.goon_speed;
    if presses.pressed(KeyCode::KeyW) {
        dir += Vec3::X;
    } else if presses.pressed(KeyCode::KeyA) {
        dir += Vec3::Y;
    } else if presses.pressed(KeyCode::KeyS) {
        dir += -Vec3::X;
    } else if presses.pressed(KeyCode::KeyD) {
        dir += -Vec3::Y;
    } else if presses.pressed(KeyCode::Space) {
        // TODO: Jump? idk?
    }
    //println!("{:?}", dir);
    player.transform.translation += dir * speed;
}
