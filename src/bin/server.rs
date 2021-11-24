use remoc::prelude::*;
use remoc::robj::rw_lock::{Owner, RwLock};
use std::net::Ipv4Addr;
use tokio::net::{TcpListener};
use std::sync::Arc;


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct ListItem {
    index: usize,
    item: String,
}

impl ListItem {
    fn new(i: usize, s: String) -> Self {
        ListItem {
            index: i,
            item: s,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Data {
    field1: u32,
    field2: String,
    list: Vec<ListItem>
}

#[tokio::main]
async fn main() {

    let list = vec![
        "Orange".into(),
        "Purple".into(),
        "Red".into()]
            .into_iter()
            .enumerate()
            .map(|(i, s)| ListItem::new(i, s))
            .collect();
    let data = Data { field1: 123, field2: "data".to_string(), list: list };

    let data = Arc::new(data);

    while 0 < 1 {
        let data = Arc::clone(&data);
        futures::join!(connect_server(data));
    }
}

async fn connect_server(data: Arc<Data>) {
    // Listen for incoming TCP connection.
    let listener =
        TcpListener::bind((Ipv4Addr::LOCALHOST, 9870)).await.unwrap();
    let (socket, _) = listener.accept().await.unwrap();
    let (socket_rx, socket_tx) = socket.into_split();

    // Establish Remoc connection over TCP.
    // The connection is always bidirectional, but we can just drop
    // the unneeded sender.
    let (conn, tx, _rx): (_, _, rch::base::Receiver<RwLock<Data>>) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
        .await.unwrap();
    tokio::spawn(conn);

    // Run server.
    server(tx, data).await;
}

// This would be run on the server.
async fn server(mut tx: rch::base::Sender<RwLock<Arc<Data>>>, data: Arc<Data>) {
    let owner = Owner::new(data);
    let rw_lock = owner.rw_lock();
    tx.send(rw_lock).await.unwrap();

    // The owner must be kept alive until the client is done with the lock.
    tx.closed().await;
}
