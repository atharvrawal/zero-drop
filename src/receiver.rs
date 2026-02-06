use std::net::TcpListener;
use std::io::{Read};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();
    let (mut sock, _) = listener.accept().unwrap();

    let mut buf = vec![0u8; 512 * 1024]; // 512 KB
    let mut total: u64 = 0;

    loop {
        let n = sock.read(&mut buf).unwrap();
        if n == 0 { break; }
        total += n as u64;
    }

    println!("Received {} bytes", total);
}
