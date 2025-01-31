use async_nats::jetstream;
use flate2::read::GzDecoder;
use std::env::args;
use std::fs::File;
use std::io::Read;
use tokio;

#[tokio::main]
async fn main() {
    if args().len() > 2 {
        let subject = args().nth(1).expect("Subject to send data to");
        let filename = args().nth(2).expect("File to send");
        let nats_url = args().nth(3).unwrap_or("nats://localhost:4222".to_string());

        println!("File: {}", filename);
        match File::open(filename) {
            Ok(file) => {
                let mut decoder = GzDecoder::new(file);
                let mut data = Vec::new();

                match decoder.read_to_end(&mut data) {
                    Ok(_) => {
                        let client = async_nats::connect(nats_url).await;
                        match client {
                            Ok(client) => {
                                //let _inbox = client.new_inbox();
                                let jetstream = jetstream::new(client);
                                let result = jetstream.publish(subject, data.into()).await;
                                match result {
                                    Ok(ack) => match ack.await {
                                        Ok(_) => {
                                            println!("Sent and acknowledged.")
                                        }
                                        Err(e) => {
                                            eprintln!("Error ack: {}", e);
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("Cannot send: {}", e)
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Cannot connect to nats: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Cannot ungzip: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Open file error: {}", e);
            }
        }
    } else {
        print!("natssend <subject> <file> [<nats url>]");
    }
}
