use crate::services::{construct_services_list, BodyType, Service, Victim};
use reqwest::{Client, Method};

pub async fn send(victim: Victim) -> Result<(), Box<dyn std::error::Error>> {
    let services = construct_services_list(victim);

    for service in services {
        send_single(service).await?;
    }

    Ok(())
}

async fn send_single(service: Service) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0")
        .default_headers(service.headers)
        .build()
        .expect("");

    println!("Sending {}", service.name);

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
    println!("{} sent", service.name);

    Ok(())
}
