use bevy::prelude::*;
pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(Startup, spawn)
       ; 
    }
}

// --

fn spawn(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut al: ResMut<AmbientLight>

) {
    al.brightness = 500.;
    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.))),
        Transform::from_xyz(1., 0., 1.),
        MeshMaterial3d(materials.add(Color::hsl(100., 1., 0.5)))
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        Transform::from_xyz(2., 0., 1.),
        MeshMaterial3d(materials.add(Color::hsl(50., 1., 0.5)))
    ));

}

// ---