use async_nats::{connect, jetstream};
use env_logger::Env;
use flate2::read::GzDecoder;
use log::{error, info};

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};
use tokio;

fn help(s: &str) -> String {
    println!("Help: natssend <subject> < file[.gz] | - > [<nats_url>]");
    return s.to_string();
}

async fn send() -> Result<(), Box<dyn Error>> {
    let subject = args().nth(1).ok_or_else(|| help("Param: subject"))?;
    let filename = args().nth(2).ok_or_else(|| help("Param: file"))?;
    let url = args().nth(3).unwrap_or("nats://localhost:4222".to_string());

    let mut data = Vec::new();

    if filename == "-" {
        stdin().read_to_end(&mut data).unwrap();
    } else {
        let mut file = File::open(&filename).unwrap();

        if filename.ends_with(".gz") {
            info!("Try ungzip: {}", filename);
            let mut decoder = GzDecoder::new(file);
            decoder.read_to_end(&mut data).unwrap();
        } else {
            file.read_to_end(&mut data).unwrap();
        }
    }

    let client = connect(url).await?;
    let js = jetstream::new(client);
    let len = data.len();
    let pub_ack = js.publish(subject.clone(), data.into()).await?;

    let ack = pub_ack.await?;

    info!("Published {} bytes to subject: {}\n{:?}", len, subject, ack);
    return Ok(());
}

#[tokio::main]
async fn main() {
    let level = Env::default().default_filter_or("info");
    env_logger::init_from_env(level);
    match send().await {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
