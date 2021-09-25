use hstrace::{Ident, prelude::*};
use std::thread::{JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{io::Read, net::TcpListener, thread};

struct TraceMessage {
    syscall: Ident
} 
fn start_trace(process: &str, args: Vec<String>, sender: Sender<TraceMessage>) -> std::thread::JoinHandle<()> {
    let process = process.to_string();

    thread::spawn(move || {
        let mut tracer = HStraceBuilder::new().program(&process).args(args).build();

        tracer.start().unwrap();

        for syscall in tracer.iter_as_syscall() {
            match syscall.name {
                Ident::Read | Ident::Write => {
                    sender.send(TraceMessage {
                        syscall: syscall.name
                    }).unwrap();
                },
                _ => {}
            }
        }
    })
}

fn start_consumer(receiver: Receiver<TraceMessage>) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let trace_message = receiver.recv().unwrap();
            println!("Trace Message: {:?}", trace_message.syscall)
        }
    })
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:1113").unwrap();

    let (tx, rx) = channel::<TraceMessage>();

    start_consumer(rx);

    // TRACE program
    // STOP
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 1024];
                let count = stream.read(&mut buf).unwrap();
                let body = String::from_utf8_lossy(&buf[0..count]);

                let mut args = body.split(" ");
                let body = args.next();

                match body {
                    Some(body) => {
                        let command = body.trim();

                        match command {
                            "TRACE" => {
                                dbg!("Tracing started");

                                match args.next() {
                                    Some(program_name) => {
                                        let args: Vec<String> = args.map(|s| s.to_string()).collect();

                                        start_trace(program_name.trim(), args, tx.clone());
                                    },
                                    None => {
                                        dbg!("Please specify the program name.");
                                    }
                                }
                            }
                            "STOP" => {
                                dbg!("Tracing stopped.");
                            }
                            _ => {
                                dbg!("Invalid command.");
                            }
                        }
                    }
                    None => {
                        dbg!("Invalid command.");
                    }
                }
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}
