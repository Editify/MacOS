use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1209416659874357258")?;
    client.connect()?;
    client.set_activity(activity::Activity::new().state("Idle").details("Test"))?;
    Ok(())
}
