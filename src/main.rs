mod attack;
// mod hwid;
mod phone;
mod services;

use crate::attack::send;
use crate::phone::{Country, Phone};
use crate::services::Victim;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test");
    
    let phone = loop {
        println!("Enter russian number");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Ok(t) = Phone::new(input, Country::Ru) {
            break t;
        }
    };

    let victim = Victim {
        phone,
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    };

    println!();
    send(victim).await?;

    Ok(())
}
