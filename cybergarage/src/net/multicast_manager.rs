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

use std::net::IpAddr;

use crate::net::interface::*;
use crate::net::multicast_server::MulticastServer;
use crate::net::observer::ObserverObject;
use crate::net::packet::Packet;
use crate::net::result::Result;

pub struct MulticastManager {
    mcast_servers: Vec<MulticastServer>,
}

impl MulticastManager {
    pub fn new() -> MulticastManager {
        MulticastManager {
            mcast_servers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        for mcast_server in self.mcast_servers.iter_mut() {
            if !mcast_server.add_observer(observer.clone()) {
                return false;
            }
        }
        true
    }

    pub fn notify(&self, msg: &Packet) -> bool {
        for mcast_server in self.mcast_servers.iter() {
            if !mcast_server.notify(msg).is_err() {
                return false;
            }
        }
        true
    }

    pub fn is_running(&self) -> bool {
        if self.mcast_servers.len() == 0 {
            return false;
        }
        true
    }

    pub fn has_interface(&self, addr: IpAddr) -> bool {
        for mcast_server in self.mcast_servers.iter() {
            if mcast_server.ifaddr().is_err() {
                continue;
            }
            if mcast_server.ifaddr().unwrap().ip() == addr {
                return true;
            }
        }
        false
    }

    pub fn start(&mut self, maddrs: &[IpAddr], port: u16) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        for ifaddr in get_all_interfaces() {
            let mut mcast_server = MulticastServer::new();
            if ifaddr.is_ipv4() {
                for maddr in maddrs {
                    if maddr.is_ipv4() {
                        let ret = mcast_server.bind(*maddr, port, ifaddr);
                        if ret.is_err() {
                            self.stop();
                            return ret;
                        }
                        break;
                    }
                }
            } else if ifaddr.is_ipv6() {
                for maddr in maddrs {
                    if maddr.is_ipv6() {
                        let ret = mcast_server.bind(*maddr, port, ifaddr);
                        if ret.is_err() {
                            self.stop();
                            return ret;
                        }
                        break;
                    }
                }
            } else {
                continue;
            }
            let ret = mcast_server.start();
            if ret.is_err() {
                self.stop();
                return ret;
            }
            self.mcast_servers.push(mcast_server);
        }
        Ok(())
    }

    pub fn stop(&mut self) -> bool {
        let mut is_all_server_stopped = true;
        for mcast_server in self.mcast_servers.iter_mut() {
            if !&mcast_server.stop() {
                is_all_server_stopped = false;
            }
        }
        self.mcast_servers.clear();
        is_all_server_stopped
    }
}

impl Drop for MulticastManager {
    fn drop(&mut self) {
        self.stop();
    }
}
