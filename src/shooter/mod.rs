use reqwest::RequestBuilder;
use tokio::task::yield_now;

use crate::bullet;

pub async fn fire_get(gun_id: i32, url: String, repeat_times: i32) {
    for i in 0..repeat_times {
        let resp = reqwest::get(&url).await;
        yield_now().await;
        match resp {
            Ok(r) => {
                println!("GUN#{gun_id}[{i}]|-> Got {:?}", r.text().await);
            }
            Err(e) => {
                println!("GUN#{gun_id}[{i}]|-> Err {:?}", e);
            }
        }
    }
}

pub async fn fire(gun_id: i32, repeat:i64, met: reqwest::Method, url: String, bullet: bullet::Bullet) -> Result<String, String>{
    let cli = reqwest::Client::new();
    for i in 0..repeat {
        let mut req_builder: RequestBuilder = cli.request(met.clone(), &url);
        if let Some(header) = bullet.get_header() {
            req_builder = req_builder.headers(header.clone()); // Use RC? since clone will take a lot of time
        }
        if let Some(form) = bullet.get_form() {
            req_builder = req_builder.form(form);
        }
        if let Some(body) = bullet.get_body() {
            req_builder = req_builder.body(body.clone());
        }
        let resp = req_builder.send().await;
        yield_now().await;

        match resp {
            Ok(r) => {
                println!("GUN#{gun_id}[{i}]|{:?}->Got{:?}",met, r.text().await);
            }
            Err(e) => {
                println!("GUN#{gun_id}[{i}]|{:?}->Err{:?}",met, e);
            }
        }
    }
    Ok(format!("GUN#{gun_id} DONE!"))
}
