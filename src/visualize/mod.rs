use bevy::{prelude::*};

use crate::maze::Maze;

mod player;
mod labyrinth;

use player::PlayerPlugin;
use labyrinth::LabyrinthPlugin;

#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
	pub frame_size: f32
}

#[derive(Resource)]
struct GameTextures {
	player: Handle<Image>,
    key: Handle<Image>,
	font: Handle<Font>
}
const FONT_PATH: &str = "font.otf";

const PLAYER_SPRITE : &str = "player.png";
const PLAYER_SCALE: f32 = 0.65;
const PLAYER_ASSET_DIMENSIONS: (f32, f32) = (144., 75.);

const KEY_SPRITE : &str = "key.png";
const KEY_SCALE: f32 = 0.5;

const FRAME_SCALE: f32 = 0.1;
const WALL_SCALE: f32 = 0.01;

const BASE_SPEED: f32 = 500.;

pub fn display(maze: Option<Maze>){
    App::new()
    .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        window: WindowDescriptor {
            title: "Labyrinth".to_string(),
        ..Default::default()
    },
    ..Default::default()
    }))
    .add_plugin(PlayerPlugin)
    .add_plugin(LabyrinthPlugin{maze_instance: maze})
    .add_startup_system(setup_system)
	.add_system(window_resize_system)
    .run();
}

fn window_resize_system(mut windows: ResMut<Windows>, mut win_size: ResMut<WinSize>) {
    let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());
	win_size.w = win_w;
	win_size.h = win_h;
	win_size.frame_size = win_w.min(win_h) * FRAME_SCALE;
}

fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut windows: ResMut<Windows>,
) {
	commands.spawn(Camera2dBundle::default());

	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

	let win_size = WinSize { w: win_w, h: win_h, frame_size: FRAME_SCALE * win_w.min(win_h) };
	commands.insert_resource(win_size);

	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
        key: asset_server.load(KEY_SPRITE),
		font: asset_server.load(FONT_PATH)
	};
	commands.insert_resource(game_textures);
}