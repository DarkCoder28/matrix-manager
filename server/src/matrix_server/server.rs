use std::{io, net::SocketAddr, time::Duration};

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}, time::sleep};

use crate::{boards::BoardRender, config_manager::ConfigWrapper, state_manager::StateWrapper};

pub async fn run_matrix_server(config: ConfigWrapper, state: StateWrapper) -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:12312").await?;
    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            let config = config.clone();
            let state = state.clone();
            tokio::spawn(process_connection(socket, addr, config, state));
        }
    }
}

async fn process_connection(mut socket: TcpStream, address: SocketAddr, config: ConfigWrapper, state: StateWrapper) {
    tracing::info!("New connection from [{}:{}]", address.ip(), address.port());
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    // Setup Session
    {
        let mut version_buff = String::new();
        match tokio::time::timeout(Duration::from_secs(5), reader.read_line(&mut version_buff)).await {
            Ok(_res) => {
                let _ = version_buff.split_off(version_buff.len()-1); // Remove newline at end of message
                let mut config = config.write().await;
                if !config.device_configs.contains_key(&address.ip().to_string()) {
                    let default_config = config.device_configs.get("default").unwrap().clone();
                    config.device_configs.insert(address.ip().to_string(), default_config);
                    config.save();
                }
                let device_config = config.device_configs.get_mut(&address.ip().to_string()).unwrap();
                if let Ok(version) = version_buff.parse::<u64>() {
                    device_config.proto_version = version;
                } else {
                    device_config.proto_version = 0;
                }
            },
            Err(_) => {
                tracing::info!("Connection [{}:{}] did not send version. Falling back to legacy mode.", address.ip(), address.port());
                let mut config = config.write().await;
                if !config.device_configs.contains_key(&address.ip().to_string()) {
                    let default_config = config.device_configs.get("default").unwrap().clone();
                    config.device_configs.insert(address.ip().to_string(), default_config);
                    config.save();
                }
            }
        }
    }
    // Protocol-specific stuff
    {
        let mut config = config.write().await;
        let device_config = config.device_configs.get_mut(&address.ip().to_string()).unwrap();
        match device_config.proto_version {
            0 => {
                device_config.size = (64,32);
            }
            1 => {
                let mut size_x = String::new();
                let res = reader.read_line(&mut size_x).await;
                if res.is_err() { tracing::error!("[{}] Protocol Version 1: Couldn't receive board size x", address.ip()); panic!() }
                let _ = size_x.split_off(size_x.len()-1); // Remove newline at end of message
                let mut size_y = String::new();
                let res = reader.read_line(&mut size_y).await;
                if res.is_err() { tracing::error!("[{}] Protocol Version 1: Couldn't receive board size y", address.ip()); panic!() }
                let _ = size_y.split_off(size_y.len()-1); // Remove newline at end of message
                if let Ok(x) = size_x.parse::<u8>() {
                    if let Ok(y) = size_y.parse::<u8>() {
                        device_config.size = (x, y);
                    }
                }
            }
            _ => {}
        }
    }
    // Render Loop
    let mut current_board = 0;
    let mut board_errors = 0;
    let mut skipped_boards = 0;
    loop {
        {
            let local_config = config.read().await;
            let current_board_name;
            let device_config;
            {
                device_config = local_config.device_configs.get(&address.ip().to_string()).unwrap();
                let board_count = device_config.boards.len();
                if board_count <= 0 {
                    tracing::warn!("Connection from [{}:{}] closed because there are no boards in the device config.", address.ip(), address.port());
                    return;
                }
                if current_board >= board_count {
                    current_board = 0;
                }
                current_board_name = local_config.device_configs.get(&address.ip().to_string()).unwrap().boards.get(current_board).unwrap();
            }
            let board = local_config.get_boards().get(current_board_name).expect(&format!("Failed to get board ({})", current_board_name));
            if board.size.0 > device_config.size.0 || board.size.1 > device_config.size.1 {
                tracing::warn!("Board [{}] is too large for device [{}] to display!", current_board_name, address.ip());
                sleep(Duration::from_secs(1)).await;
                current_board+=1;
                board_errors+=1;
                if board_errors >= device_config.boards.len() {
                    tracing::error!("Board [{}] has no valid board candidates... terminating connection!", address.ip());
                    let _ = writer.shutdown();
                    return;
                }
                continue;
            }
            board_errors = 0;
            let rendered_board = board.render(device_config, config.clone(), state.clone()).await;
            if rendered_board.is_none() {
                current_board+=1;
                skipped_boards += 1;
                if skipped_boards > device_config.boards.len() {
                    sleep(Duration::from_secs(15)).await;
                }
                continue;
            }
            let rendered_board = rendered_board.unwrap();
            if writer.write_all(rendered_board.as_bytes()).await.is_err() {
                tracing::info!("Connection from [{}:{}] closed.", address.ip(), address.port());
                return;
            }
            current_board+=1;
        }
        sleep(Duration::from_secs(5)).await;
    }
}