extern crate structopt;

use kvs::KvsClient;
use kvs::Result;
use slog::*;
use slog_async::Async;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "KVS Client")]
struct Opt {
    #[structopt(
        name = "IP:PORT",
        long = "addr",
        help = "Server address.",
        default_value = "127.0.0.1:4000",
        global = true
    )]
    addr: SocketAddr,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Sets a string key/value pair
    #[structopt(name = "set")]
    Set {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,

        #[structopt(help = "The value string of the key/value pair")]
        value: String,
    },

    /// Gets a string value according to passed string key
    #[structopt(name = "get")]
    Get {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,
    },

    /// Removes the string key/value pair according to the passed string key
    #[structopt(name = "rm")]
    Remove {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let addr = opt.addr;
    let drain = slog_json::Json::new(std::io::stdout())
        .add_default_keys()
        .build()
        .fuse();
    let async_drain = Async::new(drain).build().fuse();
    let game_info = format!("v{}", env!("CARGO_PKG_VERSION"));
    let root_log_context = o!("Super Cool Game" => game_info);
    let root_logger = Logger::root(async_drain, root_log_context);

    match opt.cmd {
        Command::Set { key, value } => {
            let mut client = KvsClient::new(addr, root_logger);
            client.set(key, value);
        }
        Command::Get { key } => {
            let mut client = KvsClient::new(addr, root_logger);
            client.set(key, "ok".to_string());
        }
        Command::Remove { key } => {
            let mut client = KvsClient::new(addr, root_logger);
            client.set(key, "o".to_string());
        }
    }
    Ok(())
}
