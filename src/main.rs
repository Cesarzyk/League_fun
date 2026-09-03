//use buttplug::{
//    ButtplugClient,
//    connector::ButtplugRemoteClientConnector,
//    device::{ClientDeviceCommandValue, ClientDeviceOutputCommand},
//    serializer::ButtplugClientJSONSerializer,
//};

use reqwest::Client;
use std::{error::Error, fs::File, io::Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = create_client()?;
    let player_name = get_player_name(&client).await?;
    get_player_score(&client, player_name).await?;
    Ok(())
}

fn create_client() -> Result<Client, Box<dyn Error>> {
    // Certificate handling
    let mut buf = Vec::new();
    File::open("riotgames.pem")?.read_to_end(&mut buf)?;
    let cert = reqwest::Certificate::from_pem(&buf)?;
    let client: Client = reqwest::Client::builder()
        .add_root_certificate(cert)
        .use_rustls_tls()
        .build()?;
    Ok(client)
}
async fn get_player_score(client: &Client, player_name: String) -> Result<String, reqwest::Error> {
    //doing the acutal request
    let body = client
        .get(format!(
            "https://127.0.0.1:2999/liveclientdata/playerscores?riotId={}",
            &player_name[1..player_name.len() - 1]
        ))
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

async fn get_player_name(client: &Client) -> Result<String, reqwest::Error> {
    //getting the player name
    let player_name = client
        .get("https://127.0.0.1:2999/liveclientdata/activeplayername")
        .send()
        .await?
        .text()
        .await?;
    Ok(player_name)
}
