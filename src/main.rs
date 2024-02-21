use clap::Parser;
use std::env;
use std::net::TcpListener;
use std::process;
use std::process::Command;
use std::thread;

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Run command in a shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,
    /// The port to use
    port: u16,
    /// The command to run
    cmd: Vec<String>,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // check command is not empty
    if args.cmd.is_empty() {
        return;
    }
    // open tcp server on port
    let address = format!("127.0.0.1:{}", args.port.to_string());
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("ERROR binding port {}: {err}", args.port);
            process::exit(1);
        }
    };
    // run listener in a separate thread
    thread::spawn(move || for _ in listener.incoming() {});
    // run command
    process::exit(run_command(args.cmd, args.shell));
}

/// Run command
fn run_command(cmd: Vec<String>, shell: bool) -> i32 {
    if shell {
        // run command in shell
        if env::consts::OS == "windows" {
            // on windows
            match Command::new("cmd").arg("/c").arg(&cmd.join(" ")).status() {
                Ok(status) => return status.code().unwrap(),
                Err(err) => {
                    eprintln!("ERROR running command: {err}");
                    process::exit(1);
                }
            }
        } else {
            // on unix
            match Command::new("sh").arg("-c").arg(&cmd.join(" ")).status() {
                Ok(status) => return status.code().unwrap(),
                Err(err) => {
                    eprintln!("ERROR running command: {err}");
                    process::exit(1);
                }
            }
        }
    } else {
        // run command without shell
        match Command::new(&cmd[0]).args(&cmd[1..]).status() {
            Ok(status) => return status.code().unwrap(),
            Err(err) => {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            }
        }
    }
}
