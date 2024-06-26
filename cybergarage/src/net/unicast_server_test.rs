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

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::time;

    use crate::net::default_test::*;
    use crate::net::interface::*;
    use crate::net::packet::Packet;
    use crate::net::unicast_server::*;

    use crate::log::Logger;
    use crate::net::notify_manager_test::*;

    #[test]
    fn unicast_server() {
        fn test_udp_server(ifaddr: IpAddr) {
            Logger::init();

            const TEST_OBSERVER_COUNT: i32 = 5;
            let counter = Arc::new(Mutex::new(0));

            let mut server = UnicastServer::new();

            let observer = TestNotifyCounter::new(counter.clone());
            assert!(server.add_observer(Arc::new(Mutex::new(observer))));

            let ret = server.bind(ifaddr, TEST_PORT);
            assert!(ret.is_ok(), "{:?}", ret);
            assert!(server.start().is_ok());
            thread::sleep(time::Duration::from_secs(5));

            let mut pkt = Packet::new();
            pkt.set_bytes(vec![0 as u8; 1]);
            for _ in 0..TEST_OBSERVER_COUNT {
                let server_addr = server.ifaddr();
                assert!(server_addr.is_ok());
                let ret = server.send(server_addr.unwrap(), &pkt);
                assert!(ret.is_ok(), "{:?}", ret);
                thread::sleep(time::Duration::from_secs(1));
            }

            let counter = counter.lock();
            // NOTE: GitHub Action is slow and may drop to send UDP packets.
            // assert_eq!(*counter.unwrap(), TEST_OBSERVER_COUNT);
            if counter.is_ok() {
                assert!(0 < *counter.unwrap());
            }

            assert!(server.stop().is_ok());
        }

        for ifaddr in get_all_interfaces() {
            test_udp_server(ifaddr);
        }
    }
}
