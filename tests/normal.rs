extern crate laminar;

mod common;

use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};
use std::sync::mpsc;

use laminar::net::{NetworkConfig, SocketAddr};

use common::{ServerMoq, ClientStub};

const TOTAL_PACKETS_TO_SEND: u32 = 10_000;
const CLIENT_ADDR: &str = "127.0.0.1:12346";
const SERVER_ADDR: &str = "127.0.0.1:12345";

/// Test description:
/// 1. Setup receiving server.
/// 2. Send x packets to server with single client.
/// 3. Check if received data is correct.
#[test]
pub fn normal_packet_integration_test() {
    let (tx,rx) = mpsc::channel();

    let test_data = "Test Data".as_bytes();

    let mut server = ServerMoq::new(NetworkConfig::default(), true, SERVER_ADDR.parse().unwrap());
    let server_thread = server.start_receiving(rx, test_data.to_vec());

    let client = ClientStub::new(Duration::from_millis(200), CLIENT_ADDR.parse().unwrap(), TOTAL_PACKETS_TO_SEND);

    let stopwatch = Instant::now();

    server.add_client(test_data.to_vec(), client).join();

    // notify server to stop receiving.
    tx.send(true);

    let total_received = server_thread.join().unwrap();
    let elapsed_time = stopwatch.elapsed();
}
