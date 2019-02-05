use std::io;
use std::net::TcpListener;
use std::result::Result;

use xdbexecutor;

mod session;

use session::Session;

fn run(addr: &str) -> Result<(), io::Error> {
    let listener = TcpListener::bind(addr)?;
    println!("listen at {}", addr);
    let executor = xdbexecutor::ThreadPool::new(4);
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
    run("127.0.0.1:8080").unwrap();
}
