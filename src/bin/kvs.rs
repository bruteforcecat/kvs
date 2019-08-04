extern crate structopt;

use kvs::KvStore;
use kvs::Result;
use std::env;
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

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Set { key, value } => {
            let mut kv_store = KvStore::open(&env::current_dir()?)?;
            kv_store.set(key, value)?;
            process::exit(0);
        }
        Opt::Get { key } => {
            let kv_store = KvStore::open(&env::current_dir()?)?;
            match kv_store.get(key) {
                Ok(val) => match val {
                    Some(val) => println!("{}", val),
                    None => println!("Key not found"),
                },
                Err(e) => println!("{}", e),
            };
            process::exit(0)
        }
        Opt::Remove { key } => {
            let mut kv_store = KvStore::open(&env::current_dir()?)?;
            if let Err(_err) = kv_store.remove(key) {
                println!("Key not found");
                process::exit(1);
            }
            process::exit(0)
        }
    }
}
