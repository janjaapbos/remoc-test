use remoc::prelude::*;
use remoc::robj::rw_lock::{RwLock};
use std::net::Ipv4Addr;
use tokio::net::{TcpStream};


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct RemocData {
    field1: u32,
    field2: String,
    list: Vec<ListItem>
}

#[tokio::main]
async fn main() {
    futures::join!(connect_client());
}

async fn connect_client() {
    // Wait for server to be ready.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Establish TCP connection.
    let socket =
        TcpStream::connect((Ipv4Addr::LOCALHOST, 9870)).await.unwrap();
    let (socket_rx, socket_tx) = socket.into_split();

    // Establish Remoc connection over TCP.
    // The connection is always bidirectional, but we can just drop
    // the unneeded receiver.
    let (conn, _tx, rx): (_, rch::base::Sender<RwLock<RemocData>>, _) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
        .await.unwrap();
    tokio::spawn(conn);

    // Run client.
    client(rx).await;
}


async fn client(mut rx: rch::base::Receiver<RwLock<RemocData>>) {
    let rw_lock = rx.recv().await.unwrap().unwrap();

    let read = rw_lock.read().await.unwrap();
    assert_eq!(read.field1, 123);
    println!("field1: {}", read.field1);
    println!("list: {:?}", read.list);
    println!("list[0].item: {}", read.list[0].item);
    assert_eq!(read.field2, "data");
    drop(read);

    let mut write = rw_lock.write().await.unwrap();
    write.field1 = 222;
    write.list[0].item = "Yellow".to_string();
    write.commit().await.unwrap();

    let read = rw_lock.read().await.unwrap();
    assert_eq!(read.field1, 222);
    println!("field1: {}", read.field1);
    println!("list[0].item: {}", read.list[0].item);
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct ListItem {
    index: usize,
    item: String,
}
