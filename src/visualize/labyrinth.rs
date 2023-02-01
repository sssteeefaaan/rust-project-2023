use bevy::{prelude::*, sprite::Anchor};

use crate::maze::Maze;

use super::{WinSize, GameTextures, WALL_SCALE, KEY_SCALE};

pub struct LabyrinthPlugin{
    pub maze_instance: Option<Maze>
}

#[derive(Component)]
pub struct Labyrinth;

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
    Exit
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
    pub c_type: CollidableType
}

impl Dimensions{
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }
}

#[derive(Resource)]
pub struct FieldState{
    pub index: (u8,u8),
    pub key: bool,
    pub walls: (bool, bool, bool, bool)
}

#[derive(Resource)]
pub struct LabyrinthState{
    pub maze: Maze,
    pub entities: Vec<Entity>,
    pub showing_solution: bool
}

impl Default for LabyrinthState {
	fn default() -> Self {
        Self {
            maze: match Maze::parse_from_file(&"primer.bin".to_string()){
                Ok(v) => v,
                Err(_)=> Maze::default()
            },
            entities: Vec::<Entity>::new(),
            showing_solution: false
        }
	}
}

impl LabyrinthState{
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

impl Plugin for LabyrinthPlugin{
    fn build(&self, app:&mut App){
        app.insert_resource(LabyrinthState::from_maze(self.maze_instance.clone()))
        .add_startup_system_to_stage(StartupStage::PostStartup, labyrinth_spawn_system.label("labyrinth-spawn"))
        .add_startup_system_to_stage(StartupStage::PostStartup, solution_system.after("labyrinth-spawn"))
        .add_system(keyboard_event_system);
    }
}

fn labyrinth_spawn_system(
	mut commands: Commands,
    mut ls: ResMut<LabyrinthState>,
    game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
    let frame_size = win_size.frame_size;

    let start_w = win_size.w - 2. * frame_size;
    let start_h = win_size.h - 2. * frame_size;

    let w = start_w / ls.maze.dimensions.1 as f32;
    let h = start_h / ls.maze.dimensions.0 as f32;

    let key_size = (w * KEY_SCALE, h * KEY_SCALE);

    let wall_size = w.min(h) * WALL_SCALE;
    let wall_width = w;
    let wall_height = h;

    let door_size = wall_size;
    let door_width = w;
    let door_height = h;

    let door_color = Color::rgb(0.9, 1., 0.);
    let wall_color = Color::rgb(0., 0.8, 1.);

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

    for (y, row) in ls.maze.fields.iter().enumerate(){
        for(x, field) in row.iter().enumerate(){
            if field.walls[0]{
                let cd = CollidableDetails{
                    id: children.len(),
                    dim:Dimensions::new(wall_size, wall_height, 0.),
                    c_type: CollidableType::Wall
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
                    c_type: CollidableType::Wall
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
                    c_type: CollidableType::Wall
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
                    c_type: CollidableType::Wall
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
                    c_type: CollidableType::Door
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: vertical_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
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
                    c_type: CollidableType::Door
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: vertical_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: (x + 1) as f32 * w - start_w / 2. - door_size,
                                    y: start_h / 2. + y as f32 * h,
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
                    c_type: CollidableType::Door
                };
                children.push(
                    commands.spawn(
                    SpriteBundle{
                            sprite: horizontal_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
                                    y: start_h / 2. - y as f32 * h,
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
                    c_type: CollidableType::Door
                };
                children.push(
                    commands.spawn(
                        SpriteBundle{
                            sprite: horizontal_door.clone(),
                            transform: Transform::
                                from_translation(Vec3 {
                                    x: x as f32 * w - start_w / 2.,
                                    y: start_h / 2. - (y + 1) as f32 * h + door_size,
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
                    c_type: CollidableType::Key
                };
                children.push(
                    commands.spawn(
                        SpriteBundle {
                            texture: game_textures.key.clone(),
                            sprite: Sprite{
                                anchor: Anchor::TopLeft,
                                custom_size: Some(Vec2::new(key_size.0, key_size.1)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3 {
                                x: x as f32 * w - start_w / 2. + key_size.0 / 2.,
                                y: start_h / 2. - y as f32 * h - key_size.1 / 2.,
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
                    dim:Dimensions::new(w, h, 0.),
                    c_type: CollidableType::Exit
                };
                children.push(
                    commands.spawn(SpriteBundle {
                            sprite: Sprite{
                                color: Color::rgb(0.,0.9,0.),
                                custom_size: Some(Vec2::new(w, h)),
                                anchor: Anchor::TopLeft,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3 {
                                x: x as f32 * w - start_w / 2.,
                                y: start_h / 2. - y as f32 * h,
                                z: 3.
                            }),
                            ..Default::default()
                        }).insert(Collidable)
                        .insert(cd)
                        .id()
                    );
            }
        }
    }
    ls.entities = children;

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

    commands.spawn(
        SpriteBundle{
            sprite: Sprite {
                color: Color::rgb(0.8,0.4,0.9),
                custom_size: Some(Vec2::new(start_w, start_h)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3 { x: 0., y: 0., z: 1. }),
            ..Default::default()
        });
}

fn solution_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut labyrinth_state: ResMut<LabyrinthState>)
    {
    let (w, h) = ((win_size.w - 2. * win_size.frame_size)/labyrinth_state.maze.dimensions.1 as f32, (win_size.h - 2. * win_size.frame_size) / labyrinth_state.maze.dimensions.0 as f32);
    let (tile_w, tile_h) = (w, h);
    let sol_sprite = Sprite{
        color: Color::rgba(0.3, 0.2, 0.8, 0.5),
        custom_size: Some(Vec2::new(tile_w, tile_h)),
        anchor: Anchor::Center,
        ..default()
    };
    let solution = labyrinth_state.maze.get_shortest_path();
    for step in solution{
        commands.spawn(SpriteBundle{
            transform: Transform::from_translation(Vec3::new(
                step.1 as f32 * tile_w - (win_size.w - 2. * win_size.frame_size - w) / 2.,
                (win_size.h - 2. * win_size.frame_size - h) / 2. - step.0 as f32 * tile_h ,
                2.
            )),
            sprite: sol_sprite.clone(),
            ..default()
        })
        .insert(Visibility{
            is_visible: labyrinth_state.showing_solution
        })
        .insert(Solution);
    }
}

fn keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut labyrinth_state: ResMut<LabyrinthState>,
    mut solution: Query<&mut Visibility, With<Solution>>,
){
    if kb.just_pressed(KeyCode::S){
        labyrinth_state.showing_solution = !labyrinth_state.showing_solution;
        for mut v in solution.iter_mut(){
            v.is_visible = labyrinth_state.showing_solution;
        }
    }
}