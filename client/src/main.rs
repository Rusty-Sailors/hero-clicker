use std::{collections::HashMap, net::UdpSocket, time::SystemTime, time::Duration};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_renet::{
    renet::{
        ClientAuthentication, RenetClient, RenetConnectionConfig
    }, RenetClientPlugin
};

const PROTOCOL_ID: u64 = 7;

mod hero;
mod gold;

pub use hero::*;
pub use gold::*;

#[derive(Debug, Serialize, Deserialize, Component)]
enum ClientMessage {
    ClickEvent
}

fn main() {
    App::new()
    .add_plugin(RenetClientPlugin)
    .insert_resource(new_renet_client())
    .add_startup_system(spawn_camera)
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::default())
    .add_plugin(HeroPlugin)
    .add_plugin(GoldPlugin)
    .add_system(send_click_event)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(current_time, socket, client_id, connection_config, authentication).unwrap()
}

fn send_click_event(mouse_button_input: Res<Input<MouseButton>>, mut client: ResMut<RenetClient>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let message = bincode::serialize(&ClientMessage::ClickEvent).unwrap();
        client.send_message(0, message)
    }
}