use clap_complete::{generate_to, shells::Bash};
use std::{io::Error, env};

//include!("src/config/arg_parser.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    use clap::CommandFactory;
    let mut cmd_set = config::arg_parser::ArgParser2::command();
    generate_to(Zsh, &mut cmd_set, "pewpew", outdir);

    Ok(())
}