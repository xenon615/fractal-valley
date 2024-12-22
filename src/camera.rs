
use bevy::input::mouse::MouseWheel;
use bevy::{
    input::common_conditions::input_pressed,
    input::mouse::MouseMotion, 
    prelude::*
};
use avian3d::schedule::PhysicsSet;
use bevy::core_pipeline::Skybox;
use crate::shared::{cell2xz, Focus, PLAYER_START_CELL};
use crate::GameState;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        // .add_systems(Update, follow)
        .add_systems(PostUpdate, 
            follow
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate)
        )
        .add_systems(Update, mouse_drag
            .run_if(input_pressed(MouseButton::Left))
            .run_if(in_state(GameState::Game))
            .run_if(on_event::<MouseMotion>)
        )
        .add_systems(Update, distancing.run_if(on_event::<MouseWheel>))    
        .add_observer(cam_reset)
        ; 
    }
} 

// ---

#[derive(Component)]
pub struct Cam;

#[derive(Resource)]
pub struct CamFollowParams {
    pub tranlation_bias: Vec3,
    pub look_bias: (f32, f32, f32),
    pub translation_speed: f32,
    pub rotation_speed: f32
}

#[derive(Event)]
pub struct CamReset;

// ---

fn setup (
    mut cmd : Commands,
    assets: ResMut<AssetServer>
) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(cell2xz(PLAYER_START_CELL)),
        Cam,
        Name::new("Camera"),
        Camera {
            hdr: true,
            ..default()
        },
        Skybox {
            image: assets.load("skyboxes/interstellar-blue.ktx2"),
            brightness: 500.,
            ..default()
        },
    ));
    cmd.insert_resource(
        CamFollowParams{
            tranlation_bias: Vec3::new(0., 2., 8.),
            look_bias: (0., 1.5, 0.),
            translation_speed: 3.,
            rotation_speed: 8.
        }
    );

}

// ---

#[allow(dead_code)]
fn follow (
    focus_q: Single<&Transform , With<Focus>>,
    cam_q: Single<&mut Transform, (With<Cam>, Without<Focus>)>,
    cam_param: Res<CamFollowParams>,
    time: Res<Time>,
) {

    let focus_t = focus_q.into_inner(); 
    let mut cam_t = cam_q.into_inner();

    // let desired = focus_t.translation + focus_t.right() * cam_param.tranlation_bias.0 + focus_t.up() * cam_param.tranlation_bias.1 + focus_t.forward() * cam_param.tranlation_bias.2;
    let desired = focus_t.translation +  focus_t.rotation.mul_vec3(cam_param.tranlation_bias);
    cam_t.translation = cam_t.translation.lerp(desired, time.delta_secs() * cam_param.translation_speed);
    let look_at = focus_t.translation + focus_t.right() * cam_param.look_bias.0 + focus_t.up() * cam_param.look_bias.1 + focus_t.forward() * cam_param.look_bias.2;
    cam_t.rotation = cam_t.rotation.slerp(cam_t.looking_at(look_at, Vec3::Y).rotation, time.delta_secs() * cam_param.rotation_speed);
}

// ---

#[allow(dead_code)]
fn mouse_drag (
    mut er: EventReader<MouseMotion>,
    mut cam_param: ResMut<CamFollowParams>,
    time: Res<Time>,
    mut angle_x: Local<f32>,
    mut angle_y: Local<f32>,
) {

    for e in er.read() {

        if e.delta.x.abs() > f32::EPSILON {
            let l = cam_param.tranlation_bias.with_y(0.).length();
            *angle_x -= time.delta_secs() * e.delta.x * 0.1;
            cam_param.tranlation_bias.x = l * angle_x.sin();
            cam_param.tranlation_bias.z = l * angle_x.cos();
        }

        if e.delta.y.abs() > f32::EPSILON {
            let l = cam_param.tranlation_bias.with_x(0.).length();
            *angle_y += time.delta_secs() * e.delta.y * 0.1;
            cam_param.tranlation_bias.y = l * angle_y.sin();
            cam_param.tranlation_bias.z = l * angle_y.cos();
        }
    }
}

// ---

fn distancing (
    mut er: EventReader<MouseWheel>,
    mut cp: ResMut<CamFollowParams>
) {
    for e in er.read() {
        let MouseWheel{y, ..} = *e;
        cp.tranlation_bias *= if y > 0. {0.9}  else {1.1};
    }
}

// ---

fn cam_reset(
    _tr: Trigger<CamReset>,
    mut cp: ResMut<CamFollowParams>
) {
    cp.tranlation_bias.x = 0.;
    cp.tranlation_bias.z = cp.tranlation_bias.z.abs();
    // cp.tranlation_bias.y = 2.;
}