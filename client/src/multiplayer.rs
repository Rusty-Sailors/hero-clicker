use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use clicker_core::AppState;
use clicker_core::network::*;

use bevy_renet::{
    renet::{
        ClientAuthentication, RenetClient, RenetConnectionConfig
    }, RenetClientPlugin
};

pub struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin)
            .insert_resource(new_renet_client())
            .add_system(send_click.run_in_state(AppState::InGame))
            .add_system(receive_message_system.run_in_state(AppState::InGame));
    }
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

fn receive_message_system(mut client: ResMut<RenetClient>, mut gold: ResMut<clicker_core::gold::Gold>) {
    while let Some(message) = client.receive_message(0) {
        let server_message = bincode::deserialize(&message).unwrap();
        info!("Received message: {:?}", server_message);
        match server_message {
            ServerMessages::UpdateState { gold: server_gold } => {
                println!("Gold: {}", server_gold);
                gold.amount = server_gold
            }
        }
    }
}

fn send_click(mut click_events: EventReader<clicker_core::gold::ClickEvent>, mut client: ResMut<RenetClient>) {
    for _ in click_events.iter() {
        let message = bincode::serialize(&ClientMessages::ClickEvent).unwrap();
        client.send_message(0, message);
    }
}