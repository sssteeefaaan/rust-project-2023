use bevy::{prelude::*, sprite::Anchor};

use crate::maze::Maze;

use super::{WinSize, GameTextures, WALL_SCALE, DOOR_SCALE, KEY_SCALE, DOOR_COLOR, FIELD_COLOR, WALL_COLOR, SOLUTION_FIELD_COLOR};

pub struct MazeVisualPlugin{
    pub maze_instance: Option<Maze>
}

#[derive(Component)]
pub struct MazeVisual;

#[derive(Component)]
pub struct Field;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct Solution;

#[derive(Component)]
pub enum CollidableType{
    Wall,
    Door,
    Key,
    Exit,
    Field
}

#[derive(Component)]
pub struct Dimensions{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Component)]
pub struct CollidableDetails{
    pub id: usize,
    pub dim: Dimensions,
    pub c_type: CollidableType,
    pub position: (usize, usize)
}

impl Dimensions{
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }
}

#[derive(Resource)]
pub struct MazeVisualState{
    pub maze: Maze,
    pub entities: Vec<Entity>,
    pub showing_solution: bool,
    pub size: Vec2,
    pub field_dimensions: Vec2
}

impl Default for MazeVisualState {
	fn default() -> Self {
        Self {
            maze: match Maze::parse_from_file(&"primer.bin".to_string()){
                Ok(v) => v,
                Err(_)=> Maze::default()
            },
            entities: Vec::<Entity>::new(),
            showing_solution: false,
            size: Vec2::default(),
            field_dimensions: Vec2::default()
        }
	}
}

impl MazeVisualState{
    fn from_maze(maze: Option<Maze>) -> Self{
        if maze.is_some(){
            Self{
                maze: maze.unwrap(),
                ..default()
            }
        }else{
            Self::default()
        }
    }
}

impl Plugin for MazeVisualPlugin{
    fn build(&self, app:&mut App){
        app.insert_resource(MazeVisualState::from_maze(self.maze_instance.clone()))
        .add_startup_system_to_stage(StartupStage::PostStartup, labyrinth_spawn_system.label("labyrinth-spawn"))
        //.add_startup_system_to_stage(StartupStage::PostStartup, solution_system.after("labyrinth-spawn"))
        .add_system(keyboard_event_system);
    }
}

fn labyrinth_spawn_system(
	mut commands: Commands,
    mut maze_visual_state: ResMut<MazeVisualState>,
    game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
    let frame_size = win_size.frame_size;

    maze_visual_state.size = Vec2::new(win_size.w - 2. * frame_size, win_size.h - 2. * frame_size);
    maze_visual_state.field_dimensions = Vec2::new(maze_visual_state.size.x / maze_visual_state.maze.dimensions.1 as f32, maze_visual_state.size.y / maze_visual_state.maze.dimensions.0 as f32);

    let (start_w, start_h) = (maze_visual_state.size.x, maze_visual_state.size.y);
    let (w, h) = (maze_visual_state.field_dimensions.x, maze_visual_state.field_dimensions.y);

    let key_size = (w * KEY_SCALE, h * KEY_SCALE);

    let wall_size = w.min(h) * WALL_SCALE;
    let wall_width = w;
    let wall_height = h;

    let door_size = w.min(h) * DOOR_SCALE;
    let door_width = w;
    let door_height = h;

    let door_color = Color::hex(DOOR_COLOR).unwrap();
    let wall_color = Color::hex(WALL_COLOR).unwrap();

    let horizontal_wall = Sprite{
        color: wall_color.clone(),
        custom_size: Some(Vec2 { x: wall_width, y: wall_size }),
        anchor: bevy::sprite::Anchor::TopLeft,
        ..Default::default()
    };

    let vertical_wall = Sprite{
        color: wall_color.clone(),
        custom_size: Some(Vec2{ x: wall_size, y: wall_height }),
        anchor: bevy::sprite::Anchor::TopLeft,
        ..Default::default()
    };

    let horizontal_door = Sprite{
        color: door_color.clone(),
        custom_size: Some(Vec2{ x : door_width, y: door_size }),
        anchor: bevy::sprite::Anchor::TopLeft,
        ..Default::default()
    };

    let vertical_door = Sprite{
        color: door_color.clone(),
        custom_size: Some(Vec2{ x : door_size, y: door_height }),
        anchor: bevy::sprite::Anchor::TopLeft,
        ..Default::default()
    };

    let mut children = Vec::<Entity>::new();

    for (y, row) in maze_visual_state.maze.fields.iter().enumerate(){
        for(x, field) in row.iter().enumerate(){
            if field.walls[0]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(wall_size, wall_height, 0.),
                    c_type: CollidableType::Wall,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                        sprite: vertical_wall.clone(),
                        transform: Transform::
                            from_translation(Vec3 {
                                x: x as f32 * w - start_w / 2.,
                                y: start_h / 2. - y as f32 * h,
                                z: 6.
                        }),
                        ..Default::default()
                    }).insert(Collidable)
                    .insert(cd)
                    .id()
                );
            }
            if field.walls[1]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(wall_size, wall_height, 0.),
                    c_type: CollidableType::Wall,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: vertical_wall.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: (x + 1) as f32 * w - start_w / 2. - wall_size,
                                    y: start_h / 2. - y as f32 * h,
                                    z: 6.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
            if field.walls[2]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(wall_width, wall_size, 0.),
                    c_type: CollidableType::Wall,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: horizontal_wall.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
                                    y: start_h / 2. - y as f32 * h,
                                    z: 6.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
            if field.walls[3]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(wall_width, wall_size, 0.),
                    c_type: CollidableType::Wall,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: horizontal_wall.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
                                    y: start_h / 2. + wall_size - (y + 1) as f32 * h ,
                                    z: 6.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
            if field.doors[0]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(door_size, door_height, 0.),
                    c_type: CollidableType::Door,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: vertical_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2. - door_size / 2.,
                                    y: start_h / 2. - y as f32 * h,
                                    z: 5.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
            if field.doors[1]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(door_size, door_height, 0.),
                    c_type: CollidableType::Door,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: vertical_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: (x + 1) as f32 * w - start_w / 2. - door_size / 2.,
                                    y: start_h / 2. - y as f32 * h,
                                    z: 5.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
            if field.doors[2]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(door_width, door_size, 0.),
                    c_type: CollidableType::Door,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                    SpriteBundle{
                            sprite: horizontal_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
                                    y: start_h / 2. - y as f32 * h + door_size / 2.,
                                    z: 5.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
            if field.doors[3]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(door_width, door_size, 0.),
                    c_type: CollidableType::Door,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: horizontal_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
                                    y: start_h / 2. - (y + 1) as f32 * h + door_size / 2.,
                                    z: 5.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }

            if field.key{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(key_size.0, key_size.1, 0.),
                    c_type: CollidableType::Key,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(
                        SpriteBundle {
                            texture: game_textures.key.clone(),
                            sprite: Sprite{
                                anchor: Anchor::Center,
                                custom_size: Some(Vec2::new(key_size.0, key_size.1)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3 {
                                x: x as f32 * w - start_w / 2. + w / 2.,
                                y: start_h / 2. - y as f32 * h - h / 2.,
                                z: 5.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }

            if field.exit{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim: Dimensions::new(key_size.0, key_size.1, 0.),
                    c_type: CollidableType::Exit,
                    position:(y, x)
                };
                children.push(
                    commands.spawn(SpriteBundle {
                            texture: game_textures.exit.clone(),
                            sprite: Sprite{
                                custom_size: Some(Vec2::new(key_size.0, key_size.1)),
                                anchor: Anchor::Center,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3 {
                                x: x as f32 * w - start_w / 2. + w / 2.,
                                y: start_h / 2. - y as f32 * h - h / 2.,
                                z: 3.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }

            children.push(commands.spawn(SpriteBundle {
                sprite: Sprite{
                    color: Color::hex(FIELD_COLOR).unwrap(),
                    custom_size: Some(Vec2::new(w, h)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3 {
                    x: x as f32 * w - start_w / 2. + w/2.,
                    y: start_h / 2. - y as f32 * h - h/2.,
                    z: 1.
                }),
                ..Default::default()
            })
            .insert(Collidable)
                .insert(CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(w * 0.1, h * 0.1, 0.),
                    c_type: CollidableType::Field,
                    position:(y, x)
                })
                .id()
        );
        }
    }
    maze_visual_state.entities = children;

    commands.spawn_empty()
    .insert(TextBundle{
        text: Text::from_section(
            "   Press 'S' for the solution :D",
            TextStyle {
                font_size: 30.,
                color: Color::rgb(0.,0.,0.),
                font: game_textures.font.clone()
            }
        ),
        z_index: ZIndex::Global(30),
        ..default()
    });
}

fn solution_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut maze_visual_state: ResMut<MazeVisualState>) -> Vec<Entity>
    {
    let (w, h) = (maze_visual_state.field_dimensions.x, maze_visual_state.field_dimensions.y);
    let (start_w, start_h) = (maze_visual_state.size.x, maze_visual_state.size.y);
    let sol_sprite = Sprite{
        color: Color::hex(SOLUTION_FIELD_COLOR).unwrap(),
        custom_size: Some(Vec2::new(w, h)),
        anchor: Anchor::Center,
        ..default()
    };
    let state = maze_visual_state.maze.get_state_mut().clone();
    let solution = maze_visual_state.maze.search_for_shortest_path_parallel(state);
    let mut spawned = Vec::new();
    if solution.is_some(){
        for step in solution.unwrap(){
            spawned.push(commands.spawn(SpriteBundle{
                transform: Transform::from_translation(Vec3::new(
                    step.1 as f32 * w - (start_w - w) / 2.,
                    (start_h - h) / 2. - step.0 as f32 * h,
                    2.
                )),
                sprite: sol_sprite.clone(),
                ..default()
            })
            .insert(Visibility{
                is_visible: maze_visual_state.showing_solution
            })
            .insert(Solution).id());
        }
    }else{
        spawned.push(commands.spawn_empty()
        .insert(TextBundle{
            text: Text::from_section(
                "   No solution found for this maze...",
                TextStyle {
                    font_size: 30.,
                    color: Color::rgb(0.,0.,0.),
                    font: game_textures.font.clone()
                }
            ),
            z_index: ZIndex::Global(30),
            ..default()
        }).insert(Visibility{
            is_visible: maze_visual_state.showing_solution
        })
        .insert(Solution).id());
    }

    return spawned;
}

fn keyboard_event_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    mut maze_visual_state: ResMut<MazeVisualState>,
    query: Query<Entity, With<Solution>>
){
    if kb.just_pressed(KeyCode::S){
        maze_visual_state.showing_solution = !maze_visual_state.showing_solution;
        if maze_visual_state.showing_solution{
            solution_system(commands, game_textures, maze_visual_state);
        }else{
            for e in query.iter(){
                commands.entity(e).despawn();
            }
        }
    }
}