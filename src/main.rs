use std::io;

mod arp;
mod ip;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::without_packet_info("tun0",tun_tap::Mode::Tap)?;
    let mut buf = [0u8; 1500];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let ether_slice = etherparse::Ethernet2Slice::from_slice_without_fcs(&buf).expect("FAILED TO READ ETHERNET PACKAGE");
        match ether_slice.ether_type() {
            etherparse::EtherType::ARP => {
                let arp_packet = arp::Arp::from_slice(ether_slice.payload().payload).expect("failed to create arp");
                let arp_resp = arp_packet.response_as_bytes(&[0x02u8, 0x02u8, 0x02u8, 0x02u8, 0x02u8, 0x02u8,] as &[u8; 6]);
                let ether_resp = etherparse::Ethernet2Header{
                    source: ether_slice.destination(),
                    destination: ether_slice.source(),
                    ether_type: ether_slice.ether_type(),
                };
                let mut resp = Vec::with_capacity(ether_resp.to_bytes().len() + arp_resp.len());
                resp.extend_from_slice(&ether_resp.to_bytes());
                resp.extend_from_slice(&arp_resp);
                let sent_bytes = nic.send(resp.as_slice()).expect("failed to send packet");
                eprintln!("Sent {} bytes", sent_bytes);
            }
            etherparse::EtherType::IPV6 => {continue;} // ignore IPV6
            etherparse::EtherType::IPV4 => {
                match ip::Ip::on_packet(ether_slice.payload_slice()) {
                    Ok(res) => {

                        let ether_resp = etherparse::Ethernet2Header{
                            source: ether_slice.destination(),
                            destination: ether_slice.source(),
                            ether_type: ether_slice.ether_type(),
                        };

                        let mut resp = Vec::with_capacity(ether_resp.to_bytes().len() + res.len());
                        resp.extend_from_slice(&ether_resp.to_bytes());
                        resp.extend_from_slice(res.as_slice());
                        let sent_bytes = nic.send(resp.as_slice()).expect("failed to send packet");
                        eprintln!("sent ping response {}: {:02x?}", sent_bytes, res);
                    }
                    Err(e) => {
                        eprintln!("ipv4 err: {:?}", e);
                    }
                } 

            } 
            _ => {
                eprintln!(
                    "Read {} bytes\nSource: {:02x?} Destination: {:02x?} Type: {:?}",
                    nbytes, ether_slice.source(), ether_slice.destination(), ether_slice.ether_type());
                eprintln!("Unknown ether type: {:?}", ether_slice.ether_type());
            }
        }
    }

    // Ok(())
}
