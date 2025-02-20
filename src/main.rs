use async_nats::connect;
use flate2::read::GzDecoder;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::Read;
use tokio;

fn help(s: &str) -> String {
    println!("Help: natssend <subject> <file[.gz]| - > [<nats_url>]");
    return s.to_string();
}

async fn send() -> Result<(), Box<dyn std::error::Error>> {
    let subject = args().nth(1).ok_or_else(|| help("Param: subject"))?;
    let filename = args().nth(2).ok_or_else(|| help("Param: file"))?;
    let nats_url = args().nth(3).unwrap_or("nats://localhost:4222".to_string());

    let mut data = Vec::new();

    if filename == "-" {
        io::stdin().read_to_end(&mut data).unwrap();
    } else {
        let mut file = File::open(&filename).unwrap();

        if filename.ends_with(".gz") {
            println!("Try ungzip: {}", filename);
            let mut decoder = GzDecoder::new(file);
            decoder.read_to_end(&mut data).unwrap();
        } else {
            file.read_to_end(&mut data).unwrap();
        }
    }
    //let s = String::from_utf8(data.clone()).unwrap();
    //println!("data: {}", s);

    let client = connect(nats_url).await?;
    let len = data.len();
    client.publish(subject.clone(), data.into()).await?;
    client.flush().await?;
    println!("Published {} bytes to subject: {}", len, subject);
    return Ok(());
}

#[tokio::main]
async fn main() {
    match send().await {
        Ok(r) => {
            println!("Ok: {:?}", r)
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
