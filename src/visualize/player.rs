use bevy::{prelude::*, sprite::{Anchor, collide_aabb::collide}};

use super::{WinSize, GameTextures, PLAYER_SCALE, labyrinth::{LabyrinthState, Collidable, CollidableType, Dimensions, CollidableDetails}, BASE_SPEED, PLAYER_ASSET_DIMENSIONS};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct Inventory;

#[derive(Resource)]
pub struct PlayerState {
	pub position: (usize, usize),
    pub spawned: bool,
    pub size: (f32, f32),
    pub key_number: usize
}

impl Default for PlayerState {
	fn default() -> Self {
		Self { position: (0, 0), spawned: false, size: (100., 50.), key_number: 0 }
	}
}

impl Plugin for PlayerPlugin{
    fn build(&self, app:&mut App){
        app.insert_resource(PlayerState::default())
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system.label("player-spawn"))
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_inventory_system.after("player-spawn"))
        .add_system(player_keyboard_event_system)
        .add_system(player_movement_system.label("movement"))
        .add_system(inventory_sync_system.after("movement"));
    }
}

fn player_keyboard_event_system(
	kb: Res<Input<KeyCode>>,
	mut query: Query<&mut Velocity, With<Player>>,
) {
	if let Ok(mut velocity) = query.get_single_mut() {
		velocity.x = if kb.pressed(KeyCode::Left) {
			-1.
		} else if kb.pressed(KeyCode::Right) {
			1.
		} else {
			0.
		};
        velocity.y = if kb.pressed(KeyCode::Up){
            1.
        } else if kb.pressed(KeyCode::Down){
            -1.
        }else{
            0.
        };
	}
}

fn player_spawn_system(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
    labyrinth: Res<LabyrinthState>
) {
    let (w, h) = ((win_size.w - 2. * win_size.frame_size) / labyrinth.maze.dimensions.1 as f32, (win_size.h - 2. * win_size.frame_size) / labyrinth.maze.dimensions.0 as f32);
    let (start_x, start_y) = (labyrinth.maze.start.1 as f32 * w, labyrinth.maze.start.0 as f32 * h);
    let p_size = PLAYER_ASSET_DIMENSIONS;//assets.get(&game_textures.player).un
    player_state.size = (p_size.0 * PLAYER_SCALE, p_size.1 * PLAYER_SCALE);
    player_state.position = (labyrinth.maze.start.0, labyrinth.maze.start.1);
    let pos = (start_x - win_size.w / 2. + win_size.frame_size + w/2., win_size.h / 2. - win_size.frame_size - start_y - h/2.);
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                sprite: Sprite{
                    custom_size:Some(Vec2::new(player_state.size.0, player_state.size.1)),
                    anchor: Anchor::Center,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        pos.0,
                        pos.1,
                        10.,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Player)
            .insert(Velocity { x: 0., y: 0. })
            .insert(Dimensions::new(player_state.size.0, player_state.size.1, 0.));

    player_state.spawned = true;
}

fn player_movement_system(
    mut commands: Commands,
	mut player: Query<(&Velocity, &mut Transform, &Dimensions), With<Player>>,
    mut player_state: ResMut<PlayerState>,
    labyrinth_state: Res<LabyrinthState>,
    coll_query: Query<(&Transform, &CollidableDetails), (With<Collidable>, Without<Player>)>,
    time: Res<Time>
) {
    if let Ok((velocity, mut transform, player_dim)) = player.get_single_mut(){
        let translation = &mut transform.translation;

        let dx = velocity.x * time.delta_seconds() * BASE_SPEED;
        let dy = velocity.y * time.delta_seconds() * BASE_SPEED;

        let mut flag_x= true;
        let mut flag_y= true;

        let x_pos = Vec3::new(translation.x + dx, translation.y,0.);
        let y_pos = Vec3::new(translation.x, translation.y + dy,0.);
        let player_dim = Vec2::new(player_dim.x, player_dim.y);

        for (trans, metadata) in coll_query.iter(){
            let coll_pos = Vec3::new(trans.translation.x + metadata.dim.x / 2., trans.translation.y - metadata.dim.y / 2., 0.);
            let coll_dim = Vec2::new(metadata.dim.x, metadata.dim.y);
            if flag_x && collide(
                coll_pos,
                coll_dim,
                x_pos,
                player_dim * 0.9).is_some(){
                flag_x = match metadata.c_type{
                    CollidableType::Wall => false,
                    CollidableType::Door => {
                        if player_state.key_number > 0{
                            player_state.key_number -= 1;
                            commands.entity(labyrinth_state.entities[metadata.id]).despawn();
                            true
                        }else{
                            false
                        }
                    },
                    CollidableType::Key => {
                        player_state.key_number += 1;
                        commands.entity(labyrinth_state.entities[metadata.id]).despawn();
                        true
                    },
                    CollidableType::Exit => {
                        true
                    }
                };
            }
            if flag_y && collide(
                coll_pos,
                coll_dim,
                y_pos,
            player_dim * 0.9).is_some(){
                flag_y = match metadata.c_type{
                    CollidableType::Wall => false,
                    CollidableType::Door => {
                        if player_state.key_number > 0{
                            player_state.key_number -= 1;
                            commands.entity(labyrinth_state.entities[metadata.id]).despawn();
                            true
                        }else{
                            false
                        }
                    },
                    CollidableType::Key => {
                        player_state.key_number += 1;
                        commands.entity(labyrinth_state.entities[metadata.id]).despawn();
                        true
                    },
                    CollidableType::Exit => {
                        true
                    }
                };
            }
        }

        if flag_x{
		    translation.x += dx;
        }

        if flag_y{
		    translation.y += dy;
        }
    }
}

fn inventory_sync_system(
    query: Query<Entity, With<Inventory>>,
    mut commands: Commands,
    player_state: Res<PlayerState>,
    game_textures: Res<GameTextures>
){
    for e in query.iter(){
        commands
        .entity(e)
        .insert(TextBundle{
            text: Text::from_section(
                format!("   Keys: {}", player_state.key_number),
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
}

fn setup_inventory_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    player_state: ResMut<PlayerState>
){
    commands.spawn_empty()
    .insert(Inventory)
    .insert(TextBundle{
        text: Text::from_section(
            format!("   Keys: {}", player_state.key_number),
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