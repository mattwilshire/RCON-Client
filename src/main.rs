mod rcon;
use rcon::client::RCONClient;
use rcon::error::RCONError;

fn main() -> Result<(), RCONError> {

    let mut client = RCONClient::new("192.168.1.98:25575", "abc123")?;

    client.send_command("Broadcast Hello_Worldddd!");
    client.send_command("Broadcast Server_restart_in_5!");

    //client.send_command("DoExit");
    client.send_command("ShowPlayers");

    Ok(())
}