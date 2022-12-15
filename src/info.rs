use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpStream;
// use json::JsonValue::String;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "request_type")]
#[serde(rename_all = "lowercase")]
pub enum Request {
    Store { key: String, hash: String },
    Load { key: String },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "response")]
#[serde(rename_all = "lowercase")]
pub enum Response {
    Details {
        response_status: String,
        requested_key: String,
        requested_hash: String,
    },
    ResponseStatus {
        response_status: String,
    },
}

pub fn read_request(stream: &mut TcpStream) -> Result<Request, Error> {
    let mut reader = BufReader::new(stream);
    let mut buf = Vec::<u8>::new();
    // println!("{:?}", &buf);
    let len = reader.read_until(b'}', &mut buf)?;
    if len == 0 {
        return Err(Error::new(ErrorKind::Other, "EOF"));
    }
    return Ok(serde_json::from_slice(&buf)?);
}

pub fn write_response(response: &Response, stream: &mut TcpStream) -> Result<(), Error> {
    // let buf = serde_json::to_vec(response)?;
    // stream.write_all(&buf)?;
    // Ok(())
    match response {
        Response::ResponseStatus { response_status } => {
            let out = String::from("response_status: \"") + response_status + "\"\n";
            stream.write_all(out.as_ref())?;
        }
        Response::Details {
            response_status,
            requested_key,
            requested_hash,
        } => {
            let mut out = String::from("response_status: \"") + response_status + "\"\n";
            out = out + "requested_key: \"" + requested_key + "\"\n";
            out = out + "requested_hash: \"" + requested_hash + "\"\n";
            stream.write_all(out.as_ref())?;
        }
    }
    Ok(())
    // unimplemented!();
}
