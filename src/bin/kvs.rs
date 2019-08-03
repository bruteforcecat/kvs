extern crate structopt;

use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "KVS", about = "A Key-Value Store CLI")]
enum Opt {
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

fn main() {
    match Opt::from_args() {
        Opt::Set { .. } => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        Opt::Get { .. } => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        Opt::Remove { .. } => {
            eprintln!("unimplemented");
            process::exit(1)
        }
    }
}
