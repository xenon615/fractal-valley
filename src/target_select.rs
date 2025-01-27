use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use avian3d::prelude::*;

use crate::camera::{
    Cam, 
    CamFollowParams
};
use crate::player::Player;
use crate::shared::CELL_HEIGHT;

pub struct TargetSelectPlugin;
impl Plugin for TargetSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_click.run_if(on_event::<MouseButtonInput>));
     }
}

// ---

fn mouse_click(
    q_camera: Single<(&Camera, &GlobalTransform), With<Cam>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    raycast_q: SpatialQuery,
    p_q: Single<&mut Transform, With<Player>>,
    m_q: Query<&Transform, Without<Player>>,
    mut cp: ResMut<CamFollowParams>
) {
    let (camera, camera_gtransform) = q_camera.into_inner();

    if buttons.just_pressed(MouseButton::Left) && keys.pressed(KeyCode::ShiftLeft){
        let window = q_window.into_inner();
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_gtransform, cursor_position) else {
            return;
        };
    

        if let Some(hit) = raycast_q.cast_ray(
            ray.origin, 
            ray.direction.into(),
            f32::MAX,
            true, 
            &SpatialQueryFilter::default()
        ) {
            if let Ok(m_t)  = m_q.get(hit.entity) {
                let mut t = p_q.into_inner();
                t.translation = m_t.translation.with_y(m_t.translation.y + CELL_HEIGHT * m_t.scale.y * 0.5 + 1.);
                cp.tranlation_bias = cp.tranlation_bias.normalize() * 8.;
            }
        }
    }

}
