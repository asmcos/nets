//
//  packet sniffer
//
mod lib;

use crate::lib::parse;
use libpcap;
use std::slice;

fn main (){

    println!("{}","Welcome to use nets");

	let dev = libpcap::lookup();	

    println!("{}",dev);

    let mut Packet = libpcap::open(dev.as_str());


    while let data = libpcap::next_ex(&mut Packet){
        println!("Packet Length {:?}",Packet.head.len);

        let parse = parse::PacketParse::new();
        let data = unsafe { slice::from_raw_parts(Packet.data, Packet.head.len.try_into().unwrap()) };
        let eth = parse.parse_link_layer(data);
        println!("{:?}",eth);
    }

    libpcap::close(&mut Packet); 
	
}
