use crate::info::{read_request, write_response, Request, Response};
use crate::log::Log;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write};
use std::net::{IpAddr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

pub fn run(ip: IpAddr, port: u16) {
    let addr = SocketAddr::from((ip, port));
    let listener = TcpListener::bind(addr).unwrap();
    let storage = HashMap::<String, String>::new();
    let storage_ref = Arc::new(Mutex::new(storage));
    let mut for_join = Vec::new();
    for user in listener.incoming() {
        let mut user = match user {
            Ok(_) => user.unwrap(),
            Err(_) => {
                continue;
            }
        };
        let ref_clone = Arc::clone(&storage_ref);
        let mut log_clone = Arc::new(Mutex::new(Log::ConnectionEstablished));
        log_clone.lock().unwrap().print(
            user.local_addr().unwrap().ip(),
            ref_clone.lock().unwrap().len(),
        );
        let welcome = "Welcome to Hash Delivery Network by sesh\n";
        user.write_all((&welcome).as_ref()).unwrap();
        for_join.push(std::thread::spawn(move || loop {
            let req: Result<Request, Error> = read_request(&mut user);
            match req {
                Ok(Request::Load { key }) => {
                    let req = Request::Load { key: key.clone() };
                    log_clone = Arc::new(Mutex::from(Log::RequestType(req)));
                    log_clone.lock().unwrap().print(
                        user.local_addr().unwrap().ip(),
                        ref_clone.lock().unwrap().len(),
                    );
                    if ref_clone.lock().unwrap().contains_key(&key) {
                        let response = Response::Details {
                            response_status: String::from("success"),
                            requested_key: key.clone(),
                            requested_hash: ref_clone.lock().unwrap().get(&key).unwrap().clone(),
                        };
                        write_response(&response, &mut user).unwrap();
                    } else {
                        let response = Response::ResponseStatus {
                            response_status: String::from("key not found"),
                        };
                        write_response(&response, &mut user).unwrap();
                    }
                }
                Ok(Request::Store { key, hash }) => {
                    let req = Request::Store {
                        key: key.clone(),
                        hash: hash.clone(),
                    };
                    log_clone = Arc::new(Mutex::from(Log::RequestType(req)));
                    ref_clone.lock().unwrap().insert(key.clone(), hash.clone());
                    log_clone.lock().unwrap().print(
                        user.local_addr().unwrap().ip(),
                        ref_clone.lock().unwrap().len(),
                    );
                    let response = Response::ResponseStatus {
                        response_status: String::from("success"),
                    };
                    match write_response(&response, &mut user) {
                        Ok(_) => {}
                        Err(_) => {
                            continue;
                        }
                    }
                }
                Err(error) => {
                    log_clone = Arc::new(Mutex::from(Log::ConnectionLost));
                    log_clone.lock().unwrap().print(
                        user.local_addr().unwrap().ip(),
                        ref_clone.lock().unwrap().len(),
                    );
                    break;
                }
            }
        }));
    }
    for i in for_join {
        i.join();
    }
}
