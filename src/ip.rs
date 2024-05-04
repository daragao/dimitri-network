use std::io;

pub struct Ip { }

impl<'a> Ip {
    pub fn on_packet(ether_payload: &'a[u8]) -> Result<Vec<u8>, io::Error> {
        let ipv4_slice = etherparse::Ipv4Slice::from_slice(ether_payload)
            .expect("failed to parse ipv4 header");
        let ipv4_header = ipv4_slice.header();
        let payload = ipv4_slice.payload().payload;
        match ipv4_header.protocol() {
            etherparse::IpNumber::ICMP => {
                let (icmp, payload) = etherparse::Icmpv4Header::from_slice(payload).expect("failed to parse the ping");
                eprintln!("got {:?} -> {:?} ping: {:?}", ipv4_header.source(), ipv4_header.destination(), icmp,);
                match icmp.icmp_type {
                    etherparse::Icmpv4Type::EchoRequest(echo_header) => {
                        // eprintln!("id: {:?} seq: {:?}", echo_header.id, echo_header.seq);

                        let builder = etherparse::PacketBuilder::
                            ipv4(ipv4_header.destination(),  //source ip
                                ipv4_header.source(), //destination ip
                                20)            //time to life
                            .icmpv4_echo_reply(
                                echo_header.id, // identifier
                                echo_header.seq, // sequence number
                            );
                        let mut result = vec![0; builder.size(payload.len())];
                        builder.write(&mut result.as_mut_slice(), payload)
                            .expect("failed to write ping response");
                        return Ok(result);
                    }
                    _ => {}
                }
            }
            _ => {
                eprintln!(
                    "Unknonwn IPv4 type: {:?} {:?}", ipv4_header.protocol(), ipv4_header
                );
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, ""))
        //eprintln!("Unknown ether type: {:?}", ether_slice.ether_type());
    }
}
