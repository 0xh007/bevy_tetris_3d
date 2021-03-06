use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;
use crate::TetronimoBlock;
use crate::TetronimoState;

pub fn fall_system(
    time: Res<Time>,
    mut tetris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform, &mut Children)>,
    block_query: Query<(&mut TetronimoBlock, &Transform)>,
) {
    for (mut tetronimo, mut tetronimo_transform, mut children) in &mut tetronimo_query.iter() {
        let direction = -1.0;
        let mut collision_detected = false;
        let mut tetronimo_translation = &mut tetronimo_transform;

        for &child in &mut children.iter() {
            let mut block = block_query.get_mut::<TetronimoBlock>(child).unwrap();
            let block_relative_transform = block_query.get_mut::<Transform>(child).unwrap();
            let mut block_relative_translation = &mut block_relative_transform;

            let x = tetronimo_translation.translation.x() + block_relative_translation.translation.x();
            let y = tetronimo_translation.translation.y() + block_relative_translation.translation.y();
            let z = tetronimo_translation.translation.z() + block_relative_translation.translation.z();
            let block_translation = Vec3::new(x, y, z);

            let cell = tetris_grid.update_position(block_translation, block.last_grid_pos, block.state);

            if cell != block.current_grid_pos {
                block.last_grid_pos = block.current_grid_pos;
                block.current_grid_pos = cell;
            }
            
            match tetronimo.state {
                TetronimoState::Moving => {
                    if TetrisGrid::is_cell_below_occupied(&tetris_grid, cell.0, cell.1) {
                        block.state = TetronimoState::Stopped;
                        collision_detected = true;
                    }
                },
                TetronimoState::Stopped => {
                    block.state = TetronimoState::Stopped;
                    collision_detected = true
                },
            }
        }

        if !collision_detected {
            *tetronimo_translation.y_mut() += time.delta_seconds * direction * tetronimo.speed;
        } else {
            tetronimo.state = TetronimoState::Stopped;
        }
    }
}

pub fn grid_debug_system(
    mut testris_grid: ResMut<TetrisGrid>,
) {
    TetrisGrid::print_grid(&testris_grid);
}
