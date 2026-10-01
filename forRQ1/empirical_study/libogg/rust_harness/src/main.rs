// ogg_encode — empirical-study harness for libogg (0_raw / c2rust_raw).
//
// EMPIRICAL STUDY: two operations dispatched by argv[1] so ONE binary covers
// libogg's ENCODE and DECODE hot paths (raises RQ1 coverage past encode-only 18%).
// Both self-contained (input ignored); C and Rust MUST print identical stdout.
//   default  → ENCODE: init a stream, feed M fixed packets, flush each into a page,
//              fold the page HEADER (carries libogg's CRC) into a checksum.
//              Exercises page framing + CRC. Prints "packets=M checksum=HEX".
//   "decode" → DECODE: build the encoded byte-buffer ONCE (M_DEC packets → pages),
//              then loop N_DEC times feeding it through ogg_sync_buffer/wrote +
//              ogg_sync_pageout + ogg_stream_pagein + ogg_stream_packetout, folding
//              each packet's (packetno,bytes). Exercises the sync/page-split/packet
//              reassembly path. Prints "decoded checksum=HEX".
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::c_long;
use std::mem::MaybeUninit;

use libogg_raw::src::framing::{
    ogg_packet, ogg_page, ogg_stream_state, ogg_sync_state,
    ogg_stream_clear, ogg_stream_flush, ogg_stream_init, ogg_stream_packetin,
    ogg_stream_packetout, ogg_stream_pagein,
    ogg_sync_buffer, ogg_sync_clear, ogg_sync_init, ogg_sync_pageout, ogg_sync_wrote,
};

const M: i64 = 300_000; // encode packets (wall-time ~1s)
const M_DEC: i64 = 30_000; // decode: packets encoded once into the buffer
const N_DEC: i64 = 10; // decode iterations over the buffer (~1.4s; decode is heavy)
const PLEN: usize = 1024; // packet payload bytes

unsafe fn fill_payload(payload: &mut [u8; PLEN]) {
    for i in 0..PLEN {
        payload[i] = (i.wrapping_mul(31).wrapping_add(7)) as u8;
    }
}

unsafe fn encode() {
    let mut os: ogg_stream_state = MaybeUninit::zeroed().assume_init();
    ogg_stream_init(&mut os, 0x1234);
    let mut payload = [0u8; PLEN];
    fill_payload(&mut payload);

    let mut checksum: u64 = 0;
    for n in 0..M {
        let mut op = ogg_packet {
            packet: payload.as_mut_ptr(),
            bytes: PLEN as c_long,
            b_o_s: (n == 0) as c_long,
            e_o_s: (n == M - 1) as c_long,
            granulepos: n,
            packetno: n,
        };
        ogg_stream_packetin(&mut os, &mut op);
        let mut og: ogg_page = MaybeUninit::zeroed().assume_init();
        while ogg_stream_flush(&mut os, &mut og) != 0 {
            // Fold ONLY the page header (carries libogg's CRC) — see note; body-fold
            // was ~40% harness runtime, diluting the framing/CRC signal.
            for k in 0..og.header_len as usize {
                checksum = checksum.rotate_left(1) ^ (*og.header.add(k) as u64);
            }
        }
    }
    ogg_stream_clear(&mut os);
    println!("packets={} checksum={:016x}", M, checksum);
}

// Encode M_DEC packets, capturing the full page bytes (header+body) into a buffer.
unsafe fn build_encoded_buffer() -> Vec<u8> {
    let mut os: ogg_stream_state = MaybeUninit::zeroed().assume_init();
    ogg_stream_init(&mut os, 0x1234);
    let mut payload = [0u8; PLEN];
    fill_payload(&mut payload);

    let mut buf: Vec<u8> = Vec::new();
    for n in 0..M_DEC {
        let mut op = ogg_packet {
            packet: payload.as_mut_ptr(),
            bytes: PLEN as c_long,
            b_o_s: (n == 0) as c_long,
            e_o_s: (n == M_DEC - 1) as c_long,
            granulepos: n,
            packetno: n,
        };
        ogg_stream_packetin(&mut os, &mut op);
        let mut og: ogg_page = MaybeUninit::zeroed().assume_init();
        while ogg_stream_flush(&mut os, &mut og) != 0 {
            buf.extend_from_slice(std::slice::from_raw_parts(og.header, og.header_len as usize));
            buf.extend_from_slice(std::slice::from_raw_parts(og.body, og.body_len as usize));
        }
    }
    ogg_stream_clear(&mut os);
    buf
}

unsafe fn decode() {
    let buf = build_encoded_buffer();
    let mut checksum: u64 = 0;
    for _ in 0..N_DEC {
        let mut oy: ogg_sync_state = MaybeUninit::zeroed().assume_init();
        ogg_sync_init(&mut oy);
        let mut os: ogg_stream_state = MaybeUninit::zeroed().assume_init();
        ogg_stream_init(&mut os, 0x1234);

        let dst = ogg_sync_buffer(&mut oy, buf.len() as c_long) as *mut u8;
        core::ptr::copy_nonoverlapping(buf.as_ptr(), dst, buf.len());
        ogg_sync_wrote(&mut oy, buf.len() as c_long);

        let mut og: ogg_page = MaybeUninit::zeroed().assume_init();
        while ogg_sync_pageout(&mut oy, &mut og) == 1 {
            ogg_stream_pagein(&mut os, &mut og);
            let mut op: ogg_packet = MaybeUninit::zeroed().assume_init();
            while ogg_stream_packetout(&mut os, &mut op) == 1 {
                checksum = checksum.rotate_left(1) ^ (op.packetno as u64) ^ (op.bytes as u64);
            }
        }
        ogg_sync_clear(&mut oy);
        ogg_stream_clear(&mut os);
    }
    println!("decoded checksum={:016x}", checksum);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");
    unsafe {
        if mode == "decode" {
            decode();
        } else {
            encode();
        }
    }
}
