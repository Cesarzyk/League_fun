use buttplug::{
    ButtplugClient, ButtplugClientEvent, ButtplugWebsocketClientTransport,
    connector::ButtplugRemoteClientConnector, serializer::ButtplugClientJSONSerializer,
};
use futures::StreamExt;
use reqwest::Client;
use std::{error::Error, fs::File, io::Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    connect_to_buttplug_server().await;
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

async fn connect_to_buttplug_server() -> anyhow::Result<()> {
    let connector = ButtplugRemoteClientConnector::<
        ButtplugWebsocketClientTransport,
        ButtplugClientJSONSerializer,
    >::new(ButtplugWebsocketClientTransport::new_insecure_connector(
        "ws://127.0.0.1:12345",
    ));

    let client = ButtplugClient::new("Example Client");
    client
        .connect(connector)
        .await
        .expect("Can't connect to Buttplug Server, exiting!");
    let mut event_stream = client.event_stream();
    while let Some(event) = event_stream.next().await {
        if let ButtplugClientEvent::DeviceAdded(device) = event {
            println!("Device {} connected", device.name());
        }
    }
    Ok(())
}
