#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate futures;
extern crate grpcio;
extern crate protobuf;

mod client;
mod schema;
mod server;

use self::schema::verfploeter::{Data, PingV4, Task};
use clap::{App, ArgMatches, SubCommand};
use futures::*;
use protobuf::RepeatedField;
use std::thread;
use std::time::Duration;

fn main() {
    // Setup logging
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    env_logger::Builder::from_env(env).init();

    let matches = parse_cmd();

    info!("Starting verfploeter v{}", env!("CARGO_PKG_VERSION"));

    if let Some(server_matches) = matches.subcommand_matches("server") {
        let mut s = server::Server::new();
        s.start();

        let mut counter = 0;
        loop {
            {
                let hashmap = s.connection_list.lock().unwrap();
                for (_, v) in hashmap.iter() {
                    let mut t = Task::new();
                    t.taskId = counter;
                    counter += 1;
                    let mut d = Data::new();
                    let mut p = PingV4::new();
                    p.source_address = 12345;
                    p.destination_addresses = vec![1234, 5678];
                    d.set_ping_v4(p);
                    t.set_data(RepeatedField::from_vec(vec![d]));
                    v.channel.clone().send(t).wait().unwrap();
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    } else if let Some(client_matches) = matches.subcommand_matches("client") {
        let c = client::Client::new();
        c.start();
    } else {
        error!("run with --help to see options");
    }
}

fn parse_cmd<'a>() -> ArgMatches<'a> {
    App::new("Verfploeter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Wouter B. de Vries <w.b.devries@utwente.nl")
        .about("Performs measurements")
        .subcommand(SubCommand::with_name("server").about("Launches the verfploeter server"))
        .subcommand(SubCommand::with_name("client").about("Launches the verfploeter client"))
        .get_matches()
}
