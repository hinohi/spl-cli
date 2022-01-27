mod error;

use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use crate::error::Error;

fn parse_jsonl<R: BufRead>(reader: &mut R) -> Result<HashMap<String, serde_json::Value>, Error> {
    let mut buf = String::new();
    reader.read_line(&mut buf).map_err(Error::Io)?;
    if buf.is_empty() {
        return Err(Error::Terminal);
    }
    serde_json::from_str(&buf).map_err(Error::Json)
}

fn main() {
    let stdin = stdin();
    let mut cin = stdin.lock();
    loop {
        match parse_jsonl(&mut cin) {
            Ok(r) => println!("{:?}", r),
            Err(Error::Terminal) => break,
            Err(e) => eprintln!("{}", e),
        }
    }
}
