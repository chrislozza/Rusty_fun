use std::env;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Arguments required 'tcp/udp' 'client/server'");
        exit(1);
    }

    let service = None;
    if &args[1] == "tcp" {
        if &args[2] == "client" {
        }
        else if &args[2] == "server" {
        }
    }
    else if &args[1] == "udp" {
        if &args[2] == "client" {
        }
        else if &args[2] == "server" {
        }
    }
    else {
        println!("Arguments required 'tcp/udp' 'client/server'");
        exit(1);
    }
}
