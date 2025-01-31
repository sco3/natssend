use async_nats::jetstream;
use flate2::read::GzDecoder;
use std::env::args;
use std::fs::File;
use std::io::Read;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if args().len() < 2 {
        print!("natssend <subject> <file> [<nats url>]");
    }
    let subject = args().nth(1).expect("Subject to send data to");
    let filename = args().nth(2).expect("File to send");
    let nats_url = args().nth(3).unwrap_or("nats://localhost:4222".to_string());

    let file = File::open(filename).unwrap();
    let mut decoder = GzDecoder::new(file);
    let mut data = Vec::new();

    decoder.read_to_end(&mut data).unwrap();
    let client = async_nats::connect(nats_url).await?;
    let jetstream = jetstream::new(client);
    jetstream.publish(subject, data.into()).await?;

    return Ok(());
}
