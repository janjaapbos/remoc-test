use remoc::prelude::*;
use remoc::robj::rw_lock::{Owner, RwLock};
use remoc_test::{Data, ListItem};
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let list = vec!["Orange".into(), "Purple".into(), "Red".into()]
        .into_iter()
        .enumerate()
        .map(|(i, s)| ListItem::new(i, s))
        .collect();
    let data = Data {
        field1: 123,
        field2: "data".to_string(),
        list: list,
    };

    let owner = Owner::new(data);
    let rw_lock = owner.rw_lock();

    connect_server(rw_lock).await;
}

async fn connect_server(rw_lock: RwLock<Data>) {
    // Listen for incoming TCP connection.
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 9870))
        .await
        .unwrap();

    while let Ok((socket, _)) = listener.accept().await {
        // Spawn a task for each client connection to be able to handle more than one
        // client at once.
        let rwlock_client = rw_lock.clone();
        tokio::spawn(async move {
            let (socket_rx, socket_tx) = socket.into_split();

            // Establish Remoc connection over TCP.
            // The connection is always bidirectional, but we can just drop
            // the unneeded sender.
            let (conn, tx, _rx): (_, _, rch::base::Receiver<RwLock<Data>>) =
                remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
                    .await
                    .unwrap();
            tokio::spawn(conn);

            // Run server.
            server(tx, rwlock_client).await;
        });
    }
}

// This would be run on the server.
async fn server(mut tx: rch::base::Sender<RwLock<Data>>, rw_lock: RwLock<Data>) {
    tx.send(rw_lock).await.unwrap();
}
