mod rcon;
use rcon::client::RCONClient;
use rcon::error::RCONError;

use std::env;

fn main() -> Result<(), RCONError> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 4 {
        print!("Usage: rcon_client.exe IP:PORT password \"command\"");
        return Ok(());
    }

    
    let mut client = RCONClient::new(args[1].as_str(), args[2].as_str())?;
    println!("{}", client.send_command(args[3].as_str()));

    Ok(())
}
