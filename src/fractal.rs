use bevy::prelude::*;

use crate::{
    player::PlayerCell, 
    shared::{TilesCenter, INITIAL_BOUNDS, TILES_COUNT, VALLEY_SIZE, COLORS_COUNT}
};

pub struct FractalPlugin;
impl Plugin for FractalPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<FractallCollors>()
        .insert_resource(FractallBounds{
            x: INITIAL_BOUNDS.0,
            y: INITIAL_BOUNDS.1
        })
        .add_systems(Update,do_fractal.run_if(resource_changed::<PlayerCell>))
        ;
    }
}

// ---
#[derive(Resource, Debug)]
pub struct FractallBounds{
    pub x: (f64, f64),
    pub y: (f64, f64),
}

#[derive(Resource, Debug)]
pub struct FractallCollors(pub [[usize; TILES_COUNT]; TILES_COUNT]);

impl FromWorld for FractallCollors {
    fn from_world(_world: &mut World) -> Self {
        Self([[0; TILES_COUNT]; TILES_COUNT])
    }
}

//  ---

fn calc_center(player_cell: (usize, usize), center_cell: (usize, usize)) -> (usize, usize) {
    let diff_x = center_cell.0 as i32 - player_cell.0 as i32;
    let diff_z = center_cell.1 as i32 - player_cell.1 as i32;

    let half = TILES_COUNT as i32  / 2;

    if diff_x.abs() > half + 1||  diff_z.abs() > half + 1 {
        return player_cell;
    } 

    let new_x = if diff_x.abs() > half {
        center_cell.0 as i32 +  TILES_COUNT as i32 * -diff_x.signum()
    } else {
        center_cell.0 as i32
    };

    let new_z = if diff_z.abs() > half {
        center_cell.1 as i32 + TILES_COUNT as i32* -diff_z.signum()
    } else {
        center_cell.1 as i32
    };
    (new_x as usize, new_z as usize)
}

// ---

fn do_fractal(
    player_cell: Res<PlayerCell>,
    mut colors: ResMut<FractallCollors>,
    bounds: Res<FractallBounds>,
    mut center_cell: ResMut<TilesCenter>,
    mut not_first: Local<bool>
) {
    let PlayerCell(px, pz) = *player_cell;
    let TilesCenter(cx, cz) = *center_cell; 

    let cell = calc_center((px,pz), (cx, cz));

    if (cell == (cx, cz)) && *not_first {
        return;
    }
    if !*not_first {
        *not_first = true;
    }
    center_cell.0 = cell.0;
    center_cell.1 = cell.1;
    let half = TILES_COUNT / 2;

// ============================================================================================================================================================================================

    let start = (
        if half as usize> cell.0 {0} else {cell.0 - half as usize},
        if half as usize> cell.1 {0} else {cell.1 - half as usize},
    );

    let step_x = (bounds.x.1 - bounds.x.0) / VALLEY_SIZE as f64;
    let step_y =  (bounds.y.1 - bounds.y.0) / VALLEY_SIZE as f64;

    let x0 = bounds.x.0 + start.0 as f64 * step_x;
    let y0 = bounds.y.0 + start.1 as f64 * step_y;
    
    let mut x = x0;
    let mut y = y0;

    for i in 0 .. TILES_COUNT {
        for j in  0 .. TILES_COUNT {
            colors.0[i][j] = calc_color(x, y);            
            y += step_y; 
        }
        x += step_x;
        y = y0;
    }

}

// ---

pub fn calc_color(x : f64, y : f64) -> usize {
    let mut lx = x;
    let mut ly = y;
    let mut n = 0;
    let ly2 = ly * ly;
    let lx2 = lx * lx;

    // cardioid check
    let q = lx2 - 0.5 * lx  + 0.0625 + ly2;
    if ly2  >= 4.0 * q * (q + lx - 0.25) {
         return 0;
    }
    // bulb check
    if (lx + 1.0) * (lx + 1.0) + ly2 < 0.0625 {
         return 0;
    }

    while  (lx * lx + ly * ly  < 4.0) && (n < COLORS_COUNT) {
        let lxt = lx * lx - ly * ly + x;
        ly = 2. * lx * ly + y;
        lx = lxt;
        n += 1;
    }

    if n == COLORS_COUNT {0} else {n}
}
