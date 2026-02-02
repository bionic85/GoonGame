// Will hold all the info for level1

use bevy::mesh::PlaneMeshBuilder;
use bevy::prelude::*;
pub struct Level1;

impl Plugin for Level1 {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level1);
        app.add_systems(Update, level1loop);
    }
}

fn spawn_level1(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let floor = Mesh3d(meshes.add(Plane3d {normal: Dir3::from_xyz(0.,1.,0.).unwrap(), half_size: Vec2::new(100.,100.)}));
    let floorMat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.),
        ..default()
    }));
    commands.spawn((Transform::from_xyz(0.0, 0.0, 0.0), floor, floorMat));

    // let floor = Plane3d {
    //     mesh: meshes.add(Plane3d::default().mesh().size(15.,15.));
    // }
}

fn level1loop() {
    //TODO: goon loop
}