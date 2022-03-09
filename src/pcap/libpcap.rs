#![allow(non_camel_case_types)]

use libc::{c_char, c_int, c_uchar, c_uint, c_ushort, sockaddr, timeval, FILE};


extern "C" {
    pub fn pcap_lookupdev(arg1: *mut c_char) -> *mut c_char;
}
