#[macro_use]
extern crate clap;

extern crate structopt;

// use kvs::KvStore;
// use kvs::KvsEngine;
use kvs::Result;
use std::net::SocketAddr;
// use std::env;kk
// use std::process;
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
// const DEFAULT_ENGINE: Engine = Engine::kvs;

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
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    unimplemented!()
}
