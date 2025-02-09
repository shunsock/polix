mod cli;
mod controller;

use clap::Parser;
use cli::{App, Request};
use controller::{
    run_interpreter_at_once::RunInterpreterAtOnce,
    run_interpreter_interactively::RunInterpreterInteractively,
};
use log::debug;

fn main() {
    let r: Request = App::parse().get_user_input();

    let mut builder = env_logger::Builder::from_default_env();
    if r.debug {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    debug!("{:?}", r);

    match r.command {
        cli::Command::RunInteractively => {
            RunInterpreterInteractively::new().run();
        }
        cli::Command::RunAtOnce(src) => {
            RunInterpreterAtOnce::new(src).run();
        }
    }
}
