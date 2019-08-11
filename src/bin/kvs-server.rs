#[macro_use]
extern crate clap;

extern crate structopt;

use kvs::KvStore;
// use kvs::KvsEngine;
// use kvs::Engine;
// use kvs::Server::KvsServer;
use kvs::KvsServer;
use kvs::Result;
use slog::Drain;
use slog::Logger;
use slog::*;
use slog_async::Async;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
// use std::env;kk
// use std::process;
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(Debug, StructOpt)]
#[structopt(name = "KVS Server")]
struct Opt {
    #[structopt(
        long,
        help = "Sets the listening address",
        value_name = "IP:PORT",
        raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
        parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(
        long,
        help = "Sets the storage engine",
        value_name = "ENGINE-NAME",
        raw(possible_values = "&Engine::variants()")
    )]
    engine: Option<Engine>,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    enum Engine {
        kvs,
        sled
    }
}

fn main() -> Result<()> {
    let drain = slog_json::Json::new(std::io::stdout())
        .add_default_keys()
        .build()
        .fuse();
    let async_drain = Async::new(drain).build().fuse();
    let game_info = format!("v{}", env!("CARGO_PKG_VERSION"));
    let root_log_context = o!("Super Cool Game" => game_info);
    let root_logger = Logger::root(async_drain, root_log_context);
    let opt = Opt::from_args();
    let engine = opt.engine;
    let addr = opt.addr;
    let pathbuf = PathBuf::from(env::current_dir()?);
    debug!(root_logger, "Address {}", addr);
    debug!(root_logger, "engine {:#?}", engine);

    match engine {
        Some(Engine::kvs) => {
            let engine_logger = root_logger.clone();
            let server = KvsServer::new(
                KvStore::open_with_logger(&pathbuf, engine_logger)?,
                root_logger,
            );
            server.run(addr)
        }
        Some(Engine::sled) => {
            let engine_logger = root_logger.clone();
            let server = KvsServer::new(
                KvStore::open_with_logger(&pathbuf, engine_logger)?,
                root_logger,
            );
            server.run(addr)
        }
        None => {
            let engine_logger = root_logger.clone();
            let server = KvsServer::new(
                KvStore::open_with_logger(&pathbuf, engine_logger)?,
                root_logger,
            );
            server.run(addr)
        }
    }
}
