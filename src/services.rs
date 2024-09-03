use crate::phone::{
    Country,
    FormatterTypes::{WithPlus, WithPlusBracketsHyphen, WithPlusHyphen},
    Phone,
};
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub service_type: ServiceType,
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub body_type: BodyType,
    pub body: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum ServiceType {
    Sms,
    Call,
    ServiceSms,
}

#[derive(Debug, Clone)]
pub enum BodyType {
    JSON,
    Form,
}

#[derive(Debug, Clone)]
pub struct Victim {
    pub phone: Phone,
    pub email: String,
    pub name: String,
    pub surname: String,
}

/*
//
{
    let mut service = Service {
        name: "".to_string(),
        service_type: ServiceType::,
        method: Method::,
        url: "".to_string(),
        headers: HeaderMap::new(),
        body_type: BodyType::,
        body: Default::default(),
    };

    service.headers.insert("", r#""#.parse().unwrap());

    let mut phone = victim.phone.clone();
    phone.format(WithPlus);
    service.body = json!({
        "phone": phone.phone
    });

    services.push(service);
}
*/

pub fn construct_services_list(victim: Victim) -> Vec<Service> {
    let mut services = Vec::new();

    match victim.phone.country {
        Country::Ru => {
            // Telegram
            {
                let mut service = Service {
                    name: "Telegram".to_string(),
                    service_type: ServiceType::ServiceSms,
                    method: Method::POST,
                    url: "https://my.telegram.org/auth/send_password".to_string(),
                    headers: HeaderMap::new(),
                    body_type: BodyType::Form,
                    body: Default::default(),
                };

                service.headers.insert(
                    "User-Agent",
                    r#"Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0"#
                        .parse()
                        .unwrap(),
                );
                service.headers.insert(
                    "Accept",
                    r#"application/json, text/javascript, */*; q=0.01"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Language",
                    r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Encoding",
                    r#"gzip, deflate, br, zstd"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Content-Type",
                    r#"application/x-www-form-urlencoded; charset=UTF-8"#
                        .parse()
                        .unwrap(),
                );
                service
                    .headers
                    .insert("X-Requested-With", r#"XMLHttpRequest"#.parse().unwrap());
                service
                    .headers
                    .insert("Origin", r#"https://my.telegram.org"#.parse().unwrap());
                service.headers.insert("DNT", r#"1"#.parse().unwrap());
                service.headers.insert("Sec-GPC", r#"1"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert(
                    "Referer",
                    r#"https://my.telegram.org/auth/"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Sec-Fetch-Dest", r#"empty"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Mode", r#"cors"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Site", r#"same-origin"#.parse().unwrap());
                service
                    .headers
                    .insert("Priority", r#"u=0"#.parse().unwrap());
                service.headers.insert("TE", r#"trailers"#.parse().unwrap());

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service.body = json!({
                    "phone": phone.phone
                });

                services.push(service);
            }
        }
    }

    services
}
