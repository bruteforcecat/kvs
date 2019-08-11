use crate::error::Result;
use crate::protocol::*;
use crate::KvsEngine;
use serde_json::Deserializer;
use slog::Logger;
use slog::*;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// The server of a key value store.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
    logger: Logger,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E, logger: Logger) -> Self {
        let kvs_server_logger = logger.new(o! {"KVS Server" => "d"});
        KvsServer {
            engine,
            logger: kvs_server_logger,
        }
    }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                    // if let Err(e) = self.serve(stream) {
                    //     error!("Error on serving client: {}", e);
                    // }
                }
                Err(e) => {
                    print!("{}", e);
                    // error!("Connection failed: {}", e),
                }
            }
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream) -> Result<()> {
        let peer_addr = stream.peer_addr()?;
        debug!(self.logger, "new tcp connection from address {}", peer_addr);
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        // this turn deserilizer into iterator over Request
        let req_iter = Deserializer::from_reader(reader).into_iter::<Request>();

        macro_rules! send_resp {
            ($resp:expr) => {{
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?;
                println!("Response sent to {}: {:?}", peer_addr, resp);
            };};
        }

        for req in req_iter {
            match req? {
                Request::Get { key } => send_resp!(match self.engine.get(key) {
                    Ok(val) => GetResponse::Ok(val),
                    Err(e) => GetResponse::Err(format!("{}", e)),
                }),
                Request::Set { key, val } => send_resp!(match self.engine.set(key, val) {
                    Ok(_) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(format!("{}", e)),
                }),
                Request::Remove { key } => send_resp!(match self.engine.remove(key) {
                    Ok(val) => RemoveResponse::Ok(val),
                    Err(e) => RemoveResponse::Err(format!("{}", e)),
                }),
            };
        }
        Ok(())
    }
}
