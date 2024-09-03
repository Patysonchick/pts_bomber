mod attack;
mod hwid;
mod phone;
mod services;

use crate::attack::send;
use crate::phone::{Country, Phone};
use crate::services::Victim;
use std::io;
// use crate::hwid::check_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check_key().unwrap();

    println!("Enter russian number");
    let mut phone = String::new();
    io::stdin().read_line(&mut phone)?;

    let victim = Victim {
        phone: Phone::new(phone, Country::Ru).expect(""),
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    };

    send(victim).await?;

    Ok(())
}
