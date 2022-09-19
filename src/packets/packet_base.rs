use crate::packet_tag::PacketTag;

pub trait PacketBase {
    fn get_tag(&self) -> PacketTag;
}
