

use crate::crash as proto_gen;
use crate::crash_grpc as grpc_gen;
use chrono::{DateTime, Local, SecondsFormat};
use clap::{crate_authors, crate_version};
use env_logger;
use std::env;
use std::io::Write;

use log::{debug, info};

#[allow(warnings)]
mod crash;
#[allow(warnings)]
mod crash_grpc;
mod config;


// Setup `env_logger` builder with our log format
pub fn make_builder(max_level: Option<log::LevelFilter>) -> env_logger::Builder {
    let format = |buf: &mut env_logger::fmt::Formatter, record: &log::Record| {
        let now: DateTime<Local> = Local::now();
        let style = buf.default_level_style(record.level());
        writeln!(
            buf,
            "[{} {:<5} {}:{}] {}",
            style.value(now.to_rfc3339_opts(SecondsFormat::Micros, true)),
            buf.default_styled_level(record.level()),
            style.value(record.file().unwrap_or("")),
            style.value(record.line().unwrap_or(0)),
            style.value(record.args()),
        )
    };
    let mut builder = env_logger::Builder::new();
    builder
        .format(format)
        .filter_level(max_level.unwrap_or(log::LevelFilter::Info));
    // Overwrite the default filter if `RUST_LOG` is set.
    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    }
    // Overwrite the default write style if `RUST_LOG_STYLE` is set.
    if let Ok(var) = env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&var);
    }
    builder
}


pub struct CrashGrpcServer {
}

impl CrashGrpcServer {
    pub fn create() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

}

impl grpc_gen::CrashService for CrashGrpcServer {
    fn crash(
        &self,
        _o: grpc::RequestOptions,
        p: proto_gen::CrashRequest,
    ) -> grpc::SingleResponse<proto_gen::CrashResponse> {
        debug!("crash({:?})", p);
        grpc::SingleResponse::no_metadata(futures::future::result({
            let size = p.get_size() as usize;
            let mut result = proto_gen::CrashResponse::new();
            let mut ret = Vec::with_capacity(size);
            let mut cnt = 0;
            while cnt < size {
                ret.push((cnt%8) as f32);
                cnt += 1;
            }
            result.set_payload(ret);
            Ok(result)
        }))
    }

}

fn main() {
    env_logger::init();
    let matches = clap::App::new("Server")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Server")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Verbose logging")
        )
        .get_matches();

    let _v = match matches.occurrences_of("verbose") {
        0 => Some(log::LevelFilter::Error),
        1 => Some(log::LevelFilter::Warn),
        2 => Some(log::LevelFilter::Info),
        3 => Some(log::LevelFilter::Debug),
        _ => Some(log::LevelFilter::Trace),
    };
    // TODO: the following fails on runtime ...
    //make_builder(v).try_init().expect("Cannot set logging level");
    log_panics::init();

    let config_path = matches
        .value_of("config")
        .expect("It is there because it is required");
    let config = config::Config::from_config_file(config_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let socket_addr = config.get_socket_addr().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let borehole_service =
        CrashGrpcServer::create().unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
    let mut server = grpc::ServerBuilder::new_plain();
    server.add_service(grpc_gen::CrashServiceServer::new_service_def(
        borehole_service,
    ));
    server.http.set_addr(socket_addr).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let _server = server.build().expect("server");
    info!("Server ready. Listening on {}.", _server.local_addr());
    loop {
        std::thread::park();
    }
}
