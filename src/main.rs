mod commands;
mod config;
mod data;
#[macro_use]
mod utils;

use anyhow::Context;
use clap::{Parser, Subcommand};
use config::{get_config, ConfigData};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::process::exit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Parser)]
#[command(
    about = "An easy-to-use command-line utility for saving and loading window arrangements on Windows",
    author = "SirGolem",
    version
)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(short, long, default_value_t = false, help = "Enable verbose logging")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Save the current arrangement of open windows")]
    Save {
        /// Arrangement name
        name: String,
    },
    #[command(about = "Load a saved window arrangement")]
    Load {
        /// Arrangement name
        name: String,

        #[arg(
            long,
            default_value_t = false,
            help = "Close windows that are not in the arrangement"
        )]
        close_others: bool,
        #[arg(
            long,
            default_value_t = false,
            help = "Minimize windows that are not in the arrangement"
        )]
        minimize_others: bool,
    },
    #[command(about = "Remove a saved window arrangement")]
    Remove {
        /// Arrangement name
        name: String,
    },
    #[command(about = "List saved arrangements")]
    List,
    #[command(about = "Clean Windough data (default - saved arrangements)")]
    Clean {
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "Delete the root directory (all data, config, etc.)"
        )]
        all: bool,
    },
    #[command(about = "Open a directory in File Explorer")]
    OpenDir {
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "Open the root directory (default)"
        )]
        root: bool,
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "Open the data directory (where arrangements are saved)"
        )]
        data: bool,
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "Open the config directory"
        )]
        config: bool,
    },
}

lazy_static! {
    static ref PROJECT_DIRS: Arc<ProjectDirs> = Arc::new(
        ProjectDirs::from(
            if cfg!(debug_assertions) { "dev" } else { "com" },
            "SirGolem",
            if cfg!(debug_assertions) {
                "Windough-Dev"
            } else {
                "Windough"
            }
        )
        .unwrap_or_else(|| {
            printerror!("error finding project directory");
            exit(1);
        })
    );
    static ref CONFIG: Arc<ConfigData> = Arc::new(
        get_config()
            .with_context(|| "error loading config")
            .unwrap_or_else(|error| {
                printerror!("{:#}", error);
                exit(1);
            })
    );
}

static VERBOSE: AtomicBool = AtomicBool::new(false);
pub fn verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

fn main() {
    if !cfg!(windows) {
        printerror!("cannot run on this OS - only Windows is supported");
        return;
    }

    let args = Args::parse();
    VERBOSE.store(args.verbose, Ordering::Relaxed);

    let command_result = match args.command {
        Command::Save { name } => {
            commands::save(name).with_context(|| "error saving window arragement")
        }
        Command::Load {
            name,
            close_others,
            minimize_others,
        } => commands::load(name, close_others, minimize_others)
            .with_context(|| "error loading window arrangement"),
        Command::Remove { name } => {
            commands::remove(name).with_context(|| "error removing window arrangement")
        }
        Command::List => commands::list().with_context(|| "error listing saved arrangements"),
        Command::Clean { all } => commands::clean(all).with_context(|| "error cleaning data"),
        Command::OpenDir { root, data, config } => {
            commands::open_dir(root, data, config).with_context(|| "error opening directory")
        }
    };
    match command_result {
        Ok(_) => (),
        Err(error) => {
            if verbose() {
                printerror!("{:?}", error);
            } else {
                match error.source() {
                    Some(source) => printerror!("{}: {}", error, source),
                    None => printerror!("{}", error),
                }
            }
        }
    }
}
