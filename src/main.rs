//! A simplified implementation of the classic game "Breakout".

use bevy::{prelude::*, render::camera::ScalingMode, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(start_matchbox_socket)
        .add_system(move_player)
        // .add_system(wait_for_players)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    // 创建固定大小为10的2d相机 窗口高度为10
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct Player;

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }
    println!("move");
    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);
    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/chinese-chess?next=2";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);
    IoTaskPool::get().spawn(message_loop).detach();
    commands.insert_resource(Socket(Some(socket)));
}

fn wait_for_players(mut socket: ResMut<Socket>) {
    let socket = socket.as_mut();
    if socket.0.is_none() {
        return;
    }
    socket.0.as_mut().unwrap().accept_new_connections();
    let players = socket.0.as_ref().unwrap().players();
    let num_player = 2;
    if players.len() < num_player {
        return;
    }
    info!("all peers have joined, going in-game");
}

#[derive(Resource)]
struct Socket(Option<WebRtcSocket>);
