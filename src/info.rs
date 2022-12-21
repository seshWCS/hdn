use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpStream;
use json::JsonValue;
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
    match response {
        Response::ResponseStatus { response_status } => {
            let mut out = JsonValue::new_array();
            out["response_status"] = JsonValue::String(response_status.to_string());
            stream.write_all(out.to_string().as_ref())?;
            stream.write("\n".as_ref())?;
        }
        Response::Details {
            response_status,
            requested_key,
            requested_hash,
        } => {
            let mut out = JsonValue::new_array();
            out["response_status"] = JsonValue::String(response_status.to_string());
            out["requested_key"] = JsonValue::String(requested_key.to_string());
            out["requested_hash"] = JsonValue::String(requested_hash.to_string());
            stream.write_all(out.to_string().as_ref())?;
            stream.write("\n".as_ref())?;
        }
    }
    Ok(())
}
