use crate::pcap::libpcap;

pub fn lookup() {

	let mut errbuf = [0i8; 256];
	unsafe {
		let devices = libpcap::pcap_lookupdev(errbuf.as_mut_ptr());
		println!("{:?}",*devices);
	}
}
