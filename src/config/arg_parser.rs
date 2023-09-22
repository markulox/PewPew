use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Nopphon.R <nopphonyel@gmail.com>",
    version = "0.1.0",
    about = "API requests tools which perform in parallel.",
    long_about = "A CLI tool for send multiple API requests simultaneously."
)]
pub struct MainCommand {

    #[arg(value_hint=clap::ValueHint::Url)]
    /// URL which used for send requests.
    pub url:String,

    #[arg(short='r', long="repeat", value_name = "Positive Integer", default_value_t = 1)]
    #[arg(value_parser = clap::value_parser!(u64).range(1..))]
    /// Number of repeat times of each request thread.
    pub repeat:u64,

    #[arg(short='d', long="repeat_delay", value_name = "Milisec")]
    /// Delay time between each repeat.
    pub repeat_delay:Option<u64>,
 
    #[arg(short='n', long="num_gun", value_name = "Positive Integer", default_value_t = 1)]
    #[arg(value_parser = clap::value_parser!(u64).range(1..))]
    /// Number of request thread going to be spawn.
    pub gun_num:u64,

    #[arg(short, long, value_name = "HTTP Method", default_value_t = String::from("get"))]
    /// Request method
    pub method:String,

    #[arg(long, value_name="HTTP Header")]
    /// Header which going to be send when making a request in a format of <HeaderName>:<HeaderValue> ...
    pub header: Option<String>,

    #[arg(long="body.raw", value_name = "String")]
    /// Text body which going to be send when making a request.
    pub body_raw: Option<String>,

    #[arg(long="body.form", value_name="HTTP Form", conflicts_with("body_raw"))]
    /// Text body which going to be send when making a request in a format of <FormKey>:<FormValue> ...
    pub body_form: Option<String>, // Conflict with body

    #[arg(long="latency_report", value_name = "Export Path")]
    /// Specify location to export a latency delay of each request as a graoh.
    pub latency_report: Option<String>,

    #[arg(long="split_res", default_value_t = false)]
    /// Split failed request results and successfully request results.
    pub split_result: bool,

    #[arg(long="verbose", default_value_t = false)]
    /// Show response from the server.
    pub verbose: bool,

    #[arg(long="quiet", default_value_t = false)]
    /// Hide all operation log that coming from all thread.
    pub quiet: bool,
}
