use bevy::prelude::*;
use std::fmt;

// Sets up the game board and walls for the arena
pub fn arena_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Colors
    let background_color = Color::rgb(0.0, 0.001, 0.002);
    let wall_color = Color::rgb(0.08, 0.38, 0.43);
    let rotate_90_x = Quat::from_rotation_x(1.5708);
    let rotate_90_z = Quat::from_rotation_z(1.5708);

    // Background
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/background/export/background.gltf")
                .unwrap(),
            material: materials.add(background_color.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(rotate_90_x),
            ..Default::default()
        },
    );

    // Left Wall
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(wall_color.into()),
            transform: Transform::from_translation(Vec3::new(-5.2, 0.0, 3.0)),
            ..Default::default()
        },
    );

    // Right Wall
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(wall_color.into()),
            transform: Transform::from_translation(Vec3::new(5.2, 0.0, 3.0)),
            ..Default::default()
        },
    );

    // Bottom Wall
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/bottom_wall/export/bottom_wall.gltf")
                .unwrap(),
            material: materials.add(wall_color.into()),
            transform: Transform::from_translation(Vec3::new(0.215, -10.15, 3.0))
                .with_rotation(rotate_90_z),
            ..Default::default()
        },
    );

    println!("Arena setup complete");
}

enum TetronimoTest{
    Fill,
    Single,
}

#[derive(Copy, Clone)]
pub enum TetronimoState {
    Moving,
    Stopped,
}

pub enum TetronimoDirection {
    Left,
    Right,
    None,
}

impl fmt::Display for TetronimoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TetronimoState::Moving => write!(f, "Moving"),
            TetronimoState::Stopped => write!(f, "Stopped"),
        }
    }
}

pub struct TetronimoBlock {
    pub name: String,
    pub last_grid_pos: (i32, i32),
    pub current_grid_pos: (i32, i32),
    pub state: TetronimoState,
}

pub struct Tetronimo {
    pub speed: f32,
    pub lateral_speed: f32,
    pub name: String,
    pub state: TetronimoState,
    pub destination: Vec3,
    pub traveling_laterally: bool,
    pub movement_direction: TetronimoDirection,
}

// Test system to setup some tetronimos for debug purposes
pub fn tetronimo_test_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tetronimo_test_type = TetronimoTest::Single;

    let tetronimo_color = Color::rgb(0.47, 0.16, 0.06);

    match tetronimo_test_type {
        TetronimoTest::Fill => {
            let x_min = -4;
            let x_max = 6;
            let y_min = -9;
            let y_max = 11;

            for x in x_min..x_max {
                for y in y_min..y_max {
                    let offset = 0.5;
                    let x_pos = x as f32 - offset;
                    let y_pos = y as f32 - offset;

                    commands.spawn(
                        PbrComponents {
                            mesh: asset_server
                                .load("assets/tetronimo/export/tetronimo.gltf")
                                .unwrap(),
                            material: materials.add(tetronimo_color.into()),
                            transform: Transform::from_translation(Vec3::new(x_pos as f32, y_pos as f32, 3.5)),
                            ..Default::default()
                        },
                    );
                }
            }

            println!("Tetronimo Fill test setup complete");
        },

        TetronimoTest::Single => {
            // ---------------------
            // T-tetronimo 1
            commands.spawn((
                    Tetronimo {
                        lateral_speed: 3.0,
                        speed: 1.0,
                        name: String::from("T-Tetronimo"),
                        state: TetronimoState::Moving,
                        destination: Vec3::new(0.0, 0.0, 0.0),
                        traveling_laterally: false,
                        movement_direction: TetronimoDirection::None,
                    },
            ))
            .with(Transform::from_translation(Vec3::new(2.5, 6.5, 3.5)))
            .with(GlobalTransform::identity())
            //.with(Transform::identity())
            .with_children(|parent| {
                // A
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("A"),
                    state: TetronimoState::Moving,
                });

                // B
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("B"),
                    state: TetronimoState::Moving,
                });

                // C
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("C"),
                    state: TetronimoState::Moving,
                });

                // D
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("D"),
                    state: TetronimoState::Moving,
                });
            });
            // ---------------------


            // ---------------------
            // T-tetronimo 2
            commands.spawn((
                    Tetronimo {
                        lateral_speed: 3.0,
                        speed: 1.0,
                        name: String::from("T-Tetronimo_2"),
                        state: TetronimoState::Moving,
                        destination: Vec3::new(0.0, 0.0, 0.0),
                        traveling_laterally: false,
                        movement_direction: TetronimoDirection::None,
                    },
            ))
            .with(Transform::from_translation(Vec3::new(2.5, -6.5, 3.5)))
            .with(GlobalTransform::identity())
            //.with(Transform::identity())
            .with_children(|parent| {
                // A
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("A"),
                    state: TetronimoState::Moving,
                });

                // B
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("B"),
                    state: TetronimoState::Moving,
                });

                // C
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("C"),
                    state: TetronimoState::Moving,
                });

                // D
                parent.spawn(
                    PbrComponents {
                        mesh: asset_server
                            .load("assets/tetronimo/export/tetronimo.gltf")
                            .unwrap(),
                        material: materials.add(tetronimo_color.into()),
                        transform: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                        ..Default::default()
                    },
                )
                .with(TetronimoBlock {
                    last_grid_pos: (-1, -1),
                    current_grid_pos: (-1, -1),
                    name: String::from("D"),
                    state: TetronimoState::Moving,
                });
            });
            // ---------------------

            println!("Tetronimo Single test setup complete");
        },
    };

}
