//
//  packet sniffer
//
mod lib;

use crate::lib::parse;
use libpcap;

fn main (){

    println!("{}","Welcome to use nets");

	let devs = libpcap::lookup();	

    println!("{}",devs);
	
}
