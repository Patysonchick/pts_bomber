mod attack;
// mod hwid;
mod phone;
mod services;

use crate::attack::send;
use crate::phone::{Country, FormatterErrors, Phone};
use crate::services::Victim;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let phone = loop {
        println!("Enter russian number");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match Phone::new(input, Country::Ru) {
            Ok(t) => break t,
            Err(e) => {
                match e {
                    FormatterErrors::IncorrectPatter => println!("Incorrect pattern\nNumber must be like 7 (9xx) xxx-xx-xx\n"),
                    FormatterErrors::IncorrectLength => println!("Incorrect number length\n")
                }
            }
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
