use crate::services::{construct_services_list, Service, Victim};
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use reqwest::{ClientBuilder, Response};
use std::collections::HashMap;

pub async fn send(victim: Victim) -> Result<(), Box<dyn std::error::Error>> {
    let services = construct_services_list(victim);

    for service in services {
        send_single(service).await?;
    }

    Ok(())
}

async fn send_single(service: Service) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .default_headers(service.headers)
        .build()
        .expect("");

    println!("Sending {}", service.name);

    match service.method {
        Method::GET => {
            let res = client.get(service.url).form(&service.fields).send().await?;

            println!("{}", res.status());
            println!("{}", res.text().await?);
        }
        Method::POST => {
            let res = client
                .post(service.url)
                .form(&service.fields)
                .send()
                .await?;

            println!("{}", res.status());
            println!("{}", res.text().await?);
        }
        _ => {}
    }

    println!("{} sent", service.name);

    Ok(())
}
