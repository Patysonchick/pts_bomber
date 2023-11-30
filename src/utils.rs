use crate::services::{Service, get_services_list};
//use fake_user_agent::get_rua;
use chrono::Utc;

pub enum PhoneNumberTypes {
    WithPlus(String), //+7**********
    WithPlusBrackets(String) //+7(***)*******
}

pub fn format_number(phone_number: PhoneNumberTypes) -> String {
    match phone_number {
        PhoneNumberTypes::WithPlus(number) => {
            let mut formatted_phone = "+".to_string();
            formatted_phone.push_str(number.as_str());

            formatted_phone
        },
        PhoneNumberTypes::WithPlusBrackets(number) => {
            let slice = &number[0..=10];
            let mut formatted_phone = "+".to_string();
            formatted_phone.push(slice.chars().next().unwrap());
            formatted_phone.push('(');
            formatted_phone.push_str(&slice[1..4]);
            formatted_phone.push(')');
            formatted_phone.push_str(&slice[4..=10]);

            println!("{}", &formatted_phone);
            formatted_phone
        }
    }
}

async fn send(service: Service) {
    //let rua = get_rua();

    let client = reqwest::Client::builder()
        //.user_agent(rua)
        .build()
        .expect("Error while client creating");

    if !service.headers.is_empty() {
        let res = client
            .post(service.url)
            .headers(service.headers)
            .json(&service.post)
            .send()
            .await
            .unwrap();

        println!("\nService with HEADERs - {}", 
            service.name);
        println!("Status - {}\n{:#?}\n", 
            res.status(),
            res.text()
            .await
            .unwrap());
    } else {
        let res = client
            .post(service.url)
            .json(&service.post)
            .send()
            .await
            .unwrap();

        println!("\nService without HEADERs - {}", 
            service.name);
        println!("Status - {}\n{:#?}\n", 
            res.status(),
            res.text()
            .await
            .unwrap());
    }
}

#[tokio::main]
pub async fn start(phone_number: String) {
    let services_list = get_services_list(phone_number);

    let services_list_0 = services_list.clone();
    let thread0 = tokio::spawn(async move {
        for i in 0..services_list_0.len() {
            if (i+1) % 2 != 0 { 
                send(services_list_0[i].clone()).await;
            }
        }
    });

    let services_list_1 = services_list.clone();
    let thread1 = tokio::spawn(async move {
        for i in 0..services_list_1.len() {
            if (i+1) % 2 == 0 {
                send(services_list_1[i].clone()).await;
            }
        }
    });

    let start_time = Utc::now();
    thread0.await.unwrap();
    thread1.await.unwrap();

    let finish_time = Utc::now();
    let duration = finish_time - start_time;
    println!("Finished\nStarted at: {}\nFinished at: {}\nWorked: {}",
        start_time.format("%Y-%m-%d %H:%M:%S"),
        finish_time.format("%Y-%m-%d %H:%M:%S"),
        duration.num_seconds());
}