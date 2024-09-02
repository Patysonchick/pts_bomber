mod attack;
mod phone;
mod services;
mod hwid;

use crate::attack::send;
use crate::phone::{Country, Phone};
use crate::services::Victim;
use crate::hwid::check_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_key().unwrap();
    
    let victim = Victim {
        phone: Phone::new("+7 9xx xxx xx xx".to_string(), Country::Ru).expect(""),
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    };

    send(victim).await?;

    Ok(())
}
