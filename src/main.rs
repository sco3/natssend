use async_nats::{connect, jetstream};
use flate2::read::GzDecoder;
use std::env::args;
use std::fs::File;
use std::io::Read;
use tokio;

fn help(s: &str) -> String {
    println!("{}", s);
    println!("Help: natssend <subjet> <file[.gz]> [<nats_url>]");
    return s.to_string();
}

async fn send() -> Result<(), Box<dyn std::error::Error>> {
    let subject = args().nth(1).ok_or(help("Param: Subject"))?;
    let filename = args().nth(2).ok_or(help("Param: File"))?;
    let nats_url = args().nth(3).unwrap_or("nats://localhost:4222".to_string());
    let mut data = Vec::new();
    let mut file = File::open(&filename).unwrap();

    if filename.ends_with(".gz") {
        let mut decoder = GzDecoder::new(file);
        decoder.read_to_end(&mut data).unwrap();
    } else {
        file.read_to_end(&mut data).unwrap();
    }

    let client = connect(nats_url).await?;
    let jetstream = jetstream::new(client);
    jetstream.publish(subject, data.into()).await?;
    return Ok(());
}

#[tokio::main]
async fn main() {
    match send().await {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
