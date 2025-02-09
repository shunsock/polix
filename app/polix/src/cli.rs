use std::process::exit;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "shunsock", version = "0.0.1", about = "Prolix, but polished language", long_about = None)]
pub(crate) struct App {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    one_liner: Option<String>,

    #[arg(short, long)]
    file_path: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) debug: bool,
    pub(crate) command: Command,
}

#[derive(Debug)]
pub(crate) enum Command {
    RunInteractively,
    RunAtOnce(String)
}

impl App {
    pub(crate) fn get_user_input(&self) -> Request {
        self.validate().route()
    }

    fn validate(&self) -> &Self {
        if self.interactive && self.one_liner.is_some() {
            println!("[Cli User Error] Cannot use --interactive and --one-liner together");
            exit(1);
        }
        if self.interactive && self.file_path.is_some() {
            println!("[Cli User Error] Cannot use --interactive and --file-path together");
            exit(1);
        }
        if self.one_liner.is_some() && self.file_path.is_some() {
            println!("[Cli User Error] Cannot use --one-liner and --file-path together");
            exit(1);
        }
        if !self.interactive && self.one_liner.is_none() && self.file_path.is_none() {
            println!("[Cli User Error] Must use one of --interactive, --one-liner, or --file-path");
            exit(1);
        }
        self
    }

    fn route(&self) -> Request {
        if self.interactive {
            Request { debug: self.debug, command: Command::RunInteractively }
        } else if let Some(src) = &self.one_liner {
            Request { debug: self.debug, command: Command::RunAtOnce(src.clone()) }
        } else if let Some(file_path) = &self.file_path {
            let src: String = match std::fs::read_to_string(file_path) {
                Ok(src) => src,
                Err(e) => {
                    println!("[Cli Error] Failed to read file: {}", e);
                    exit(1);
                }
            };
            Request { debug: self.debug, command: Command::RunAtOnce(src) }
        } else {
            println!("[Cli Error] Unhandled route");
            exit(1);
        }
    }
}
