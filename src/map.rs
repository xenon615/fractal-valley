use bevy::{
    asset::RenderAssetUsages, input::keyboard::KeyboardInput, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}, ui::RelativeCursorPosition
};

use crate::{
    camera::Cam, fractal::{calc_color, FractallBounds}, player::{Player, PlayerCell}, shared::{cell2xz, get_colorset, TILES_COUNT, VALLEY_SIZE}, GameState
};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, toggle_map.run_if(on_event::<KeyboardInput>))
        .add_systems(OnEnter(GameState::Map), change_vis)
        .add_systems(OnExit(GameState::Map), change_vis)
        ;
    }
}

// --

#[derive(Component)]
pub struct ValleyMap;

#[derive(Component)]
pub struct ValleyFrame;

#[derive(Resource, Debug)]
pub struct MapDim(u32, u32);


// --

fn startup(
    mut cmd: Commands,
    mut images: ResMut<Assets<Image>>,
    window_q: Single<&Window>,
    bounds: Res<FractallBounds>,
) {

    let window = window_q.into_inner();
    let w_height = window.height() as u32 - 20;
    let w_width =  (((bounds.x.1 - bounds.x.0) / (bounds.y.1 - bounds.y.0)) * w_height as f64) as u32;
    cmd.insert_resource(MapDim(w_width, w_height));
    let mut image = Image::new_fill(
        Extent3d {
            width: w_width,
            height: w_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &Srgba::WHITE.to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );

    let step_x = (bounds.x.1 - bounds.x.0) / w_width as f64;
    let step_y =  (bounds.y.1 - bounds.y.0) / w_height as f64;
    let x0 = bounds.x.0 as f64;
    let y0 = bounds.y.0 as f64;
    let mut x = x0;
    let mut y = y0;

    // let light_step = 1. / COLORS_COUNT as f32;
    let colorset = get_colorset();
    for i in 0 .. w_width {
        for j in 0 .. w_height {
            image.set_color_at(i, j, colorset[calc_color(x, y)].with_alpha(1.)).expect("Error");
            y += step_y; 
        }
        x += step_x;
        y = y0;
    } 

    cmd.spawn((
        Node {
            width: Val::Px(w_width as f32),
            height: Val::Px(w_height as f32),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        BorderColor(Color::srgb(1., 0., 0.)),
        ImageNode {
            image: images.add(image),
            ..default()
        },
        ValleyMap,
        Visibility::Hidden,
        RelativeCursorPosition::default()
    ))
    .observe(on_click)
    .with_children(|parent| {
        let size = w_width as f32 / TILES_COUNT as f32;
        parent.spawn(
            (
                ValleyFrame,
                Node {
                    border: UiRect::all(Val::Px(1.)),
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    width: Val::Px(size),
                    height: Val::Px(size),
                
                    ..default()
                },
                ZIndex(10),
                BorderColor(Color::WHITE)
            )
        );
    })
    ;

}

// ---

fn toggle_map(
    keys: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>
) {
    if keys.just_pressed(KeyCode::KeyM) {

        if *state == GameState::Map {
            next.set(GameState::Game);
        } else {
            next.set(GameState::Map);
        }
    }
}

// ---

fn on_click(
    _click: Trigger<Pointer<Click>>,
    map_q: Single<&RelativeCursorPosition, With<ValleyMap>>,
    player_q: Single<&mut Transform, With<Player>>,
    mut next: ResMut<NextState<GameState>>,
    cam_q: Single<&mut Transform, (With<Cam>, Without<Player>)>
) {
    let rcp = map_q.into_inner();
    if let Some(v) = rcp.normalized {
        let cell = (
            (VALLEY_SIZE as f32 * v.x).round() as usize, 
            (VALLEY_SIZE as f32 * v.y).round() as usize
        );
        let mut pt = player_q.into_inner();
        let pos = cell2xz(cell);
        pt.translation.x = pos.x;
        pt.translation.z = pos.z;
        let mut cam_t = cam_q.into_inner();
        cam_t.translation.x = pos.x;
        cam_t.translation.z = pos.z;
        next.set(GameState::Game)    
    }

}

// ---

fn change_vis(
    vis_q: Single<&mut Visibility, With<ValleyMap>>,
    player_cell: Res<PlayerCell>,
    map_dim: Res<MapDim>,
    frame_q: Single<&mut Node, With<ValleyFrame>>
) {
    let mut vis = vis_q.into_inner();
    if *vis == Visibility::Visible {
        *vis = Visibility::Hidden;
    } else {
        *vis = Visibility::Visible;
        let x = player_cell.0 as f32 * (map_dim.0 as f32/ VALLEY_SIZE as f32);
        let y = player_cell.1 as f32 * (map_dim.1 as f32/ VALLEY_SIZE as f32);
        let frame_size = map_dim.0 as f32 / TILES_COUNT as f32;
        let mut node = frame_q.into_inner();
        node.left = Val::Px(x - 0.5 * frame_size);
        node.top = Val::Px(y - 0.5 * frame_size);
    }
}
