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

#[cfg(test)]
mod tests {
    use crate::log::hexdump::*;

    #[test]
    fn decode_from() {
        struct Test {
            log: &'static str,
            expected_bytes: &'static [u8],
        }

        let tests = vec![Test {
            log: include_str!("log/hexdump01.log"),
            expected_bytes: include_bytes!("log/hexdump01.bin"),
        }];

        for test in tests {
            let hexdump_bytes = Decoder::from_log(test.log);
            assert!(hexdump_bytes.is_ok());
            if hexdump_bytes.is_err() {
                return;
            }

            let hexdump_bytes = hexdump_bytes.unwrap();
            assert_eq!(test.expected_bytes.len(), hexdump_bytes.len());
            if test.expected_bytes.len() != hexdump_bytes.len() {
                return;
            }
            for n in 0..test.expected_bytes.len() {
                assert_eq!(test.expected_bytes[n], hexdump_bytes[n]);
            }
        }
    }
}
