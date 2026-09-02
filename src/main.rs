use std::{error::Error, fs::File, io::Read};

//use buttplug::{
//    ButtplugClient,
//    connector::ButtplugRemoteClientConnector,
//    device::{ClientDeviceCommandValue, ClientDeviceOutputCommand},
//    serializer::ButtplugClientJSONSerializer,
//};
use reqwest::{Certificate, ClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let riot_id = "Cesarzyk#6088";
    // Certificate handling
    let mut buf = Vec::new();
    File::open("riotgames.pem")?.read_to_end(&mut buf)?;
    let cert = reqwest::Certificate::from_pem(&buf)?;
    let client = reqwest::Client::builder()
        .add_root_certificate(cert)
        .use_rustls_tls()
        .build()?;
    let player_name = client
        .get("https://127.0.0.1:2999/liveclientdata/activeplayername")
        .send()
        .await?
        .text()
        .await?;
    let body = client
        .get(format!(
            "https://127.0.0.1:2999/liveclientdata/playerscores?riotId={}",
            &player_name[1..player_name.len() - 1]
        ))
        .send()
        .await?
        .text()
        .await?;
    println!("body = {body}");
    println!("https://127.0.0.1:2999/liveclientdata/playerscores?riotId={player_name}");
    Ok(())
}
