use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Id(pub u16);
impl Id {
    pub fn as_bytes(&self) -> [u8; size_of::<Id>()] {
        let bytes = self.0.to_be_bytes();
        [bytes[0], bytes[1]]
    }
}
