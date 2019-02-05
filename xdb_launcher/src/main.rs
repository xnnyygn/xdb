use std::io;
use std::net::TcpListener;
use std::result::Result;

use log::info;
use env_logger;

use xdb_executor::ThreadPool;

mod session;

use session::Session;

fn run(addr: &str) -> Result<(), io::Error> {
    let listener = TcpListener::bind(addr)?;
    info!("listen at {}", addr);
    let executor = ThreadPool::new(4);
    loop {
        let (stream, addr) = listener.accept()?;
        executor.execute(move || {
            if let Err(e) = Session::new(stream, addr).run() {
                println!("{:?}", e);
            }
        });
    }
//    println!("shutdown");
//    Ok(())
}

fn main() {
    env_logger::init();
    run("127.0.0.1:8080").unwrap();
}
