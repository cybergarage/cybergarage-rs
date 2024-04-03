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

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

use cybergarage::log::hexdump::Decoder;

fn usages() {
    println!("Usage: hexdump2bin <input hexdump file> <output binary file>");
    println!(" -h : Print this message");
}

fn main() {
    if env::args().len() < 2 {
        usages();
        return;
    }

    let args: Vec<String> = env::args().collect();
    let hexdump_filename = &args[1];
    let hexdump_str = fs::read_to_string(hexdump_filename);
    if hexdump_str.is_err() {
        println!("Failed to read the hexdump ({}) file", hexdump_filename);
        return;
    }

    let hex_bytes = Decoder::from_log(hexdump_str.unwrap().as_str());
    if hex_bytes.is_err() {
        println!("Failed to decode the hexdump ({}) file", hexdump_filename);
        return;
    }

    let bin_filename = &args[2];
    let f = File::create(bin_filename);
    if f.is_err() {
        println!("Failed to create the binary file ({})", bin_filename);
        return;
    }
    if f.unwrap().write_all(hex_bytes.unwrap().as_slice()).is_err() {
        println!("Failed to write the binary file ({})", bin_filename);
        return;
    }
}
