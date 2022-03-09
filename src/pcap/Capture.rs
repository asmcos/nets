use crate::pcap::libpcap;
use std::ffi::{self, CStr, CString};


use crate::Capture::Error::*;


#[derive(Debug, PartialEq)]
pub enum Error {
    /// The underlying library returned invalid UTF-8
    formedError(std::str::Utf8Error),
}

impl From<std::str::Utf8Error> for Error {
    fn from(obj: std::str::Utf8Error) -> Error {
        formedError(obj)
    }
}

#[inline]
unsafe fn cstr_to_string(ptr: *const libc::c_char) -> Result<Option<String>,Error> {
    let string = if ptr.is_null() {
        None
    } else {
        Some(CStr::from_ptr(ptr as _).to_str()?.to_owned())
    };
    Ok(string)
}


pub fn lookup() {

	let mut errbuf = [0i8; 256];
	unsafe {
		let devices = libpcap::pcap_lookupdev(errbuf.as_mut_ptr());
		println!("{:?}",cstr_to_string(devices));
	}
}
