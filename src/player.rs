use std::default;

use avian3d::{
    math::Vector, 
    prelude::*
};
use bevy::{
    // gizmos, 
    pbr:: {NotShadowCaster, NotShadowReceiver}, 
    prelude::*,
    input::{
        common_conditions::input_pressed,
        mouse::MouseMotion, 
     }
};
use crate::{
    animator::{AllAnimations, AnimationKey, CurrentAnimation}, camera::CamReset, shared::{cell2xz, xz2cell, Focus, CELL_HEIGHT, PLAYER_START_CELL}, valley::Tile, GameState
};


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PlayerCell>()
        .add_systems(Startup, startup)
        .add_systems(Update, change_cell)
        .add_systems(OnEnter(GameState::Game), enter_game)
        .add_systems(Update, (
            keyboard_input,
            switch_anim,
            dump_xz,
            check_grounded,
            grounded_anim.run_if(condition_changed(any_with_component::<Grounded>))
        ).chain())
         .add_systems(Update, mouse_input
            .run_if(input_pressed(MouseButton::Right))
            .run_if(in_state(GameState::Game))
            .run_if(on_event::<MouseMotion>)
        )
        .add_observer(movement)
        .add_observer(adjust_y)
        ;
    }
}

// ---

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerChild;

#[derive(Resource, Debug)]
pub struct PlayerCell(pub usize, pub usize);

impl FromWorld for PlayerCell {
    fn from_world(_world: &mut World) -> Self {
        Self (PLAYER_START_CELL.0, PLAYER_START_CELL.1)
    }
}

#[derive(Event)]
pub struct Movement {
    direction: f32,
    rotation: f32,
    jump: bool
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Running;

#[derive(Event)]
pub struct AdjustY;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Other,
    Player
}


// ---

fn startup(
    mut cmd: Commands,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset: ResMut<AssetServer>
) {
    all_animations.add(AnimationKey::Player, "models/player.glb", 4, &mut graphs, &asset);
    let collider = Collider::capsule(0.5, 1.);
    let mut caster_shape  = collider.clone();
    caster_shape.set_scale(Vector::ONE * 0.99, 10);
    cmd.spawn((
        SceneRoot(asset.load(GltfAssetLabel::Scene(0).from_asset("models/player.glb"))),
        Transform::from_translation(
            cell2xz(PLAYER_START_CELL)
            .with_y(10.)
        )        
        .looking_to(Vec3::X, Vec3::Y),
        Player,
        Focus,
        NotShadowCaster,
        NotShadowReceiver,
        AnimationKey::Player,
        AngularDamping(1.5),
        RigidBody::Dynamic,
        LockedAxes::new()
        .lock_rotation_x()
        .lock_rotation_z(),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Max),
        GravityScale(1.5),
        Name::new("Player")
     ))
     .with_child((
        PlayerChild,
        Transform::from_xyz(0., 1., 0.),
        ShapeCaster::new(caster_shape, Vec3::ZERO,  Quat::default(), Dir3::NEG_Y).with_max_distance(0.2).with_ignore_origin_penetration(true),
        collider,
        CollisionLayers::new(GameLayer::Player, [LayerMask::ALL]),
        Name::new("Player child")
     ))
     ;
}

// ---

fn enter_game(
    ca_q: Single<&mut CurrentAnimation, With<Player>>
) {
    ca_q.into_inner().0 = 0;
}

// ---

fn mouse_input (
    mut er: EventReader<MouseMotion>,
    time: Res<Time>,
    p_q: Single<&mut Transform, With<Player>>
) {
    let mut t = p_q.into_inner();

    for e in er.read() {
        if e.delta.x != 0. {
            let delta = time.delta_secs() * e.delta.x * - 0.5 ;
            t.rotate_y(delta);
        }
    }
}

// --

fn keyboard_input(
    mut cmd: Commands,
    keys: Res<ButtonInput<KeyCode>>
) {
    let forward_keys = [KeyCode::ArrowUp,  KeyCode::KeyW];
    let back_keys = [KeyCode::ArrowDown,  KeyCode::KeyS];
    let right_keys = [KeyCode::ArrowRight,  KeyCode::KeyD];
    let left_keys = [KeyCode::ArrowLeft,  KeyCode::KeyA];

    let forward = keys.any_pressed(forward_keys);
    let back  = keys.any_pressed(back_keys);

    let right = keys.any_pressed(right_keys);
    let left = keys.any_pressed(left_keys);

    let direction = forward as i8 - back as i8;
    let rotation = right as i8 - left as i8;

    let jump = keys.just_pressed(KeyCode::Space);
    if direction != 0 || rotation != 0 || jump {
        cmd.trigger(
            Movement{
                direction: direction as f32,
                rotation: rotation as f32,
                jump
            }
        );
    }
}

// ---

fn movement(
    trigger: Trigger<Movement>,
    p_q: Single<(&mut LinearVelocity, &mut AngularVelocity, &mut ExternalImpulse, &Transform, Option<&Grounded>), With<Player>>,
    time: Res<Time>
) {
    let Movement{direction, rotation, jump} = trigger.event();

    let (mut l, mut a, mut i, t, og) = p_q.into_inner();
    if og.is_none() {
        return;
    }

    if *direction != 0. {
        let accel_coef = if *direction > 0. {30.0} else {10.0};
        l.x += direction * t.forward().x * time.delta_secs() * accel_coef;
        l.z += direction * t.forward().z * time.delta_secs() * accel_coef;
    }

    if *rotation != 0. {
        a.y = rotation * time.delta_secs()  * -60.;
    }
    if *jump {
        i.set_impulse(Vec3::Y * 10.);
    }
}

// ---

fn change_cell(
    player_q: Single<&Transform, (Changed<Transform>, With<Player>)>,
    mut cell: ResMut<PlayerCell>,
    // mut cmd: Commands
) {
    let Transform {translation: trans, ..} = player_q.into_inner();
    let (cell_x, cell_z) = xz2cell(*trans);
    if cell_x != cell.0  || cell_z != cell.1 {
        cell.0 = cell_x;
        cell.1 = cell_z;
        // cmd.trigger(AdjustY);
    }
}

// ---

#[allow(dead_code)]
fn switch_anim (
    p_q: Single<(Entity, &mut CurrentAnimation), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cmd : Commands
) {
    let (e, mut ca) = p_q.into_inner();
    if keys.any_just_pressed([ KeyCode::ArrowUp,  KeyCode::KeyW ]) {
        ca.0 = 1;
        cmd.entity(e).insert(Running);
        cmd.trigger(CamReset);
    }

    if keys.any_just_pressed([ KeyCode::ArrowDown,  KeyCode::KeyS ]) {
        ca.0 = 3;
    }

    if keys.any_just_released([ KeyCode::ArrowUp,  KeyCode::KeyW, KeyCode::ArrowDown,  KeyCode::KeyS ]) {
        ca.0 = 0;
        cmd.entity(e).remove::<Running>();
    }

}

// ---

fn dump_xz(
    p_q: Single<(&mut LinearVelocity, Option<&Grounded>),With<Player>>,
) {
    let (mut lv, og) = p_q.into_inner();
    if og.is_some() {
        let dump_coef = 0.95;
        lv.x *= dump_coef;
        lv.z *= dump_coef;
    }
}

// ---

fn check_grounded (
    p_q: Single<(&ShapeHits, &Parent), With<PlayerChild>>,
    mut cmd : Commands
) {
    let (hits, p) = p_q.into_inner();
    let is_grounded = !hits.is_empty();
    if is_grounded {
        cmd.entity(p.get()).insert(Grounded);
    } else {
        cmd.entity(p.get()).remove::<Grounded>();
    }
}

// ---

fn grounded_anim(
    p_q: Single<(&mut CurrentAnimation, Option<&Grounded>, Option<&Running>), With<Player>>,
    c_q: Single<&mut Collider, With<PlayerChild>>
) {
    let (mut ca, og, or) = p_q.into_inner();
    ca.0 = if og.is_none() { 2 } else if or.is_some() { 1 } else { 0 };
    let mut c = c_q.into_inner();
    if og.is_none() {
        c.set_scale(Vec3::splat(0.5), 0);
    } else {
        c.set_scale(Vec3::splat(1.0), 0);
    }
}

// ---

fn adjust_y(
    _trg: Trigger<AdjustY>,
    ps: Res<PlayerCell>,
    p_q: Single<&mut Transform, With<Player>>,
    raycast: SpatialQuery,
    // m_q: Query<&Transform, (Without<Player>, With<Tile>)>
    m_q: Query<&Transform, Without<Player>>,
    qn: Query<&Name>
) {
    println!("here");
    if let Some(hit) = raycast.cast_ray(
        cell2xz((ps.0, ps.1)).with_y(1000.), 
        Dir3::NEG_Y,
        f32::MAX,
        true, 
        &SpatialQueryFilter::from_mask(GameLayer::Other)
    ) {
        println!("a_y");
        if let Ok(m_t)  = m_q.get(hit.entity) {
            if let Ok(name)  = qn.get(hit.entity) {
                println!("name {:}", name);
            }
            let mut t = p_q.into_inner();
            t.translation.y = m_t.translation.y + CELL_HEIGHT * m_t.scale.y * 0.5 + 1.;
            println!("hit {}",  t.translation.y);
        }
    }
}