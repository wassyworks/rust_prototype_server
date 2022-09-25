#[allow(dead_code)]
#[derive(Eq, Hash, Clone, Copy, PartialEq, Debug)]
pub enum PacketTag {
    SimpleEntity,
    SimpleEntityList,
    InitValueTest = 10,
    IncrementTest,
}
