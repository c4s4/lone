use clap::Parser;
use std::env;
use std::net::TcpListener;
use std::process;
use std::process::Command;
use std::thread;
use anyhow::{Context, Result};

const DEFAULT_SHELL_UNIX: &str = "sh";
const DEFAULT_SHELL_WINDOWS: &str = "cmd";

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Shell to run command in (default: sh on Unix or cmd on Windows)
    #[clap(short, long)]
    shell: Option<String>,
    /// The port to use
    port: u16,
    /// The command to run
    cmd: Vec<String>,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    match run(args) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR {:#}", e);
            process::exit(1);
        },
    }
}

fn run(args: Cli) -> Result<()> {
    // check command is not empty
    if args.cmd.is_empty() {
        ()
    }
    // open tcp server on port
    let address = format!("127.0.0.1:{}", args.port.to_string());
    let listener = TcpListener::bind(address)
        .with_context(|| format!("binding port {}", args.port))?;
    // run listener in a separate thread
    thread::spawn(move || for _ in listener.incoming() {});
    // run command
    let exit_code = run_command(args.cmd, args.shell).context("running command")?;
    process::exit(exit_code);
}

/// Run command
fn run_command(cmd: Vec<String>, shell: Option<String>) -> Result<i32> {
    // run command in shell
    if env::consts::OS != "windows" {
        // on unix
        let shell = shell.unwrap_or(DEFAULT_SHELL_UNIX.to_string());
        match Command::new(shell).arg("-c").arg(&cmd.join(" ")).status() {
            Ok(status) => return Ok(status.code().unwrap()),
            Err(err) => anyhow::bail!(err),
        }
    } else {
        // on windows
        let shell = shell.unwrap_or(DEFAULT_SHELL_WINDOWS.to_string());
        match Command::new(shell).arg("/c").arg(&cmd.join(" ")).status() {
            Ok(status) => return Ok(status.code().unwrap()),
            Err(err) => anyhow::bail!(err),
        }
    }
}
