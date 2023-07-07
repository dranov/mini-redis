use mini_redis::{clients::Client, server};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// This test fails if we are not running in the `madsim` test environment.
#[tokio::test]
async fn assert_madsim() {
    assert!(cfg!(madsim));
}

#[test]
fn kv_get_set() {
    ::tokio::madsim::runtime::Builder::from_env().run(|| async {
        let (addr, _) = start_server().await;
        let mut client = Client::connect(addr).await.unwrap();
        client.set("hello", "world".into()).await.unwrap();
        let value = client.get("hello").await.unwrap().unwrap();
        assert_eq!(b"world", &value[..])
    })
}

async fn start_server() -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c()).await });

    (addr, handle)
}


#[tokio::test]
async fn key_value_get_set() {
    let (addr, _) = start_server().await;

    let mut client = Client::connect(addr).await.unwrap();
    client.set("hello", "world".into()).await.unwrap();

    let value = client.get("hello").await.unwrap().unwrap();
    assert_eq!(b"world", &value[..])
}
