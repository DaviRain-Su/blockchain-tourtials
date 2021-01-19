// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::coding::decode_fixed_32;

pub fn hash(data: &[u8], seed: u32) -> u32 {
    // Similar to murmur hash
    let n = data.len();
    let m: u32 = 0xc6a4a793;
    let r = 24;
    let mut h = seed ^ (m.wrapping_mul(n as u32));

    // Pick up four bytes at a time
    let mut i = 0;
    while i + 4 <= n {
        let w = decode_fixed_32(&data[i..]);
        i += 4;
        h += w;
        h = h.wrapping_mul(m);
        h ^= h >> 16;
    }

    let diff = n - i;
    if diff >= 3 {
        h += (data[i + 2] as u32) << 16
    };
    if diff >= 2 {
        h += (data[i + 1] as u32) << 8
    };
    if diff >= 1 {
        h += data[i] as u32;
        h = h.wrapping_mul(m);
        h ^= h >> r;
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_unsigned_issue() {
        let data1 = [0x62];
        let data2 = [0xc3, 0x97];
        let data3 = [0xe2, 0x99, 0xa5];
        let data4 = [0xe1, 0x80, 0xb9, 0x32];
        let data5 = [
            0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
            0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(hash(&[], 0xbc9f1d34), 0xbc9f1d34);
        assert_eq!(hash(&data1, 0xbc9f1d34), 0xef1345c4);
        assert_eq!(hash(&data2, 0xbc9f1d34), 0x5b663814);
        assert_eq!(hash(&data3, 0xbc9f1d34), 0x323c078f);
        assert_eq!(hash(&data4, 0xbc9f1d34), 0xed21633a);
        assert_eq!(hash(&data5, 0x12345678), 0xf333dabb);
    }
}
