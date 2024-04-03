// Copyright (C) 2024 Satoshi Konno All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::log::hexdump::default::*;
use crate::log::hexdump::error::ParseError;
use std::io::Error;

pub struct Decoder {}

impl Decoder {
    pub fn from_bytes(bytes: &mut [u8]) -> Result<Vec<u8>, Error> {
        Decoder::from_str(std::str::from_utf8(bytes).unwrap())
    }

    pub fn from_str(s: &str) -> Result<Vec<u8>, Error> {
        if s.len() == 0 {
            return Ok(vec![]);
        }
        let split_hexes = s.split(" ");
        let line_hexes = split_hexes[1..HEXDUMP_TWO_COLUMN_BYTES + 3];
        let mut bytes = vec![];
        for s in line_hexes {
            if s.len() == 0 {
                continue;
            }
            let hex_byte = hex::decode(s);
            match hex_byte {
                Ok(val) => bytes.push(val),
                Err(e) => return Err(ParseError::new(e.to_string().as_str())),
            }
        }
        Ok(vec![])
    }
    pub fn from_lines(lines: &Vec<&str>) {}
}
