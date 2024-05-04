use etherparse::err;

///
/// Packet format:
/// --------------
/// 
/// To communicate mappings from <protocol, address> pairs to 48.bit
/// Ethernet addresses, a packet format that embodies the Address
/// Resolution protocol is needed.  The format of the packet follows.
/// 
///     Ethernet transmission layer (not necessarily accessible to
///          the user):
///         48.bit: Ethernet address of destination
///         48.bit: Ethernet address of sender
///         16.bit: Protocol type = ether_type$ADDRESS_RESOLUTION
///     Ethernet packet data:
///         16.bit: (ar$hrd) Hardware address space (e.g., Ethernet,
///                          Packet Radio Net.)
///         16.bit: (ar$pro) Protocol address space.  For Ethernet
///                          hardware, this is from the set of type
///                          fields ether_typ$<protocol>.
///          8.bit: (ar$hln) byte length of each hardware address
///          8.bit: (ar$pln) byte length of each protocol address
///         16.bit: (ar$op)  opcode (ares_op$REQUEST | ares_op$REPLY)
///         nbytes: (ar$sha) Hardware address of sender of this
///                          packet, n from the ar$hln field.
///         mbytes: (ar$spa) Protocol address of sender of this
///                          packet, m from the ar$pln field.
///         nbytes: (ar$tha) Hardware address of target of this
///                          packet (if known).
///         mbytes: (ar$tpa) Protocol address of target.
///
pub struct Arp<'a> {
    hw_addr_space: u16,
    proto_addr_space: u16,
    len_hw_addr: u8,
    len_proto_addr: u8,
    opcode: u16,
    hw_addr_sender: &'a [u8],
    proto_addr_sender: &'a [u8],
    hw_addr_receiver: &'a [u8],
    proto_addr_receiver: &'a [u8],
}

impl<'a> Arp<'a> {
    pub fn from_slice(slice: & [u8]) -> Result<Arp, err::LenError> {

        let hw_addr_space = u16::from_be_bytes([slice[0], slice[1]]);
        let proto_addr_space = u16::from_be_bytes([slice[2], slice[3]]);
        let len_hw_addr = slice[4];
        let len_proto_addr = slice[5];
        let opcode = u16::from_be_bytes([slice[6], slice[7]]);
        let mut addr_idx = 8;
        let hw_addr_sender = &slice[addr_idx..addr_idx+usize::from(len_hw_addr)];
        addr_idx = addr_idx+usize::from(len_hw_addr);
        let proto_addr_sender = &slice[addr_idx..addr_idx+usize::from(len_proto_addr)];
        addr_idx = addr_idx+usize::from(len_proto_addr);
        let hw_addr_receiver = &slice[addr_idx..addr_idx+usize::from(len_hw_addr)];
        addr_idx = addr_idx+usize::from(len_hw_addr);
        let proto_addr_receiver = &slice[addr_idx..addr_idx+usize::from(len_proto_addr)];

        Ok(Arp{
            hw_addr_space,
            proto_addr_space,
            len_hw_addr,
            len_proto_addr,
            opcode,
            hw_addr_sender,
            proto_addr_sender,
            hw_addr_receiver,
            proto_addr_receiver,
        }) 
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = vec![
            self.hw_addr_space.to_be_bytes(),
            self.proto_addr_space.to_be_bytes(),
            [self.len_hw_addr, self.len_proto_addr],
            self.opcode.to_be_bytes(),
        ].concat();

        res.extend_from_slice(self.hw_addr_sender);
        res.extend_from_slice(self.proto_addr_sender);
        res.extend_from_slice(self.hw_addr_receiver);
        res.extend_from_slice(self.proto_addr_receiver);

        return res;
    }

    pub fn response_as_bytes(&self, hw_addr: &[u8]) -> Vec<u8> {
        let mut res = vec![
            self.hw_addr_space.to_be_bytes(),
            self.proto_addr_space.to_be_bytes(),
            [self.len_hw_addr, self.len_proto_addr],
            [0x00, 0x02],
        ].concat();

        res.extend_from_slice(hw_addr);
        res.extend_from_slice(self.proto_addr_receiver);
        res.extend_from_slice(self.hw_addr_sender);
        res.extend_from_slice(self.proto_addr_sender);

        return res;
    }

}
