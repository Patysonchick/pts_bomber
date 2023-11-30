use crate::utils::{PhoneNumberTypes, format_number};
use std::collections::HashMap;
use reqwest::header::HeaderMap;

#[derive(Clone)]
pub struct Service {
    pub name: String,
    pub url: String,
    pub headers: HeaderMap,
    pub post: HashMap<String, String>
}

/*
Образец

{
    let mut service = Service {
        name: todo!(),
        url: todo!(),
        headers: HeaderMap::new(),
        post: HashMap::new()
    };

    service.headers.insert(
        "",
        r#""#.parse().unwrap());

    service.post.insert(
        "".to_string(),
        phone_number.clone());

    services_list.push(service);
}
*/

pub fn get_services_list(phone_number: String) -> Vec<Service> {
    let mut services_list = Vec::new();

    // MyGames
    { 
        let mut service = Service {
            name: "MyGames".to_string(),
            url: "https://account.my.games/signup_phone_init/".to_string(),
            headers: HeaderMap::new(),
            post: HashMap::new()
        };

        service.post.insert(
            "csrfmiddlewaretoken".to_string(), 
            "B6GwyuwOSpMCrx80eXfOWAAKsqHR3qjBv7UYwkpYprKv7LOJCmfYwvwWVmIHmeRQ".to_string());
        service.post.insert(
            "continue".to_string(), 
            "https://account.my.games/profile/userinfo/".to_string());
        service.post.insert(
            "lang".to_string(),
            "ru_RU".to_string());
        service.post.insert(
            "adId".to_string(),
            "0".to_string());
        service.post.insert(
            "phone".to_string(), 
            phone_number.clone());
        service.post.insert(
            "password".to_string(), 
            "marchenkoserofim@gmail.com".to_string());
        service.post.insert(
            "method".to_string(), 
            "phone".to_string());
        
        services_list.push(service);
    }
    
    services_list
}