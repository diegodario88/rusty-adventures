use bytes::Bytes;
use log::{info, warn};
use mini_redis::client;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        responder: Responder<()>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    city: String,
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Starting mini-redis client");

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let user = User {
        uuid: String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908"),
        name: String::from("John"),
        city: String::from("New York"),
        age: 30,
    };

    let user_serialized = serde_json::to_string(&user).unwrap();
    let user_in_bytes = user_serialized.as_bytes().to_vec().into();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, responder } => {
                    let res = client.get(&key).await;
                    let _ = responder.send(res);
                }
                Set {
                    key,
                    val,
                    responder,
                } => {
                    let res = client.set(&key, val).await;
                    let _ = responder.send(res);
                }
            }
        }
    });

    let getter_task = tokio::spawn(async move {
        let (responder_tx, responder_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908"),
            responder: responder_tx,
        };

        if tx.send(cmd).await.is_err() {
            warn!("connection task shutdown");
            return;
        }

        let result = responder_rx.await;

        match result {
            Ok(result) => {
                if let Some(user) = result.unwrap() {
                    let user_deserialized: Result<User, _> = serde_json::from_slice(&user);
                    match user_deserialized {
                        Ok(user) => info!("successfully got:\n {:?}", user),
                        Err(err) => warn!("failed to deserialize user: {:?}", err),
                    }
                }
            }
            Err(_) => warn!(
                "cannot set value for key {:?}",
                String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908")
            ),
        }
    });

    let setter_task = tokio::spawn(async move {
        let (responder_tx, responder_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908"),
            val: user_in_bytes,
            responder: responder_tx,
        };

        if tx2.send(cmd).await.is_err() {
            warn!("connection task shutdown");
            return;
        };

        let result = responder_rx.await;

        match result {
            Ok(_) => info!(
                "successfully set value for key {:?}",
                String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908")
            ),
            Err(_) => warn!(
                "cannot set value for key {:?}",
                String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908")
            ),
        }
    });

    setter_task.await.unwrap();
    getter_task.await.unwrap();
    manager.await.unwrap();
}
