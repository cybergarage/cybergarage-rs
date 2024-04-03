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

use std::io::Error;

pub struct Decoder {}

impl Decoder {
    pub fn from_bytes(bytes: &mut [u8]) -> Result<Vec<u8>, Error> {
        Decoder::from_str(std::str::from_utf8(bytes).unwrap())
    }

    pub fn from_str(s: &str) -> Result<Vec<u8>, Error> {
        // if len(src) == 0 {
        //     return []byte{}, nil
        // }
        // splitHexes := strings.Split(src, " ")
        // lineHexes := splitHexes[1 : hexdumpTwoColumnBytes+3]
        // var bytes []byte
        // for _, s := range lineHexes {
        //     if len(s) == 0 {
        //         continue
        //     }
        //     hexByte, err := hex.DecodeString(s)
        //     if err != nil {
        //         return bytes, err
        //     }
        //     bytes = append(bytes, hexByte...)
        // }
        // return bytes, nil
        if s.len() == 0 {
            return Ok(vec![]);
        }
        Ok(vec![])
    }
    pub fn from_lines(lines: &Vec<&str>) {}
}
