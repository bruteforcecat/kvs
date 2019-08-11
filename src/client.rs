use serde::Deserialize;
use serde_json::Deserializer;
use slog::Logger;
use slog::*;
use std::io::{BufReader, BufWriter, Write};
use std::net::SocketAddr;
use std::net::TcpStream;

use crate::error::{KvsError, Result};
use crate::protocol::{GetResponse, Request, SetResponse};

/// KVS client
pub struct KvsClient {
    addr: SocketAddr,
    logger: Logger,
}

impl KvsClient {
    /// create new kvs client
    pub fn new(addr: SocketAddr, logger: Logger) -> Self {
        let kvs_client_logger = logger.new(o!("KVS Client" => "kvs client"));
        KvsClient {
            addr: addr,
            logger: kvs_client_logger,
        }
    }

    // pub fn request(&mut self, req: Request) -> Result<()> {
    //     let tcp_reader = TcpStream::connect(self.addr)?;
    //     let tcp_writer = tcp_reader.try_clone()?;
    //     TcpStream::connect(self.addr)
    //         .map_err(|e| {
    //             crit!(self.logger, "failed to request {}: {}", self.addr, e);
    //         })
    //         .map(|stream| {
    //             let mut writer = BufWriter::new(&stream);
    //             serde_json::to_writer(&mut writer, &Request::Get { key })?;
    //             writer.flush()?;
    //             let resp = GetResponse::deserialize(&mut stream)?;
    //             match resp {
    //                 GetResponse::Ok(val) => Ok(val),
    //                 GetResponse::Err(msg) => Err(msg),
    //             }

    //         });

    //     Ok(())
    // }

    // pub fn set(&self, key: String, val: String) -> Result<()> {
    //     let req = Request::Set{
    //         key,
    //         val
    //     };
    //    TcpStream::connect(self.addr)
    //         .map_err(|e| {
    //             crit!(self.logger, "failed to request {}: {}", self.addr, e);
    //         })
    //         .map(|stream| {
    //             let mut writer = stream.try_clone()?;
    //             let cmd_reader = Deserializer::from_reader(BufReader::new(stream));
    //             serde_json::to_writer(&mut writer, &req)?;
    //             writer.flush()?;
    //             let resp = GetResponse::deserialize(&mut stream)?;
    //             match resp {
    //                 GetResponse::Ok(val) => Ok(val),
    //                 GetResponse::Err(msg) => Err(msg),
    //             }

    //         });

    //     Ok(())
    //     // let log = self.log.clone();
    // }
    /// set a value
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let tcp_reader = TcpStream::connect(self.addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        let mut reader = Deserializer::from_reader(BufReader::new(tcp_reader));
        let mut writer = BufWriter::new(tcp_writer);
        serde_json::to_writer(&mut writer, &Request::Set { key, val })?;
        writer.flush()?;
        let resp = SetResponse::deserialize(&mut reader)?;
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(msg) => Err(KvsError::StringError(msg)),
        }
    }
}
