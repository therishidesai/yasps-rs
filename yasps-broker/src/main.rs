#![warn(rust_2018_idioms)]

use serde_cbor;
use yasps::server_types::{ReqType, TopicReq, TopicRes};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6666".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

				println!("received {} bytes", n);
				let req: TopicReq = serde_cbor::from_slice(&buf[0..n]).unwrap();
				match req.req_type {
					ReqType::Publisher => {
						// TODO: check if publisher exists, if it does return TopicRes with None topic
						//       if it doesn't exist, create eventfd and shared mem queue
						let res = TopicRes {
							topic: Some(req.topic),
						};
						let res_v = serde_cbor::to_vec(&res).unwrap();
						socket
							.write_all(&res_v)
							.await
							.expect("failed to write data to socket");
						
					},

					ReqType::Subscriber => {
						// TODO: check if publisher exists for this topic, if not return TopicRes with None topic
						socket
							.write_all(&buf[0..n])
							.await
							.expect("failed to write data to socket");
					}
				}

            }
        });
    }
}
