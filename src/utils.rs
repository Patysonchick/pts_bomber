use crate::services::get_services_list;
use chrono::Utc;

pub enum PhoneNumberTypes {
    WithPlus(String), //+7**********
    WithPlusBrackets(String) //+7(***)*******
}

pub fn format_number(phone_number: &PhoneNumberTypes) -> String {
    match phone_number {
        PhoneNumberTypes::WithPlus(number) => {
            let mut formatted_phone = "+".to_string();
            formatted_phone.push_str(number);

            formatted_phone
        },
        PhoneNumberTypes::WithPlusBrackets(number) => {
            let slice = &number[0..=10];
            let mut formatted_phone = "+".to_string();
            formatted_phone.push(slice.chars().nth(0).unwrap());
            formatted_phone.push('(');
            formatted_phone.push_str(&slice[1..4]);
            formatted_phone.push(')');
            formatted_phone.push_str(&slice[4..=10]);

            println!("{}", &formatted_phone);
            formatted_phone
        }
    }
}

#[tokio::main]
pub async fn send(phone_number: String) { //-> Result<(), Box<dyn std::error::Error>>
    let services_list = get_services_list(phone_number);
    
    let services_list_clone1 = services_list.clone();
    let thread1 = tokio::spawn(async move {
        for i in 0..services_list_clone1.len() {
            if (i+1) % 2 != 0 {
                let client = reqwest::Client::new();
                let res = client
                    .post(&services_list_clone1[i].url)
                    .json(&services_list_clone1[i].post)
                    .send()
                    .await
                    .unwrap();

                println!("\nService №{} - {}", 
                    i, 
                    &services_list_clone1[i].name);
                println!("Status - {}\n{:#?}\n", 
                    res.status(), 
                    res.text()
                    .await
                    .unwrap());
            }
        }
    });

    let services_list_clone2 = services_list.clone();
    let thread2 = tokio::spawn(async move {
        for i in 0..services_list_clone2.len() {
            if (i+1) % 2 == 0 {
                let client = reqwest::Client::new();
                let res = client
                    .post(&services_list_clone2[i].url)
                    .json(&services_list_clone2[i].post)
                    .send()
                    .await
                    .unwrap();

                println!("\nService №{} - {}", 
                    i, 
                    &services_list_clone2[i].name);
                println!("Status - {}\n{:#?}\n", 
                    res.status(), 
                    res.text()
                    .await
                    .unwrap());
            }
        }
    });

    let start_time = Utc::now();
    thread1.await.unwrap();
    thread2.await.unwrap();

    let finish_time = Utc::now();
    let duration = finish_time - start_time;
    println!("Finished\nStarted at: {}\nFinished at: {}\nWorked: {}",
        start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        finish_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        duration.num_seconds().to_string());
}