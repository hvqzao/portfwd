extern crate tokio;
use std::env;
use std::net::Ipv4Addr;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let command = args.next().unwrap();
    let fwds: Vec<String> = args.filter(|arg| {
            !arg.to_string().starts_with("-")
        }).collect();
    let verbose = !env::args().any(|arg| {
        ["-q", "--quiet"].iter().any(|x| x == &arg)
    });
    if fwds.len() == 0 {
        eprintln!("TCP Port Fwd v1.0\nUsage:\n  {} [-q|--quiet] [<bind_addr>:]<bind_port>:<dst_addr>:<dst_port> [...]", command);
        std::process::exit(1);
    }
    let pairs: Vec<(String, String)> = fwds.iter().map(|fwd| -> (String, String) { 
        let mut parts: Vec<&str> = fwd.split(":").collect();
        match parts.len() {
            3 | 4 => {
                let dst_port = parts.pop().unwrap();
                let dst_addr = match parts.pop().unwrap().parse::<Ipv4Addr>().ok() {
                    Some(addr) => {
                        addr.to_string()
                    },
                    None => {
                        eprintln!("Invalid dst IP address provided!");
                        std::process::exit(1);
                    }
                };
                let bind_port = parts.pop().unwrap();
                let bind_addr = match parts.pop() {
                    // bind addr is optional
                    Some(addr) => {
                        match addr.parse::<Ipv4Addr>().ok() {
                            Some(addr) => {
                                addr.to_string()
                            },
                            None => {
                                eprintln!("Invalid bind IP address provided!");
                                std::process::exit(1);
                            }
                        }
                    },
                    // bind to 0.0.0.0 if not provided
                    None => {
                        "0.0.0.0".to_string()
                    }
                };
                (format!("{}:{}", bind_addr, bind_port), format!("{}:{}", dst_addr, dst_port))
            }
            _ => {
                eprintln!("Unable to parse parameter provided!");
                std::process::exit(1);
            }
        }
    }).collect();
    // handle Ctrl-C
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        if verbose {
            eprintln!("Exiting ...");
        }
        std::process::exit(0);
    });
    // handle port forwards
    let tasks: Vec<_> = pairs.into_iter().map(|pair| {
        tokio::spawn(async move {
            let (src, dst) = pair.clone();
            let listener = TcpListener::bind(&src).await.expect(&format!("Failed to bind to {}.", &src));
            if verbose {
                println!("Forwarding {} -> {} ...", src, dst);
            }
            loop {
                match listener.accept().await {
                    Ok((mut input, src_address)) => {
                        if verbose {
                            println!("New connection from {} to {} --> {}", &src_address, &src, &dst);
                        }
                        let (src, dst) = pair.clone();
                        tokio::spawn(async move {
                            let mut buf = vec![0u8; 1024];
                            match TcpStream::connect(&dst).await {
                                Ok(mut output) => {
                                    loop {
                                        match input.read(&mut buf).await {
                                            Ok(0) => {
                                                break;
                                            },
                                            Ok(n) => {
                                                match output.write_all(&buf[0..n]).await {
                                                    Ok(_) => {
                                                    },
                                                    Err(_) => {
                                                        eprintln!("Failed to write to {}", &dst);
                                                    }
                                                }
                                            },
                                            Err(_) => {
                                                eprintln!("Failed to read from {}.", &src);
                                            }
                                        }
                                    }
                                },
                                Err(_) => {
                                    eprintln!("Unable to connect to {}", &dst);
                                }
                            }
                        });
                    },
                    Err(_) => {
                    }
                };
            }
        })
    }).collect();
    for task in tasks {
        let _ = task.await;
    }
    Ok(())
}
