use std::{net::{TcpListener, TcpStream}, panic::PanicHookInfo, thread::{self, Thread}};
use std::io::{Read, Write}; 
use m2e::{DataPoint,PriceDB};
use byteorder::{BigEndian, ByteOrder};

fn main() {
    let listener= TcpListener::bind("127.0.0.1:3000").unwrap();
    eprintln!("Starting listener...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut db = PriceDB::new();
                thread::spawn(move || handle_connection(stream, &mut db));
            }
            Err(e) => {
                eprintln!("error occured {e}")
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, db: &mut PriceDB) {
    let mut temp_buf = [0; 1024];
    let mut buffer = Vec::new();

    match stream.peer_addr() {
        Ok(addr) => {
            eprintln!("handling connection from {}", addr);
        }

        Err(_) => {
            panic!("unable to get remote address");
        }
    }

    loop {
        while buffer.len() < 9 {
            match stream.read(&mut temp_buf) {
                Ok(0) => {
                    eprintln!("Connection closed before receiving enough data.");
                    return;
                }
                Ok(n) => buffer.extend_from_slice(&temp_buf[..n]),
                Err(e) => {
                    eprintln!("Error reading from stream: {}", e);
                    return;
                }
            }
        }
    
        if buffer.len() >= 9 {
            let first_byte = buffer[0] as char;
            eprintln!("first byte: {}", first_byte);
            eprintln!("first bytes: {:?}", &buffer);
    
            let t1 = BigEndian::read_u32(&buffer[1..5]);
            let t2 = BigEndian::read_u32(&buffer[5..9]);
    
            if first_byte == 'Q' {    
                let res = db.query(t1, t2);
                if let Err(e) = stream.write_all(&res.to_be_bytes()) {
                    eprintln!("Failed to write to stream: {}", e);
                }
            }
    
            if first_byte == 'I' {
                let dp = DataPoint::new(t1, t2);
                db.insert(dp);
            }

            buffer.drain(..9);
        }
    }
}
