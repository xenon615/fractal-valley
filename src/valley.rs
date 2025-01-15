use avian3d::prelude::{Collider, RigidBody, CollisionLayers, LayerMask};
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*
};

use crate::{
    fractal::FractallCollors, 
    player::{AdjustY, Player}, 
    shared::{cell2xz, get_colorset, TilesCenter, CELL_HEIGHT, CELL_SIZE, PLAYER_START_CELL, TILES_COUNT, CoLayer}
};


pub struct ValleyPlugin;
impl Plugin for ValleyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, repaint.run_if(resource_changed::<FractallCollors>))
        // .add_systems(Update, show_gizmos)
        ;
    }
}

// ---

#[derive(Component, Debug)]
pub struct Tile(usize, usize);

#[derive(Resource)]
pub struct MaterialSet(pub Vec<Handle<StandardMaterial>>);

// ---

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {

    let color_vec:Vec<Handle<StandardMaterial>> = get_colorset()
    .into_iter()
    .map(|c| {
        materials.add(
            StandardMaterial {
                base_color: c,
                alpha_mode: AlphaMode::Mask(0.1),
                // alpha_mode: AlphaMode::Multiply,
                // alpha_mode: AlphaMode::Add,
                
                emissive: LinearRgba::from(c),
                ..default()
            })
    })
    .collect()
    ;

    cmd.insert_resource(MaterialSet(color_vec));

    // ---

    let tile_mesh = meshes.add(Cuboid::from_size(Vec3::new(CELL_SIZE, CELL_HEIGHT, CELL_SIZE)));
    // let tile_mesh = meshes.add(Cylinder::new(CELL_SIZE * 0.5, CELL_HEIGHT));
    let center = cell2xz(PLAYER_START_CELL);
    let start = - ((TILES_COUNT - 1 ) as f32 *  CELL_SIZE * 0.5)   ;
    let mut pos_x = start;
    let mut pos_z = pos_x; 

    for i in 0 .. TILES_COUNT   {
        for j in 0 .. TILES_COUNT {
            cmd.spawn((
                Mesh3d(tile_mesh.clone()),
                MeshMaterial3d(Handle::<StandardMaterial>::default()),
                Transform::from_xyz(pos_x + center.x, 0. + center.y + CELL_HEIGHT / 2., pos_z + center.z),
                Tile(i, j),
                NotShadowCaster,
                NotShadowReceiver,
                Collider::cuboid(CELL_SIZE, CELL_HEIGHT, CELL_SIZE),
                // Collider::cylinder(CELL_SIZE * 0.5, CELL_HEIGHT),
                RigidBody::Static,
                CollisionLayers::new(CoLayer::Tile, [LayerMask::ALL]),
                Name::new("Tile")
            ));
            pos_z += CELL_SIZE;    
        }
        pos_x += CELL_SIZE;
        pos_z = start;
    }
    cmd.insert_resource(TilesCenter(PLAYER_START_CELL.0, PLAYER_START_CELL.1));

    cmd.spawn((
        DirectionalLight {
            color: Color::hsl(50., 1., 0.5),
            illuminance: 50000.,
            shadows_enabled: false,
            ..default()
        },
        Transform::IDENTITY.looking_to(Vec3::ZERO, Vec3::Y)
    ));
}

// ---

fn repaint (
    colors: Res<FractallCollors>,
    mut tiles_q: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile, &mut Transform), Without<Player>>,
    colorset: Res<MaterialSet>,
    tc: Res<TilesCenter>,
    mut cmd: Commands
) {

    let middle = TILES_COUNT / 2;

    let (_, _, t,) = tiles_q.iter().find(|(_, tp, _)| {
        tp.0 == middle && tp.1 == middle
    }).unwrap();

    let step = cell2xz((tc.0, tc.1)) - t.translation.with_y(0.);
    // println!("step : {:?}", step);
    for i in 0..TILES_COUNT{
        for j in 0..TILES_COUNT {
            if let Some((mut t_mat, _ , mut t_trans)) = tiles_q.iter_mut().find(|(_, tp, _)| {
                tp.0 == i && tp.1 == j
            }) {
                let color_index = colors.0[i][j] as usize;
                t_mat.0 =  colorset.0[color_index].clone();
                t_trans.translation += step;
                // t_trans.scale.y = 0.5 * (color_index + 1) as f32;
                // t_trans.translation.y = t_trans.scale.y * CELL_HEIGHT / 2.;
                t_trans.translation.y = color_index as f32 * 0.5;
                
            }
        }
    }
    let m_y = colors.0[middle][middle];

    cmd.trigger(AdjustY(m_y as f32 * 0.5 + CELL_HEIGHT / 2. + 2.));

}

// ---

#[allow(dead_code)]
fn show_gizmos(
    pp: Single<&Transform, With<Player>>,
    mut gizmos: Gizmos
) {
    let t = pp.into_inner();
    // gizmos.ray(start + Vec3::Y, Vec3::X * 100., Color::hsl(0., 1.0, 0.5));
    // gizmos.ray(start + Vec3::Y, Vec3::Z * 100., Color::hsl(300., 1.0, 0.5));
    gizmos.ray(t.translation, t.forward() *  100., Color::srgb(1., 0., 0.));
}

// ---

