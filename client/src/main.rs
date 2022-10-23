use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_renet::{
    renet::{
        ClientAuthentication, RenetClient, RenetConnectionConfig
    }, RenetClientPlugin
};

use clicker_core::network::*;

mod hero;
mod gold;
mod camera;
mod render;

pub use camera::*;
pub use render::*;

fn main() {
    App::new()
        .add_plugin(RenetClientPlugin)
        .insert_resource(new_renet_client())
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugins(clicker_core::ClickerCorePlugins)
        .add_plugins(RenderPlugins)
        .add_system(send_click)
        .run();
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

fn send_click(mut click_events: EventReader<clicker_core::gold::ClickEvent>, mut client: ResMut<RenetClient>) {
    for _ in click_events.iter() {
        let message = bincode::serialize(&Messages::ClickEvent).unwrap();
        client.send_message(0, message);
    }
}