Nets is a Rust language crate for accessing the packet sniffing capabilities of pcap .
It's use rust-pcap/pcap.

### Features:

* List Devices
* parse http request/response
* display http header information

### Depends:

* rust-pcap
* http
* Linux/MacOSX libpcap, Windows WinPcap

### License:

* "MIT OR Apache-2.0"

### Install

```bash
git clone https://github.com/asmcos/nets
cd nets
Cargo build
```


### Demo
```bash
Ok(ParsedPacket { len: 0, timestamp: "", headers: [Tcp(TcpHeader { source_port: 50683, dest_port: 443, sequence_no: 286770016, ack_no: 0, data_offset: 11, reserved: 0, flag_urg: false, flag_ack: false, flag_psh: false, flag_rst: false, flag_syn: true, flag_fin: false, window: 65535, checksum: 14832, urgent_pointer: 0, options: None }), Ipv4(IPv4Header { version: 4, ihl: 20, tos: 0, length: 64, id: 0, flags: 2, fragment_offset: 0, ttl: 64, protocol: TCP, chksum: 11203, source_addr: 192.168.1.5, dest_addr: 12.27.16.10 }), Ether(EthernetFrame { source_mac: MacAddress([0, 116, 111, 112, 113, 122]), dest_mac: MacAddress([20, 113, 18, 15, 0, 10]), ethertype: IPv4 })], remaining: [] })
```

* http parse 
```bash
Ok(Complete(330)),Request { method: Some("GET"), path: Some("/js/polyfill.min.js?features=es6"), version: Some(1), headers: [Header { name: "Host", value: "rustai.cn" }, Header { name: "Connection", value: "keep-alive" }, Header { name: "User-Agent", value: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36" }, Header { name: "Accept", value: "*/*" }, Header { name: "Referer", value: "http://rustai.cn/" }, Header { name: "Accept-Encoding", value: "gzip, deflate" }, Header { name: "Accept-Language", value: "zh-CN,zh;q=0.9" }] }
```
