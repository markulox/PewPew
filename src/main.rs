use config::Config;
use tokio::task::JoinHandle;

mod config;
mod shooter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    match config::Config::new_by_load_args_vec(args) {
        Ok(conf) => {
            println!(
                "URL:{} GUN_NUM:{} REPEAT:{}",
                conf.get_url(),
                conf.get_gun_num(),
                conf.get_repeat()
            );
        }
        Err(err_msg) => {
            println!("<X> {err_msg}");
        }
    }

    // Check armory size
    // let mut armory_size: i32 = 1;
    // if let Some(arg_n) = args_map.get("-n") {
    //     if let Ok(n) = arg_n.get(0).unwrap().parse::<i32>(){
    //         armory_size = n;
    //     }
    // } else {
    //     println!("<I> Using default armory size (aka gun num) = 1");
    // }

    // let mut repeat: i32 = 1;
    // if let Some(arg_repeat) = args_map.get("-r") {
    //     if let Ok(r) = arg_repeat.get(0).unwrap().parse::<i32>(){
    //         repeat = r;
    //     }
    // }

    // if let Some(arg_url) = args_map.get("--url") {
    //     let url = arg_url.get(0).expect("<X> There is no URL to read.");

    //     let mut armory:Vec<JoinHandle<()>> = Vec::new();
    //     for i in 0..armory_size {
    //         armory.push(
    //             tokio::spawn(
    //                 shooter::fire_get(i, url.clone(), repeat)
    //             )
    //         )
    //     }

    //     for each_task in armory {
    //         each_task.await.expect("<X> Gun Break!");
    //     }

    // } else {
    //     println!("<X> URL is not specified.");
    // }

    // TODO: Implement body, header support (bullet customization) on
}
