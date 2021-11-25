#![warn(rust_2018_idioms)]
use serde_cbor;
use yasps::server_types::{ReqType, TopicReq, TopicRes};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6666").await?;
    println!("created stream");

	let p = TopicReq {
		req_type: ReqType::Publisher,
		topic: "test",
	};

	//let (mut reader, mut writer) = stream.split();
	let v = serde_cbor::to_vec(&p)?;

	let result = stream.write_all(&v).await;
    println!("wrote to stream; success={:?}", result.is_ok());

	println!("waiting for stream to read");
	stream.readable().await?;
	println!("stream readable!!");

	let mut data = [0 as u8; 1024]; 
    match stream.try_read(&mut data) {
		Ok(0) => {
			println!("empty res");
		},
        Ok(n) => {
			let r: TopicRes = serde_cbor::from_slice(&data[0..n]).unwrap();
			println!("Reply: {:?}", r);
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    Ok(())
}
