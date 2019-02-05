use std::cmp::Ordering;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::result::Result;

use log::info;

pub struct Session {
    stream: TcpStream,
    addr: SocketAddr,
    num: u32,
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    IllegalNumber,
}

impl Session {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Session {
        let num = 50;
        Session { stream, addr, num }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        info!("accept connection from {}", self.addr);
        let mut buffer = [0; 512];
        loop {
            self.write(&mut buffer, b"please input a number between 1 and 99\n")?;
            match self.read_number(&mut buffer) {
                Ok(n) => match self.num.cmp(&n) {
                    Ordering::Equal => {
                        self.write(&mut buffer, b"you win\n")?;
                        break;
                    }
                    Ordering::Less => self.write(&mut buffer, b"too big\n")?,
                    Ordering::Greater => self.write(&mut buffer, b"too small\n")?,
                },
                Err(Error::IllegalNumber) => continue,
                Err(Error::Io(e)) => return Err(e),
            };
        }
        Ok(())
    }

    fn write(&mut self, buffer: &mut [u8], contents: &[u8]) -> Result<usize, io::Error> {
        buffer[..contents.len()].copy_from_slice(contents);
        self.stream.write(&buffer[..contents.len()])
    }

    fn read_number(&mut self, buffer: &mut [u8]) -> Result<u32, Error> {
        let n_bytes = self.stream.read(buffer)
            .map_err(|e| { Error::Io(e) })?;
        let num_str = String::from_utf8(buffer[..n_bytes].to_vec())
            .map_err(|_| Error::IllegalNumber)?;
        num_str.trim().parse()
            .map_err(|_| Error::IllegalNumber)
    }
}