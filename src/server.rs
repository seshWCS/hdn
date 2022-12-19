use crate::info::{read_request, write_response, Request, Response};
use crate::log::Log;
use std::collections::HashMap;
use std::io::{Error, Write};
use std::net::{IpAddr, SocketAddr, TcpListener};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub fn run(ip: IpAddr, port: u16) {
    let addr = SocketAddr::from((ip, port));
    let listener = match TcpListener::bind(addr) {
        Ok(tcp) => { tcp },
        Err(_) => {
            println!("Use another port");
            return;
        },
    };
    let storage = HashMap::<String, String>::new();
    let storage_ref = Arc::new(Mutex::new(storage));
    let mut for_join = Vec::new();
    let logger_ref = Arc::new(Mutex::new(Log::ConnectionEstablished));
    for user in listener.incoming() {
        let mut user = match user {
            Ok(ok_user) => ok_user,
            Err(_) => {
                continue;
            }
        };
        let storage_ref = Arc::clone(&storage_ref);
        let log_clone = Arc::clone(&logger_ref);
        *log_clone.lock().unwrap().deref_mut() = Log::ConnectionEstablished;
        log_clone.lock().unwrap().print(
            user.local_addr().unwrap().ip(),
            storage_ref.lock().unwrap().len(),
        );
        let welcome = "Welcome to Hash Delivery Network by sesh\n";
        user.write_all((&welcome).as_ref()).unwrap();

        for_join.push(std::thread::spawn(move || loop {
            let req: Result<Request, Error> = read_request(&mut user);
            match req {
                Ok(Request::Load { key }) => {
                    let req = Request::Load { key: key.clone() };
                    *log_clone.lock().unwrap().deref_mut() = Log::RequestType(req);
                    log_clone.lock().unwrap().print(
                        user.local_addr().unwrap().ip(),
                        storage_ref.lock().unwrap().len(),
                    );
                    match storage_ref.lock().unwrap().get(&key) {
                        None => {
                            let response = Response::ResponseStatus {
                                response_status: String::from("key not found"),
                            };
                            match write_response(&response, &mut user) {
                                Ok(_) => {}
                                Err(_) => {
                                    *log_clone.lock().unwrap().deref_mut() = Log::ConnectionLost;
                                    log_clone.lock().unwrap().print(
                                        user.local_addr().unwrap().ip(),
                                        storage_ref.lock().unwrap().len(),
                                    );
                                    break;
                                }
                            }
                        },
                        Some(hash) => {
                            let response = Response::Details {
                                response_status: String::from("success"),
                                requested_key: key,
                                requested_hash: hash.clone(),
                            };
                            match write_response(&response, &mut user) {
                                Ok(_) => {}
                                Err(_) => {
                                    *log_clone.lock().unwrap().deref_mut() = Log::ConnectionLost;
                                    log_clone.lock().unwrap().print(
                                        user.local_addr().unwrap().ip(),
                                        storage_ref.lock().unwrap().len(),
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok(Request::Store { key, hash }) => {
                    let req = Request::Store {
                        key: key.clone(),
                        hash: hash.clone(),
                    };
                    *log_clone.lock().unwrap().deref_mut() = Log::RequestType(req);
                    storage_ref.lock().unwrap().insert(key, hash);
                    log_clone.lock().unwrap().print(
                        user.local_addr().unwrap().ip(),
                        storage_ref.lock().unwrap().len(),
                    );
                    let response = Response::ResponseStatus {
                        response_status: String::from("success"),
                    };
                    match write_response(&response, &mut user) {
                        Ok(_) => {}
                        Err(_) => {
                            *log_clone.lock().unwrap().deref_mut() = Log::ConnectionLost;
                            log_clone.lock().unwrap().print(
                                user.local_addr().unwrap().ip(),
                                storage_ref.lock().unwrap().len(),
                            );
                            break;
                        }
                    }
                }
                Err(_) => {
                    *log_clone.lock().unwrap().deref_mut() = Log::ConnectionLost;
                    log_clone.lock().unwrap().print(
                        user.local_addr().unwrap().ip(),
                        storage_ref.lock().unwrap().len(),
                    );
                    break;
                }
            }
        }));
    }
    for i in for_join {
        i.join().unwrap();
    }
}
