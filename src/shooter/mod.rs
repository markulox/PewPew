use reqwest::Error;
use tokio::task::yield_now;

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

pub async fn fire_post() {}
