use std::usize;

use bevy::prelude::*;

#[derive(Component)]
pub struct Focus;

#[derive(Resource, Debug)]
pub struct TilesCenter(pub usize, pub usize);

pub const COLORS_COUNT: usize = 128;

pub const VALLEY_SIZE: u32 = 8001;
pub const TILES_COUNT: usize = 41;
// pub const PLAYER_START_CELL:(usize, usize) = (3317, 3046);
pub const PLAYER_START_CELL:(usize, usize) = (200, 200);

pub const CELL_SIZE: f32 = 4.;
pub const CELL_HEIGHT: f32 = 0.5;
pub const INITIAL_BOUNDS: ((f64, f64), (f64, f64)) = ((-2., 0.6), (-1.30, 1.30));


// ---

pub fn cell2xz(cell: (usize, usize)) -> Vec3 {
    let x0 = VALLEY_SIZE as f32 * CELL_SIZE / -2. + CELL_SIZE / 2. ;
    let z0 = x0;
    Vec3::new(x0 + CELL_SIZE * cell.0 as f32, 0., z0 + CELL_SIZE * cell.1 as f32)
}

// ---

pub fn xz2cell(pos: Vec3) -> (usize, usize) {
    let half_valley = ((VALLEY_SIZE)  as f32  * 0.5).floor();
     ((half_valley + (pos.x / CELL_SIZE).round()) as usize, (half_valley + (pos.z / CELL_SIZE).round()) as usize)
}

// ---

pub fn get_colorset() -> Vec<Color> {
    let light_step = 1. / 16.;
    let start_color = (0., 1.0, 0.5, 0.6);
    let mut light = 0.;
    let huestep = (360. - start_color.0) / COLORS_COUNT as f32;
    (0..COLORS_COUNT).map(|i| {
        if i == 0 {
            Color::hsla(0., 0., 0., 1.0)    
        } else {
            let i_f = i as f32;
            let hue = start_color.0 + huestep * i_f;
            if light >= 16. {
                light = 0.;
            } else {
                light += light_step;
            }
            // Color::hsla(hue, start_color.1, start_color.2 + light_step * i_f, start_color.3)
            Color::hsla(hue, start_color.1, start_color.2, start_color.3)
        }
    }).collect()

}