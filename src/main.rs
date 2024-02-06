use clap::Parser;
use std::env;
use std::net::TcpListener;
use std::process;
use std::process::Command;
use std::thread;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
struct Cli {
    /// The lone version
    #[arg(short, long)]
    version: bool,
    /// Run command in a shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,
    /// The port to use
    #[arg(short, long, default_value_t = 1234)]
    port: u16,
    /// The command to run
    cmd: Vec<String>,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // print version and exit
    if args.version {
        println!("{}", VERSION);
        return;
    }
    // check command is not empty
    if args.cmd.is_empty() {
        return;
    }
    // open tcp server on port
    let mut address = String::from("127.0.0.1:");
    address.push_str(&args.port.to_string());
    let result = TcpListener::bind(address);
    if result.is_err() {
        eprintln!("ERROR: port {:?} already in use", args.port);
        process::exit(1);
    }
    // run listener in a separate thread
    thread::spawn(move || {
        let listener = result.unwrap();
        for _ in listener.incoming() {}
    });
    if args.shell {
        // run command
        if env::consts::OS == "windows" {
            // on windows
            if let Err(err) = Command::new("cmd")
                .arg("/c")
                .arg(&args.cmd.join(" "))
                .status()
            {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            };
        } else {
            // on unix
            if let Err(err) = Command::new("sh")
                .arg("-c")
                .arg(&args.cmd.join(" "))
                .status()
            {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            };
        }
    } else {
        // run command
        if let Err(err) = Command::new(&args.cmd[0]).args(&args.cmd[1..]).status() {
            eprintln!("ERROR running command: {err}");
            process::exit(1);
        };
    }
}
