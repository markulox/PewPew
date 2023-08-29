use reqwest::Method;
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
        match met {
            Method::GET => {
                //cli.get(url).header(bullet.get_header());
            },
            Method::POST => {
                cli.post(url).headers(headers).form(bullet.get_body());
            },
            _ => {}
        }
    }
    Ok(format!("GUN#{gun_id} DONE!"))
}
