use crate::utils::{PhoneNumberTypes, format_number};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Service {
    pub name: String,
    pub url: String,
    pub post: HashMap<String, String>
}

pub fn get_services_list(phone_number: String) -> Vec<Service> {
    let mut services_list = Vec::new();

    /*{
        "last_name": "Kuchiki",
        "first_name": "Byakuya",
        "email": "marchenkoserofim@gmail.com",
        "phone": "+7(***)*******"
    }*/
    /*{ // Gosuslugi
        let mut service = Service {
            name: "Gosuslugi".to_string(),
            url: "https://www.gosuslugi.ru/auth-provider/mobile/register".to_string(),
            post: HashMap::new(),
        };

        service.post.insert(
            "last_name".to_string(),
            "Kuchiki".to_string()
        );
        service.post.insert(
            "first_name".to_string(),
            "Byakuya".to_string()
        );
        service.post.insert(
            "email".to_string(),
            "marchenkoserofim@gmail.com".to_string()
        );
        service.post.insert(
            "phone".to_string(),
            format_number(&PhoneNumberTypes::WithPlusBrackets(phone_number.to_string())));

        services_list.push(service);
    }*/

    { // Telegram
        let mut service = Service {
            name: "Telegram".to_string(),
            url: "https://my.telegram.org/auth/send_password".to_string(),
            post: HashMap::new(),
        };

        service.post.insert(
            "phone".to_string(),
            format_number(&PhoneNumberTypes::WithPlus(phone_number.to_string())));
        
        services_list.push(service);
    }

    { // MyGames
        let mut service = Service {
            name: "MyGames".to_string(),
            url: "https://account.my.games/signup_phone_init/".to_string(),
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
            phone_number.to_string());
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