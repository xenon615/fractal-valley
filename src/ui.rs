use bevy::prelude::*;

use crate::player::PlayerCell;
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, display.run_if(resource_changed::<PlayerCell>))
        ;
    }
}

// --

#[derive(Component)]
pub struct IndCell;

// --

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        IndCell,
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(10.),
            bottom: Val::Percent(10.),
            ..default()
        }
    ));
}

// ---

fn display(
    cell: Res<PlayerCell>,
    ind_q: Single<&mut Text, With<IndCell>>,
) {
    let mut ind = ind_q.into_inner();
    ind.0 = format!("{} / {}", cell.0, cell.1);
}

// ---

