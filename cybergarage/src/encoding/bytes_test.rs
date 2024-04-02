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

    use crate::encoding::bytes::*;

    #[test]
    fn bytes_u32_from() {
        let mut buf: [u8; 1] = [0; 1];
        for n in 0..=0xFF {
            Bytes::from_u32(n, &mut buf);
            assert_eq!(n, Bytes::to_u32(&buf));
        }

        let mut buf2: [u8; 2] = [0; 2];
        for n in (0..=0xFFFF).step_by(0xFFFF / 0xFF) {
            Bytes::from_u32(n, &mut buf2);
            assert_eq!(n, Bytes::to_u32(&buf2));
        }

        let mut buf: [u8; 3] = [0; 3];
        for n in (0..=0xFFFFFF).step_by(0xFFFFFF / 0xFF) {
            Bytes::from_u32(n, &mut buf);
            assert_eq!(n, Bytes::to_u32(&buf));
        }

        let mut buf: [u8; 4] = [0; 4];
        for n in (0..=0xFFFFFFFFu32).step_by(0xFFFFFFFF / 0xFF) {
            Bytes::from_u32(n, &mut buf);
            assert_eq!(n, Bytes::to_u32(&buf));
        }
    }

    #[test]
    fn bytes_hex_from() {
        for n in 0..=0xFF {
            let hex_str = format!("{:02X}", n);
            let hex_bytes = Bytes::from_hexstr(&hex_str).unwrap();
            let to_hex_str = Bytes::to_hexstring(&hex_bytes);
            assert_eq!(hex_str, to_hex_str);
        }

        for n in (0..=0xFFFF).step_by(0xFFFF / 0xFF) {
            let hex_str = format!("{:04X}", n);
            let hex_bytes = Bytes::from_hexstr(&hex_str).unwrap();
            let to_hex_str = Bytes::to_hexstring(&hex_bytes);
            assert_eq!(hex_str, to_hex_str);
        }

        for n in (0..=0xFFFFFF).step_by(0xFFFFFF / 0xFF) {
            let hex_str = format!("{:06X}", n);
            let hex_bytes = Bytes::from_hexstr(&hex_str).unwrap();
            let to_hex_str = Bytes::to_hexstring(&hex_bytes);
            assert_eq!(hex_str, to_hex_str);
        }

        for n in (0..=0xFFFFFFFFu32).step_by(0xFFFFFFFF / 0xFF) {
            let hex_str = format!("{:08X}", n);
            let hex_bytes = Bytes::from_hexstr(&hex_str).unwrap();
            let to_hex_str = Bytes::to_hexstring(&hex_bytes);
            assert_eq!(hex_str, to_hex_str);
        }
    }
}
