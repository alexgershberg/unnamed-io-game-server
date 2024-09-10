use crate::player::Id;

#[derive(Copy, Clone, Debug)]
pub struct GetPlayer(pub Id);

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Command {
    GetPlayer(GetPlayer) = 0x1,
    GetAllPlayers = 0x2,
}
