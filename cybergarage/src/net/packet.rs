// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Packet represents a transport packet.
pub struct Packet {
    data: Vec<u8>,
    from: SocketAddr,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            data: Vec::new(),
            from: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Packet {
        Packet {
            data: data.clone(),
            from: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        }
    }

    pub fn set_from(&mut self, addr: SocketAddr) -> &mut Self {
        self.from = addr;
        self
    }

    pub fn from(&self) -> SocketAddr {
        self.from
    }

    pub fn set_bytes(&mut self, data: Vec<u8>) {
        self.data = data.clone();
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.data
    }
}

impl Clone for Packet {
    fn clone(&self) -> Packet {
        Packet {
            data: self.data.clone(),
            from: self.from,
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.bytes() {
            let res = f.write_fmt(format_args!("{:02X}", b));
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }
}
