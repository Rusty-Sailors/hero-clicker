use bevy::prelude::*;
use bevy_renet::{
    renet::{
        RenetConnectionConfig, RenetServer, ServerAuthentication,
        ServerConfig, ServerEvent,
    },
    RenetServerPlugin,
};
use std::time::SystemTime;
use std::{
    net::UdpSocket,
    time::Duration,
};

use clicker_core::network::*;
use clicker_core::gold::ClickEvent;

fn main() {
    let server = new_renet_server();
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetServerPlugin)
        .add_plugins(clicker_core::ClickerCorePlugins)
        .insert_resource(server)
        .insert_resource(SyncTimer{ timer: Timer::new(Duration::from_millis(500), true)})
        .add_system_to_stage(CoreStage::PreUpdate, handle_server_events_system)
        .add_system_to_stage(CoreStage::PreUpdate, handle_client_messages)
        .add_system(send_state_update);
    app.run();
}

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn handle_server_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Client {} connected", id);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Client {} disconnected", id);
            }
        }
    }
}

fn handle_client_messages(mut server: ResMut<RenetServer>, mut writer: EventWriter<ClickEvent>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, 0) {
            let msg = bincode::deserialize(&message).unwrap();
            match msg {
                ClientMessages::ClickEvent => {
                    writer.send(ClickEvent);
                    info!("Got ClickEvent");
                }
            }
        }
    }
}

pub struct SyncTimer {
    pub timer: Timer
}

fn send_state_update(mut sync_timer: ResMut<SyncTimer>, time: Res<Time>, gold: Res<clicker_core::gold::Gold>, mut server: ResMut<RenetServer>) {
    sync_timer.timer.tick(time.delta());
    if sync_timer.timer.just_finished() {
        let state = ServerMessages::UpdateState{
            gold: gold.amount,
        };
        let message = bincode::serialize(&state).unwrap();
        for client_id in server.clients_id().iter() {
            server.send_message(*client_id, 0, message.clone());
        }
    }
}