#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(warnings, unused)]
//
//  packet sniffer
//
mod lib;

use crate::lib::ethparse;
use crate::lib::httparse;
use crate::lib::stream::Stream;
use libpcap;
use std::slice;

fn main (){

    println!("{}","Welcome to use nets");

	let dev = libpcap::lookup();	

    println!("{}",dev);

    let mut Packet = libpcap::open(dev.as_str());

    libpcap::setfilter(&mut Packet,"tcp port 80");
    while let data = libpcap::next_ex(&mut Packet){
        if (data == 0){
            continue;
        }
        println!("Packet Length {:?}",Packet.head.len);
        
        let s = unsafe{
            Stream::from_raw(Packet.data,Packet.head.len as usize)
        };

        println!("{:?}",s.buf);

        
        let parse = ethparse::PacketParse::new();
        let eth = parse.parse_link_layer(&s.buf.as_slice());
        //println!("{:?}",eth);
        match eth{
            Ok(p)=>{
                let tcp = p.headers.get(0);
                match tcp{ 
                    Some(ethparse::PacketHeader::Tcp(packet))=>{
                        //println!("{:?}",packet);
                        if (p.payload.len() > 10){
                            httparse::isRequest(&p.payload);
                        }
                            
                    }
                    _ => {}
                }
            
            }
            _ => {}
        }
        
    }

    libpcap::close(&mut Packet); 
	
}
