pub enum RCONType {
    Login,
    Command,
    //MultiPacket
}

impl RCONType {
    pub fn to_number(&self) -> u32 {
        match self {
            RCONType::Login => 3,
            RCONType::Command => 2,
            //RCONType::MultiPacket => 3,
        }
    }
}