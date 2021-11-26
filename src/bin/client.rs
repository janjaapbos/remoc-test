use remoc::prelude::*;
use remoc::robj::rw_lock::RwLock;
use remoc_test::Data;
use std::net::Ipv4Addr;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    connect_client().await
}

async fn connect_client() {
    // Establish TCP connection.
    let socket = TcpStream::connect((Ipv4Addr::LOCALHOST, 9870))
        .await
        .unwrap();
    let (socket_rx, socket_tx) = socket.into_split();

    // Establish Remoc connection over TCP.
    // The connection is always bidirectional, but we can just drop
    // the unneeded receiver.
    let (conn, _tx, rx): (_, rch::base::Sender<RwLock<Data>>, _) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
            .await
            .unwrap();
    tokio::spawn(conn);

    // Run client.
    client(rx).await;
}

async fn client(mut rx: rch::base::Receiver<RwLock<Data>>) {
    let rw_lock = rx.recv().await.unwrap().unwrap();

    let read = rw_lock.read().await.unwrap();
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
