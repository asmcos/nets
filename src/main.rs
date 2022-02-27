//
//  packet sniffer
//
mod lib;

use pcap;

use crate::lib::parse;


fn main (){
    let mut cap ;

    println!("{}","Welcome to use nets");
    //cap = init_pcap();

    cap = pcap::Capture::from_device("any")
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    // filter out all packets that don't have 127.0.0.1 as a source or destination.
    cap.filter("tcp and port 80", true).unwrap();




    while let Ok(packet) = cap.next() {
        //println!("got packet! {:?}", packet);
        let ts: String = format!(
                "{}.{:06}",
                &packet.header.ts.tv_sec, &packet.header.ts.tv_usec
            );

        let n_packet = parse::PacketParse::new();
        let ret = n_packet.parse_packet(packet.data.to_vec(),packet.header.len,ts);
        println!("{:?}",ret);

    }

}
