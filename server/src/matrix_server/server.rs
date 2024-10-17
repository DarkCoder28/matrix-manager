use std::{io, net::SocketAddr, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::sleep};

use crate::{boards::BoardRender, config_manager::ConfigWrapper, state_manager::StateWrapper};

pub async fn run_matrix_server(config: ConfigWrapper, state: StateWrapper) -> io::Result<()> {
    // {
    //     let local_config = config.read().await;
    //     let board = local_config.get_boards().get("clock").expect("Failed to make board");
    //     let cfg = local_config.device_configs.get("default").unwrap();
    //     tracing::info!("{}", board.render(cfg, config.clone(), state.clone()).await);
    // }

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
    // Setup Session
    {
        match tokio::time::timeout(Duration::from_secs(5), socket.read_u64()).await {
            Ok(version) => {
                let mut config = config.write().await;
                if !config.device_configs.contains_key(&address.ip().to_string()) {
                    let default_config = config.device_configs.get("default").unwrap().clone();
                    config.device_configs.insert(address.ip().to_string(), default_config);
                    config.save();
                }
                let device_config = config.device_configs.get_mut(&address.ip().to_string()).unwrap();
                device_config.proto_version = version.unwrap_or(0);
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
    // Render Loop
    let mut current_board = 0;
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
            let board = local_config.get_boards().get(current_board_name).expect("Failed to get board (clock)");
            if socket.write_all(board.render(device_config, config.clone(), state.clone()).await.as_bytes()).await.is_err() {
                tracing::info!("Connection from [{}:{}] closed.", address.ip(), address.port());
                return;
            }
            current_board+=1;
        }
        sleep(Duration::from_secs(5)).await;
    }
}