mod bullet;
mod config;
mod shooter;

use std::sync::Arc;

use bullet::Bullet;
use clap::Parser;
use config::Config;

use shooter::{analytic::ResultAnalyzer, ShootRes};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<(), String> {
    //let args: Vec<String> = std::env::args().collect();

    // Build the zsh auto complete
    // use clap::CommandFactory;
    // let mut cmd_set = config::arg_parser::ArgParser2::command();
    // generate(Zsh, &mut cmd_set, "pewpew", &mut io::stdout());

    let args = config::arg_parser::MainCommand::parse();

    // Now import the arguments to the config object
    let mut conf: Config = Config::new();
    match args.method.to_lowercase().as_str() {
        "get" | "g" => conf.method = reqwest::Method::GET,
        "post" | "pos" => conf.method = reqwest::Method::POST,
        "put" | "pu" => conf.method = reqwest::Method::PUT,
        "patch" | "pat" => conf.method = reqwest::Method::PATCH,
        "delete" | "del" => conf.method = reqwest::Method::DELETE,
        _ => {
            return Err(String::from("Unknown HTTP method."));
        }
    }
    conf.url = args.url;
    conf.gun_num = args.gun_num;
    conf.repeat = args.repeat;
    conf.delay = args.repeat_delay;
    conf.verbose = args.verbose;
    conf.quiet = args.quiet;

    // Initialize bullet
    let mut bullet = Bullet::new();
    if let Some(header_ezk) = args.header {
        let hm = config::ezkey_parser::parse_to_hashmap(header_ezk)?;
        bullet.import_header(hm)?;
    }
    if let Some(body_ezk) = args.body_form {
        let bdf = config::ezkey_parser::parse_to_hashmap(body_ezk)?;
        bullet.import_body_form(bdf);
    }
    if let Some(bdr) = args.body_raw {
        bullet.add_to_body_raw(bdr);
    }
    conf.bullet = bullet;

    // Spawning thread
    println!(
        "<I> Spawning {} thead{} for shooting at {} {} time{}...",
        conf.gun_num,
        {
            if conf.gun_num > 1 {
                "s"
            } else {
                ""
            }
        },
        conf.url,
        conf.repeat,
        {
            if conf.repeat > 1 {
                "s"
            } else {
                ""
            }
        }
    );
    let arc_conf = Arc::new(conf);
    let mut armory: Vec<JoinHandle<Vec<shooter::ShootRes>>> = Vec::new();
    for i in 0..arc_conf.gun_num {
        armory.push(tokio::spawn(shooter::fire(i, arc_conf.clone())));
    }

    // Start shooting
    println!("<I> Now shooting...");
    let mut all_shooting_list: Vec<ShootRes> = vec![];
    let mut thread_err: Vec<String> = vec![];
    let mut task_id: u64 = 0;
    let mut thread_err_count: u64 = 0;
    for each_task in armory {
        match each_task.await {
            Ok(mut shoot_res_list) => {
                all_shooting_list.append(&mut shoot_res_list);
            }
            Err(join_err) => {
                thread_err.push(format!("<X> GUN#{task_id}|THREAD_ERR: {join_err}"));
                thread_err_count += 1;
            }
        }
        task_id += 1;
    }

    let mut analyzer = ResultAnalyzer::new(all_shooting_list);
    if args.split_result {
        analyzer.split_err_event()?;
    }
    if let Some(file_loc) = args.latency_report {
        match analyzer.plot_latency(&file_loc) {
            Ok(res) => {
                println!("<I> {res}");
            }
            Err(e) => {
                println!("<X> {:?}", e);
            }
        }
    }

    if thread_err_count > 0 {
        Err(format!("There was a thread error occured"))
    } else {
        println!("<I> Done.");
        Ok(())
    }
}
