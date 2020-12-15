//!----------------------------------------------------------------------------
//! # Verfploter main.rs
//!----------------------------------------------------------------------------
//! ```
//! Treat command line and start VerfPloeter server/client or CLI 
//!  USAGE:
//!    verfploeter {OPTIONS}{SUBCOMMAND}
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! OPTIONS:
//!     -p, --prometheus <prometheus>    Enables prometheus metrics
//! SUBCOMMANDS:
//!     cli       Verfploeter CLI
//!     client    Launches the verfploeter client
//!     help      Prints this message or the help of the given subcommand(s)
//!     server    Launches the verfploeter server
//! ```
//!----------------------------------------------------------------------------

#![feature(drain_filter)]
#[macro_use]
extern crate log;
extern crate byteorder;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate ratelimit_meter;
extern crate socket2;
extern crate tokio;
#[macro_use]
extern crate prettytable;
extern crate hmac;
extern crate maxminddb;
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

mod cli;
mod client;
mod metrics;
mod net;
mod schema;
mod server;

use clap::{App, Arg, ArgMatches, SubCommand};

use crate::client::ClientConfig;
use crate::server::ServerConfig;
use metrics::Prometheus;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

//static INFO_URL: &'static str = "anycast-testbed.nl/9qt8h";
const INFO_URL: &'static str = "anycast-testbed.nl/9qt8h";
const PING_OUT_RATE_LIMIT: u32 =5000;

/// VerfPloeter:: main() - Treat command line and start VerfPloeter server/client or CLI 
fn main() {
    // Setup logging
    // TODO: L-> there is no logs on testbed, just stderr - some issue with verfploeter logger or rsyslog.conf
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env).init();

    let matches = parse_cmd();

    if let Some(cli_matches) = matches.subcommand_matches("cli") {
        cli::execute(cli_matches);
        return;
    }

    info!("Starting verfploeter v{} - Leandro Edition v6", env!("CARGO_PKG_VERSION"));

    // TODO: L-> what is expected as prometeus_address:port ??
    if let Some(prometheus_addr) = matches.value_of("prometheus") {
        debug!("Starting Prometheus...");
        let addr = prometheus_addr
            .parse::<SocketAddr>()
            .expect("Missing valid address for prometheus (ip:port)");
        thread::spawn(move || {
            Prometheus::new(addr).start();
        });
    }

    // TODO: L-> just to confirm is unused - what was the idea o this certificates??
    if let Some(server_matches) = matches.subcommand_matches("server") {
        debug!("Selected SERVER_MODE!");
        // Read certificate and private key from filesystem
        let mut certificate = None;
        let mut private_key = None;
        if let (Some(certificate_path), Some(private_key_path)) = (
            server_matches.value_of("certificate"),
            server_matches.value_of("private-key"),
        ) {
            certificate = read_file_content(certificate_path);
            private_key = read_file_content(private_key_path);
        }

        // Create the config struct
        let config = ServerConfig {
            certificate,
            private_key,
            port: server_matches
                .value_of("port")
                .unwrap_or("50001")
                .parse::<u16>()
                .expect("Port should be a 16-bits integer"),
        };

        // Start the server
        let mut s = server::Server::new(&config);
        s.start();

        // TODO: come up with a smarter way to keep the program alive
        // MAYBE SOMETHING LIKE THIS: icmp listener 
        //     for stream in listener.incoming() {
        //         let stream = stream.unwrap();
        //     }
        loop {
            debug!("Going into my eternal loop - until implement a better option");
            thread::sleep(Duration::from_secs(1));
        }
    } else if let Some(client_matches) = matches.subcommand_matches("client") {
        debug!("Selected CLIENT_MODE!");
        // Read certificate
        let mut certificate = None;
        if let Some(certificate_path) = client_matches.value_of("certificate") {
            certificate = read_file_content(certificate_path);
        }

        let grpc_host = client_matches.value_of("server").unwrap();
        let client_hostname = client_matches.value_of("hostname").unwrap();

        // Create the config struct
        let config = ClientConfig {
            grpc_host,
            client_hostname,
            certificate,
        };

        // Start the client
        let c = client::Client::new(&config);
        c.start();
    } else {
        error!("run with --help to see options");
    }
    debug!("exiting");
}

/// Read a client/server certificate file
fn read_file_content(path: &str) -> Option<Vec<u8>> {
    let mut buffer = Vec::new();
    BufReader::new(File::open(path).expect(&format!("Unable to open file: {}", path)))
        .read_to_end(&mut buffer)
        .expect(&format!("Unable to read file: {}", path));
    Some(buffer)
}

/// Parse $ verfploter [OPTIONS][SUBCOMANDS}  to start server, client, CLI or help (--help)
fn parse_cmd<'a>() -> ArgMatches<'a> {
    App::new("Verfploeter")
        .version(env!("CARGO_PKG_VERSION"))
        //.author(" Wouter B. de Vries <w.b.devries@utwente.nl> and Leandro Bertholdo <l.m.bertholdo@utwente.nl>")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Performs measurements")
        .arg(Arg::with_name("prometheus").short("p").long("prometheus").takes_value(true).required(false).help("Enables prometheus metrics"))
        .subcommand(SubCommand::with_name("server").about("Launches the verfploeter server")
            .arg(Arg::with_name("certificate").short("c").takes_value(true).help("Certificate to use for SSL connection from clients (PEM-encoded file)").required(false))
            .arg(Arg::with_name("private-key").short("P").takes_value(true).help("Private key to use for SSL connection from clients (PEM-encoded file)").required(false))
            .arg(Arg::with_name("port").short("p").takes_value(true).help("Port to listen on").required(false))
        )
        .subcommand(
            SubCommand::with_name("client").about("Launches the verfploeter client")
                .arg(
                    Arg::with_name("hostname")
                        .short("h")
                        .takes_value(true)
                        .help("hostname for this client")
                        .required(true)
                )
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .takes_value(true)
                        .help("hostname/ip address:port of the server")
                        .default_value("127.0.0.1:50001")
                )
                .arg(Arg::with_name("certificate").short("c").takes_value(true).help("Certificate to use for SSL connection to server (PEM-encoded file)").required(false))
        )
        .subcommand(
            SubCommand::with_name("cli").about("Verfploeter CLI")
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .takes_value(true)
                        .help("hostname/ip address:port of the server")
                        .default_value("127.0.0.1:50001")
                )
                .subcommand(SubCommand::with_name("client-list").about("retrieves a list of currently connected clients from the server"))
                .subcommand(SubCommand::with_name("start").about("performs verfploeter on the indicated client")
                    .arg(Arg::with_name("CLIENT_HOSTNAME").help("Sets the client to run verfploeter from (i.e. the outbound ping)")
                    .required(true)
                    .index(1))
                    .arg(Arg::with_name("SOURCE_IP").help("The IP to send the pings from")
                        .required(true)
                        .index(2))
                    .arg(Arg::with_name("IP_FILE").help("A file that contains IP address to ping")
                    .required(true)
                    .index(3))
                    .arg(Arg::with_name("stream")
                        .short("s")
                        .multiple(false)
                        .help("Stream results to stdout"))
                    .arg(Arg::with_name("json")
                        .short("j")
                        .multiple(false)
                        .help("Output results in JSON format"))
                    .arg(Arg::with_name("ip2country")
                        .short("c")
                        .takes_value(true)
                        .help("Adds a column with IP2Country information. Needs a path to a IP2Country database (MaxMind binary format)"))
                    .arg(Arg::with_name("ip2asn")
                        .short("a")
                        .takes_value(true)
                        .help("Adds a column with IP2ASN information. Needs a path to a IP2ASN database (MaxMind binary format)"))
                )
        )
        .get_matches()
}

// End-of-main.rs
