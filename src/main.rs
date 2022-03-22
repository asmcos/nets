#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(warnings, unused)]
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
        //println!("{:?}",eth);
        match eth{
            Ok(tcp)=>{
                for i in tcp.headers{
                    
                        println!("{:?}",i);
                    
                }
            }
            _ => {}
        }
        
    }

    libpcap::close(&mut Packet); 
	
}
