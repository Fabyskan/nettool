use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;
use std::time::Instant;

use crate::input_control::InputArguments;
pub fn tcp(arguments: &InputArguments) {
    match arguments.argument_len() {
        0 => {
            eprintln!("Tool missing Arguments! Consider tcp [IP:PORT]");
            std::process::exit(2)
        }
        1 => {
            let x = match from_something(arguments.argument(0)) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };
            match tcp_connect(x) {
                Ok(x) => x,
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::TimedOut => {
                            eprintln!("Connection to {} not succesfull! Can`t reach host.", x)
                        }
                        std::io::ErrorKind::ConnectionRefused => {
                            eprintln!("Connection to {} not succesfull! Port closed.", x)
                        }
                        _ => eprintln!("{e}"),
                    }
                    std::process::exit(1);
                }
            };
        }
        _ => {
            eprintln!("Too many arguments!");
            std::process::exit(2)
        }
    }
}

fn from_something(input: &str) -> Result<SocketAddr, String> {
    let test: SocketAddr = input
        .parse()
        .map_err(|_| "Wrong input! Consider 10.0.5.77:22")?;
    Ok(test)
}
fn tcp_connect(x: SocketAddr) -> std::io::Result<()> {
    let time: Duration = Duration::new(5, 0);
    let now = Instant::now();
    TcpStream::connect_timeout(&x, time)?;
    println!("Connection to {} succesfull! Responded in {} ms",x, now.elapsed().as_millis());
    Ok(())
}
