mod db;
mod matcher;
mod shell;
mod ui;

use clap::{Parser, Subcommand};
use db::Database;

#[derive(Parser)]
#[command(name = "smart-run", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        command: String,
        #[arg(long)]
        dir: Option<String>,
    },
    List {
        #[arg(long)]
        cmds_only: bool,
    },
    Remove {
        command: String,
    },
    Clean,
    Query {
        keywords: Vec<String>,
    },
    Init {
        shell: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { command, dir } => {
            let mut db = Database::load().unwrap_or_else(|e| {
                eprintln!("DB load error: {e}");
                std::process::exit(1);
            });
            db.add(command, dir);
            if let Err(e) = db.save() {
                eprintln!("DB save error: {e}");
                std::process::exit(1);
            }
        }
        Command::List { cmds_only } => {
            let db = Database::load().unwrap_or_else(|e| {
                eprintln!("DB load error: {e}");
                std::process::exit(1);
            });
            let current_dir = std::env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(str::to_owned));
            for entry in db.sorted_entries(current_dir.as_deref()) {
                if cmds_only {
                    println!("{}", entry.command);
                } else {
                    println!("{:.4}  {}", entry.score(current_dir.as_deref()), entry.command);
                }
            }
        }
        Command::Remove { command } => {
            let mut db = Database::load().unwrap_or_else(|e| {
                eprintln!("DB load error: {e}");
                std::process::exit(1);
            });
            let before = db.entries.len();
            db.entries.retain(|e| e.command != command);
            if db.entries.len() == before {
                eprintln!("Not found: {command}");
                std::process::exit(1);
            }
            if let Err(e) = db.save() {
                eprintln!("DB save error: {e}");
                std::process::exit(1);
            }
        }
        Command::Clean => {
            let mut db = Database::load().unwrap_or_else(|e| {
                eprintln!("DB load error: {e}");
                std::process::exit(1);
            });
            let mut seen = std::collections::HashSet::new();
            db.entries.retain(|e| seen.insert(e.command.clone()));
            if let Err(e) = db.save() {
                eprintln!("DB save error: {e}");
                std::process::exit(1);
            }
        }
        Command::Query { keywords } => {
            let db = Database::load().unwrap_or_else(|e| {
                eprintln!("DB load error: {e}");
                std::process::exit(1);
            });
            let current_dir = std::env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(str::to_owned));
            let entries = db.sorted_entries(current_dir.as_deref());
            let initial_query = keywords.join(" ");
            if let Some(command) = ui::run(entries, initial_query) {
                println!("{command}");
            } else {
                std::process::exit(1);
            }
        }
        Command::Init { shell } => {
            let script = match shell.as_str() {
                "bash" => shell::init_bash(),
                "powershell" => shell::init_powershell(),
                "fish" => shell::init_fish(),
                other => {
                    eprintln!("Unknown shell: {other}. Supported: bash, powershell, fish");
                    std::process::exit(1);
                }
            };
            print!("{script}");
        }
    }
}
