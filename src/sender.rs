use std::net::TcpStream;
use std::io::{Write};
use std::time::Instant;

fn main() {
    let mut sock = TcpStream::connect("127.0.0.1:9000").unwrap();

    let buf = vec![0u8; 512 * 1024]; // 512 KB
    let start = Instant::now();
    let mut sent: u64 = 0;

    for _ in 0..20000 {
        sock.write_all(&buf).unwrap();
        sent += buf.len() as u64;
    }

    let t = start.elapsed().as_secs_f64();
    println!("Sent {} bytes in {:.2}s = {:.2} Mbps",
        sent,
        t,
        (sent as f64 * 8.0) / (t * 1e6)
    );
}
