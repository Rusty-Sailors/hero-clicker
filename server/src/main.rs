use bevy::prelude::*;
use bevy_renet::{
    renet::{
        RenetConnectionConfig, RenetServer, ServerAuthentication,
        ServerConfig, ServerEvent,
        // RenetError,
    },
    RenetServerPlugin,
};
use std::time::SystemTime;
use std::{net::UdpSocket};
// use serde::{Deserialize, Serialize};

const PROTOCOL_ID: u64 = 7;

fn main() {
    let server = new_renet_server();
    let mut app = App::new();
    app
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin)
        .insert_resource(server)
        .add_system(handle_server_events_system);
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
