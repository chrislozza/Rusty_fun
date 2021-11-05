use std::env;
use std::process::exit;

//mod tcp_service;
mod settings;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Arguments required 'tcp/udp' 'client/server' 'settings_file_path'");
        exit(1);
    }

    let settings = settings::Settings::new("Test");
//    let service = Box::<settings::Settings>::new(settings::Settings::new(&args[3]));
    if &args[1] == "tcp" {
        if &args[2] == "client" {
        }
//        else if &args[2] == "server" {
//        }
    }
//    else if &args[1] == "udp" {
//        if &args[2] == "client" {
//        }
//        else if &args[2] == "server" {
//        }
//    }
    else {
        println!("Arguments required 'tcp/udp' 'client/server'");
        exit(1);
    }
}
