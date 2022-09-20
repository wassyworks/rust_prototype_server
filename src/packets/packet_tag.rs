#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Debug)]
pub enum PacketTag {
    SimpleEntity,
    SimpleEntityList,
    InitValueTest = 10,
    IncrementTest,
}
