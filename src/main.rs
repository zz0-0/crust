#[tokio::main]
async fn main() {
    send_message_to_peers().await?;
}

async fn send_message_to_peers() -> Result<(), Box<dyn std::error::Error>> {
    let mut peers = env::var("PEERS").unwrap();
    let mut peer_list = peers.split(',').collect();

    for peer in peer_list {}

    Ok(())
}

async fn receive_message_from_peers() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
