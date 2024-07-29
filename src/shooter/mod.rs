pub mod analytic;

use crate::config::Config;
use reqwest::header::HeaderMap;
use reqwest::{RequestBuilder, StatusCode};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::{fmt::format, sync::Arc};
use tokio::{task::yield_now, time};

pub struct ShootRes {
    id: u64,
    timestamp: u128,
    iter_id: u64,
    result: Option<String>,
    err: bool,
    latency: Option<u128>,
    http_status: Option<StatusCode>,
    headers: Option<HeaderMap>,
    body_text: Option<String>
}

impl ShootRes {
    fn new() -> Self {
        ShootRes {
            id: 0,
            timestamp: 0,
            iter_id: 0,
            result: None,
            err: false,
            latency: None,
            http_status: None,
            headers: None,
            body_text: None
        }
    }

    pub fn get_latency_ms(&self) -> Option<u128> {
        self.latency
    }

    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn is_err(&self) -> bool {
        self.err
    }

    pub fn show_res(&self) -> String {
        match &self.result {
            Some(res_str) => res_str.clone(),
            None => String::from("Not firing yet..."),
        }
    }
}

pub async fn fire(gun_id: u64, arc_conf: Arc<Config>) -> Vec<ShootRes> {
    // Init ShootResult object
    let mut shoot_res_list: Vec<ShootRes> = Vec::new();

    let conf = arc_conf.as_ref();
    let met = &conf.method;
    let cli = reqwest::Client::new();
    conf.verbose_log(format!("GUN#{gun_id} Start shooting at {}", conf.url));
    for i in 0..conf.repeat {
        let mut shoot_res = ShootRes::new();
        shoot_res.id = gun_id;
        shoot_res.iter_id = i;

        let mut req_builder: RequestBuilder = cli.request(met.to_owned(), &conf.url);
        if let Some(header) = conf.bullet.get_header() {
            req_builder = req_builder.headers(header.clone()); // Use RC? since clone will take a lot of time
        }
        if let Some(form) = conf.bullet.get_form() {
            req_builder = req_builder.form(form);
        }
        if let Some(body) = conf.bullet.get_body() {
            req_builder = req_builder.body(body.clone());
        }

        // Timer here
        let fire_time = SystemTime::now();
        let resp = req_builder.send().await;
        let fire_time_elapse = fire_time.elapsed();
        shoot_res.timestamp = fire_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::default())
            .as_nanos();
        shoot_res.latency = fire_time_elapse
            .ok()
            .and_then(|d|{
                Some(d.as_millis())
            });
        // let end_fire_time = SystemTime::now()
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .unwrap();
        // shoot_res.timestamp = begin_fire_time.as_nanos();
        // shoot_res.latency = Some(end_fire_time.as_millis() - begin_fire_time.as_millis());

        yield_now().await;

        // Add delay here
        if let Some(delay_ms) = conf.delay {
            conf.verbose_log(format!("GUN#{gun_id}[{i}]|Delay for {delay_ms} ms."));
            tokio::time::sleep(time::Duration::from_millis(delay_ms)).await;
        }

        match resp {
            Ok(r) => {
                let lat = match shoot_res.get_latency_ms() {
                    Some(lat) => format!("{lat}"),
                    None => format!("N/A"),
                };
                shoot_res.http_status = Some(r.status());
                shoot_res.headers = Some(r.headers().clone());
                shoot_res.result = Some(format!("{:?}",r));
                shoot_res.body_text = Some(match r.text().await {
                    Ok(r) => r,
                    Err(e) => {
                        shoot_res.err = true;
                        format!("{:?}", e)
                    }
                });
                
                conf.log(format!("GUN#{gun_id}[{i}]|{:?}->Reply in {lat} ms", met));
                conf.verbose_log(format!("GUN#{gun_id}[{i}]|{:?}->Reply in {lat} ms|{:?}", met, shoot_res.result));
                conf.verbose_log(format!(">>GUN#{gun_id}[{i}].BODY|{:?}",shoot_res.body_text));
            }
            Err(e) => {
                conf.log(format!("GUN#{gun_id}[{i}]|{:?}->Err{:?}", met, e));
                shoot_res.result = Some(format!("{:?}", e));
                shoot_res.err = true;
            }
        }
        shoot_res_list.push(shoot_res);
    }
    shoot_res_list
}
