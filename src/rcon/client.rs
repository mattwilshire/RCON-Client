use std::net::TcpStream;
use std::io::{Read, Write};

use super::error::RCONError;
use super::rcontype::RCONType;

pub struct RCONClient {
    stream: TcpStream,
    packet_id: u32,
}

impl RCONClient {
    pub fn new(host: &str, password: &str) -> Result<RCONClient, RCONError> {
        if let Ok(mut stream) = TcpStream::connect(host) {
            println!("Connected to {}", host);

            send_password(&mut stream, password);

            let (resp_id, _, _) = get_data(&mut stream);
            
            if resp_id == -1 {
                return Err(RCONError {
                    message: String::from("Credentials are incorrect!")
                })
            }

            println!("Successfully logged in!");

            return Ok(RCONClient {
                stream: stream,
                packet_id: 2
            });
        } else {
            println!("ERROR connecting to {}", host);
            return Err(RCONError {
                message: String::from("Failed to connect to the server.")
            })
        }
    }

    pub fn send_command(&mut self, command: &str) {
        let mut buff: Vec<u8> = Vec::new();
        create_packet(&mut buff, RCONType::Command, self.packet_id, command);
        self.stream.write_all(&buff).unwrap();

        self.packet_id += 1;

        println!("Sent Command: {}", command);
        let (_, _, string_bytes) = get_data(&mut self.stream);
        
        if let Some(bytes) = string_bytes {
            let response = std::str::from_utf8(&bytes).expect("invalid utf-8 sequence");
            println!("Received: {}", response);
        }
    }
}

fn send_password(stream: &mut TcpStream, password: &str) {
    let mut buff_to_send: Vec<u8> = Vec::new();
    let packet_id: u32 = 1;

    create_packet(&mut buff_to_send, RCONType::Login, packet_id, password);

    stream.write_all(&buff_to_send).unwrap();
}

fn create_packet(packet: &mut Vec<u8>, rcon_type: RCONType, packet_id: u32, text: &str) {
    let size: u32 = ((4 * 2) + text.len() + 2).try_into().expect("Size of auth packet is too big!");

    push_bytes(packet, &size.to_ne_bytes());
    push_bytes(packet, &packet_id.to_ne_bytes());
    push_bytes(packet, &rcon_type.to_number().to_ne_bytes());
    push_bytes(packet, &text.as_bytes());

    packet.push(0);
    packet.push(0);
}

fn get_data(stream: &mut TcpStream) -> (i32, i32, Option<Vec<u8>>) {
    let mut recv_size = [0u8; 4];
    let mut data_size = -1;
    match stream.read_exact(&mut recv_size) {
        Ok(_) => {
            data_size = i32::from_le_bytes(recv_size);
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }

    let mut data_buff = vec![0; data_size.try_into().expect("Couldn't convert u to i ?")];
    match stream.read_exact(&mut data_buff) {
        Ok(_) => {

            let resp_id  = i32::from_le_bytes(<[u8; 4]>::try_from(&data_buff[0..4]).expect("Couldn't convert data_buff to response code;"));
            let auth_code = i32::from_le_bytes(<[u8; 4]>::try_from(&data_buff[4..8]).expect("Couldn't convert data_buff to auth code;"));
            let body: Option<Vec<u8>> = if data_size > 10 {
                Some(data_buff[8..(data_size - 2).try_into().expect("No")].to_vec())
            } else {
                None
            };

            return (resp_id, auth_code, body);
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }

    return (-1, -1, None);
}

fn push_bytes(buff: &mut Vec<u8>, bytes: &[u8]) {
    for byte in bytes {
        buff.push(*byte);
    }
}