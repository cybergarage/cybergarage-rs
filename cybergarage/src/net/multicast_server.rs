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

#![allow(dead_code)]

use log::*;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::net::default::*;
use crate::net::notifier::*;
use crate::net::observer::ObserverObject;
use crate::net::packet::Packet;
use crate::net::result::Result;
use crate::net::udp_socket::UdpSocket;

pub struct MulticastServer {
    socket: Arc<RwLock<UdpSocket>>,
    notifier: Notifier,
    maddr: IpAddr,
    port: u16,
}

impl MulticastServer {
    pub fn new() -> MulticastServer {
        MulticastServer {
            socket: Arc::new(RwLock::new(UdpSocket::new())),
            notifier: notifier_new(),
            maddr: std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 0,
        }
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        self.notifier.lock().unwrap().add_observer(observer)
    }

    pub fn notify(&self, msg: &Packet) -> Result<usize> {
        let to_addr_str = format!("{}:{}", self.maddr, self.port);
        let to_addr: SocketAddr = to_addr_str.parse().unwrap();
        let msg_bytes = msg.bytes();
        let addr = to_addr.ip();
        let port = to_addr.port();
        info!(
            "MCST {} -> {}:{} ({})",
            self.socket.read().unwrap().addr().unwrap(),
            addr,
            port,
            msg,
        );
        let ret = self.socket.read().unwrap().send_to(&msg_bytes, to_addr);
        if ret.is_err() {
            warn!("Couldn't notify Packet to {} {}", addr, port);
        }
        ret
    }

    pub fn ifaddr(&self) -> io::Result<SocketAddr> {
        self.socket.read().unwrap().addr()
    }

    pub fn is_bound(&self) -> bool {
        self.socket.read().unwrap().addr().is_ok()
    }

    pub fn bind(&mut self, maddr: IpAddr, port: u16, ifaddr: IpAddr) -> Result<()> {
        let mut addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
        if ifaddr.is_ipv6() {
            addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), port);
        }
        debug!("BIND MCT {}", addr);
        let ret = self.socket.write().unwrap().bind(addr);
        if ret.is_err() {
            return ret;
        }
        match ifaddr {
            IpAddr::V4(ifaddr_v4) => match maddr {
                IpAddr::V4(maddr_v4) => {
                    let ret = self
                        .socket
                        .write()
                        .unwrap()
                        .join_multicast_v4(&maddr_v4, &ifaddr_v4);
                    if ret.is_err() {
                        self.close();
                        return ret;
                    }
                    debug!("BIND MCT {}:{} -> {}:{}", ifaddr, port, maddr_v4, ifaddr_v4);
                }
                IpAddr::V6(maddr_v6) => {
                    error!("BIND MCT {}:{} -> {}:{}", ifaddr, port, maddr_v6, ifaddr_v4);
                }
            },
            IpAddr::V6(ifaddr_v6) => match maddr {
                IpAddr::V4(maddr_v4) => {
                    error!("BIND MCT {}:{} -> {}:{}", ifaddr, port, maddr_v4, ifaddr_v6);
                }
                IpAddr::V6(maddr_v6) => {
                    let ret = self
                        .socket
                        .write()
                        .unwrap()
                        .join_multicast_v6(&maddr_v6, &ifaddr_v6);
                    if ret.is_err() {
                        self.close();
                        return ret;
                    }
                    debug!("BIND MCT {}:{} -> {}:{}", ifaddr, port, maddr_v6, ifaddr_v6);
                }
            },
        }
        self.maddr = maddr;
        self.port = port;
        Ok(())
    }

    pub fn close(&self) -> bool {
        self.socket.read().unwrap().close();
        true
    }

    pub fn start(&mut self) -> bool {
        let socket = self.socket.clone();
        let notifier = self.notifier.clone();
        thread::spawn(move || {
            let mut buf = [0 as u8; MAX_PACKET_SIZE];
            loop {
                let recv_res = socket.read().unwrap().recv_from(&mut buf);
                match &recv_res {
                    Ok((n_bytes, remote_addr)) => {
                        let recv_msg = &buf[0..*n_bytes];
                        let mut msg = Packet::from_bytes(&recv_msg.to_vec());
                        info!(
                            "RECV {} -> {} ({})",
                            remote_addr,
                            socket.read().unwrap().addr().ok().unwrap(),
                            msg
                        );
                        msg.set_from(remote_addr.clone());
                        notifier.lock().unwrap().notify(&msg);
                    }
                    Err(e) => {
                        warn!(
                            "RECV {} ({})",
                            socket.read().unwrap().addr().ok().unwrap(),
                            e
                        );
                        break;
                    }
                }
            }
        });
        true
    }

    pub fn stop(&self) -> bool {
        if !self.close() {
            return false;
        }
        true
    }
}

impl Drop for MulticastServer {
    fn drop(&mut self) {
        self.stop();
    }
}
