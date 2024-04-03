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
        let log = include_str!("log/hexdump01.log");
        let hexdump_bytes = Decoder::from_log(log);
        assert!(hexdump_bytes.is_ok());
        if hexdump_bytes.is_err() {
            return;
        }

        let expected_bytes = include_bytes!("log/hexdump01.bin");
        let hexdump_bytes = hexdump_bytes.unwrap();
        assert_eq!(expected_bytes.len(), hexdump_bytes.len());
        if expected_bytes.len() != hexdump_bytes.len() {
            return;
        }
        for n in 0..expected_bytes.len() {
            assert_eq!(expected_bytes[n], hexdump_bytes[n]);
        }
    }
}
