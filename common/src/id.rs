#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Id(pub u16);
impl Id {
    pub fn as_bytes(&self) -> [u8; size_of::<Id>()] {
        let bytes = self.0.to_be_bytes();
        [bytes[0], bytes[1]]
    }
}
