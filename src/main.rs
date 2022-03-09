//
//  packet sniffer
//
mod lib;
mod pcap;

use crate::lib::parse;
use crate::pcap::Capture;

fn main (){

    println!("{}","Welcome to use nets");

	Capture::lookup();	




}
